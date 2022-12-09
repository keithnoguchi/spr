//! Drop trait example
//!
//! ```
//! $ RUST_LOG=trace cargo r -q
//! 2022-12-09T15:16:01.020624Z TRACE drop: in the main
//! 2022-12-09T15:16:01.020670Z TRACE drop: p uninitialized
//! 2022-12-09T15:16:01.020686Z TRACE drop: in the block
//! 2022-12-09T15:16:01.020700Z TRACE drop: q initialized q=A { b: B } q.b=B
//! 2022-12-09T15:16:01.020719Z TRACE drop: out of block
//! 2022-12-09T15:16:01.020786Z TRACE A::drop{self=A { b: B }}: drop: dropping
//! 2022-12-09T15:16:01.020841Z TRACE B::drop{self=B}: drop: dropping
//! 2022-12-09T15:16:01.020874Z TRACE drop: out of main
//! ```
//!
//! ```
//! $ RUST_LOG=trace c r 1
//! 2022-12-09T15:17:29.105789Z TRACE drop: in the main
//! 2022-12-09T15:17:29.105829Z TRACE drop: p uninitialized
//! 2022-12-09T15:17:29.105859Z TRACE drop: in the block
//! 2022-12-09T15:17:29.105875Z TRACE drop: q initialized q=A { b: B } q.b=B
//! 2022-12-09T15:17:29.105895Z TRACE drop: q uninitialized
//! 2022-12-09T15:17:29.105909Z TRACE drop: p initialized p=A { b: B } p.b=B
//! 2022-12-09T15:17:29.105929Z TRACE drop: out of block
//! 2022-12-09T15:17:29.105941Z TRACE drop: out of main
//! 2022-12-09T15:17:29.106013Z TRACE A::drop{self=A { b: B }}: drop: dropping
//! 2022-12-09T15:17:29.106071Z TRACE B::drop{self=B}: drop: dropping
//! ```
use tracing::{instrument, trace};

#[derive(Debug)]
struct A {
    b: B,
}

impl Drop for A {
    #[instrument(name = "A::drop")]
    fn drop(&mut self) {
        trace!("dropping");
    }
}

#[derive(Debug)]
struct B {}

impl Drop for B {
    #[instrument(name = "B::drop")]
    fn drop(&mut self) {
        trace!("dropping");
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    trace!("in the main");

    let condition = std::env::args().len() > 1;
    let p;
    trace!(/* ?p, */ "p uninitialized");
    {
        trace!("in the block");
        let q = A { b: B {} };
        trace!(?q, ?q.b, "q initialized");
        if condition {
            p = q;
            trace!(/* ?q, */ "q uninitialized");
            trace!(?p, ?p.b, "p initialized");
        }
        trace!("out of block");
    }
    trace!("out of main");
}
