use async_net::AsyncToSocketAddrs as ToSocketAddrs;
use async_net::{TcpListener, TcpStream};
use async_std::task;
use futures_lite::io::{self, Result};
use futures_lite::stream::StreamExt;

pub async fn run<A>(addr: A) -> Result<()>
where
    A: ToSocketAddrs,
{
    let l = TcpListener::bind(addr).await?;

    while let Some(socket) = l.incoming().try_next().await? {
        task::spawn(async move {
            if let Err(e) = serve(socket).await {
                eprintln!("{e}");
            }
        });
    }
    Ok(())
}

async fn serve(tx: TcpStream) -> Result<u64> {
    let rx = tx.clone();
    io::copy(tx, rx).await
}
