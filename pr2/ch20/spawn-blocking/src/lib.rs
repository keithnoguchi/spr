use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use tracing::{instrument, trace};

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

impl<T> Future for SpawnBlocking<T>
where
    T: Send + 'static,
{
    type Output = T;

    #[instrument(skip(self))]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut guard = self.0.lock().unwrap();

        // it's ready.
        if let Some(value) = guard.value.take() {
            trace!("future is ready");
            return Poll::Ready(value);
        }

        // register the new waker to be called.
        trace!("future is not ready");
        guard.waker = Some(cx.waker().clone());
        trace!("waker is set");

        Poll::Pending
    }
}

#[instrument(skip(closure))]
pub fn spawn_blocking<F, T>(closure: F) -> SpawnBlocking<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));

    thread::spawn({
        let inner = inner.clone();

        move || {
            let value = closure();
            trace!("closure is complete");

            let maybe_waker = {
                let mut guard = inner.lock().unwrap();

                guard.value = Some(value);
                guard.waker.take()
            };

            if let Some(waker) = maybe_waker {
                waker.wake();
                trace!("waker is called");
            }
        }
    });

    SpawnBlocking(inner)
}
