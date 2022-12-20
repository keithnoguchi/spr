use std::io::{self, Result};
use std::net::{TcpListener, ToSocketAddrs};
use std::thread::spawn;

pub fn server<A>(addr: A) -> Result<()>
where
    A: ToSocketAddrs,
{
    let listener = TcpListener::bind(addr)?;
    loop {
        let (mut s, addr) = listener.accept()?;

        spawn(move || {
            println!("{:?}: connected", addr);
            let mut tx = s.try_clone()?;
            io::copy(&mut s, &mut tx)?;
            println!("{:?}: disconnected", addr);
            Ok::<_, io::Error>(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::server;

    #[test]
    fn test_server() {
        let addr = "127.0.0.1:64999";

        assert!(server(addr).is_ok());
    }
}
