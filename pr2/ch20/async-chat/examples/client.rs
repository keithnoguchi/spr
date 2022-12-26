use futures_lite::future::{self, FutureExt};
use futures_lite::io::{AsyncBufReadExt, BufReader};
use futures_lite::stream::StreamExt;

use async_chat::{recv_as_json, send_as_json, Request, Response};
use async_net::AsyncToSocketAddrs as ToSocketAddrs;
use async_net::TcpStream;
use async_std::io::stdin;

const ADDR: &str = "localhost:8080";

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = future::block_on(client(addr)) {
        eprintln!("error: {e}");
    }
}

async fn client(addr: impl ToSocketAddrs) -> eyre::Result<()> {
    let s = TcpStream::connect(addr).await?;
    receiver(s.clone()).race(sender(s)).await?;
    Ok(())
}

async fn sender(mut tx: TcpStream) -> eyre::Result<()> {
    let reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    while let Some(line) = lines.try_next().await? {
        let req = Request::try_from(line)?;
        send_as_json(&mut tx, &req).await?;
    }
    Ok(())
}

async fn receiver(rx: TcpStream) -> eyre::Result<()> {
    let reader = BufReader::new(rx);
    let mut stream = recv_as_json(reader);

    while let Some(resp) = stream.try_next().await? {
        match resp {
            Response::Message {
                group_name,
                message,
            } => {
                println!("{group_name}: {message:?}");
            }
            Response::Error(e) => {
                println!("error: {e}");
            }
        }
    }
    Ok(())
}
