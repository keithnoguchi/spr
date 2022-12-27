//! simple block_on executor
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

use futures_lite::pin;
use std::future::Future;
use std::task::{Context, Poll};
use tracing::{instrument, trace};
use waker_fn::waker_fn;

/// simple block_on with parking, waker-fn and futures-lite.
#[instrument(skip(future))]
pub fn block_on<F: Future>(future: F) -> F::Output {
    let (parker, unparker) = parking::pair();
    let waker = waker_fn(move || {
        unparker.unpark();
    });
    let mut context = Context::from_waker(&waker);

    pin!(future);

    trace!("start the executor");
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Pending => {
                trace!("future is pending, let's park now");
                parker.park();
                trace!("waked up by the waker");
            }
            Poll::Ready(value) => {
                trace!("future is ready");
                return value;
            }
        }
    }
}
