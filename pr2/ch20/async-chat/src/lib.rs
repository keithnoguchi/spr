use eyre::eyre;
use futures_lite::io::{self, AsyncBufReadExt, AsyncWriteExt};
use futures_lite::stream::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

use async_broadcast::{broadcast, Receiver, RecvError, Sender};
use async_net::{AsyncToSocketAddrs, TcpListener, TcpStream};
use async_std::task;

#[instrument(skip(addr))]
pub async fn server(addr: impl AsyncToSocketAddrs) -> io::Result<()> {
    let group_table = Arc::new(GroupTable::new());
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();

    while let Some(mut socket) = incoming.try_next().await? {
        let peer = socket.peer_addr()?;
        let groups = group_table.clone();
        task::spawn(async move {
            if let Err(e) = reader(&mut socket, groups).await {
                eprintln!("{}: {e}", peer);
            } else {
                println!("{}: done", peer);
            }
        });
    }
    Ok(())
}

#[instrument(skip(s))]
async fn reader(s: &mut TcpStream, groups: Arc<GroupTable>) -> eyre::Result<()> {
    let tx = Arc::new(Outbound::new(s.clone()));
    let rx = io::BufReader::new(s);
    let mut stream = recv_as_json(rx);

    while let Some(result) = stream.next().await {
        let req = match result {
            Err(e) => return tx.send_err(e).await,
            Ok(req) => req,
        };
        match req {
            Request::Join { group_name } => groups.get_or_create(group_name).join(tx.clone()),
            Request::Post {
                group_name,
                message,
            } => match groups.get(&group_name) {
                Some(group) => {
                    if let Err(e) = group.post(message).await {
                        tx.send_err(e).await?;
                    }
                }
                None => {
                    let e = eyre!("wrong group: {group_name:?}");
                    tx.send_err(e).await?;
                }
            },
        }
    }
    Ok(())
}

#[instrument(skip(tx, rx))]
async fn writer(
    name: Arc<String>,
    tx: Arc<Outbound>,
    mut rx: Receiver<Arc<String>>,
) -> eyre::Result<()> {
    loop {
        let packet = match rx.recv().await {
            Ok(message) => Response::Message {
                group_name: name.clone(),
                message,
            },
            Err(RecvError::Closed) => return Err(eyre!("group closed")),
            Err(e) => Response::Error(e.to_string()),
        };
        tx.send(packet).await?;
    }
}

#[derive(Debug)]
struct GroupTable(std::sync::Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    fn new() -> Self {
        Self(std::sync::Mutex::new(HashMap::new()))
    }

    fn get(&self, name: &String) -> Option<Arc<Group>> {
        self.0.lock().unwrap().get(name).cloned()
    }

    fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Group::new(name)))
            .clone()
    }
}

#[derive(Debug)]
struct Group {
    name: Arc<String>,
    tx: Sender<Arc<String>>,
    _rx: Receiver<Arc<String>>,
}

impl Group {
    fn new(name: Arc<String>) -> Self {
        let (tx, _rx) = broadcast(8);
        Self { name, tx, _rx }
    }

    fn join(&self, tx: Arc<Outbound>) {
        let rx = self.tx.new_receiver();
        task::spawn(writer(self.name.clone(), tx, rx));
    }

    async fn post(&self, msg: Arc<String>) -> eyre::Result<()> {
        self.tx.broadcast(msg).await?;
        Ok(())
    }
}

#[derive(Debug)]
struct Outbound(async_lock::Mutex<TcpStream>);

impl Outbound {
    fn new(s: TcpStream) -> Self {
        Self(async_lock::Mutex::new(s))
    }

    async fn send(&self, packet: Response) -> eyre::Result<()> {
        let mut guard = self.0.lock().await;
        send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }

    async fn send_err(&self, err: eyre::Report) -> eyre::Result<()> {
        let resp = Response::Error(err.to_string());
        self.send(resp).await
    }
}

pub async fn send_as_json<S, P>(tx: &mut S, packet: &P) -> eyre::Result<()>
where
    S: AsyncWriteExt + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(packet)?;
    json.push('\n');
    tx.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn recv_as_json<S, P>(rx: S) -> impl Stream<Item = eyre::Result<P>>
where
    S: AsyncBufReadExt + Unpin,
    P: DeserializeOwned,
{
    rx.lines().map(|result| -> eyre::Result<P> {
        let line = result?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Request {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Response {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

impl TryFrom<String> for Request {
    type Error = eyre::Report;

    // simple line parser.
    fn try_from(req: String) -> eyre::Result<Self> {
        let req = req.trim_end();
        if req.starts_with('/') {
            match req.split_once(' ') {
                Some((command, rest)) => match command.to_lowercase().trim_end() {
                    "/join" => Ok(Self::Join {
                        group_name: Arc::new(rest.trim_start().to_string()),
                    }),
                    _ => Err(eyre!("unsupported command: {command:?}")),
                },
                None => Err(eyre!("invalid request: {req:?}")),
            }
        } else {
            match req.split_once(' ') {
                Some((group, message)) => Ok(Self::Post {
                    group_name: Arc::new(group.trim().to_string()),
                    message: Arc::new(message.trim().to_string()),
                }),
                None => Err(eyre!("invalid post message: {req:?}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Request;
    use std::sync::Arc;

    #[test]
    fn request_join_try_from_ok() {
        let requests = ["/join world", "/join   world"];
        let want = Request::Join {
            group_name: Arc::new("world".to_string()),
        };
        for request in requests {
            assert_eq!(Request::try_from(request).unwrap(), want);
        }
    }

    #[test]
    fn request_post_try_from_ok() {
        let requests = ["world: hello", "world:   hello", "  world:   hello   "];
        let want = Request::Post {
            group_name: Arc::new("world".to_string()),
            message: Arc::new("hello".to_string()),
        };
        for request in requests {
            assert_eq!(Request::try_from(request).unwrap(), want);
        }
    }
}
