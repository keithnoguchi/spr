//! std::sync::Arc from Scratch
//!
//! As in the [rustnomicon].
//!
//! [rustnomicon]: https://doc.rust-lang.org/nomicon/arc-mutex/
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use tracing::{instrument, trace};

pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
}

pub struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Drop for Arc<T> {
    #[instrument(name = "Arc::drop", skip(self))]
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Release) != 1 {
            return;
        }
        atomic::fence(Acquire);
        unsafe { Box::from_raw(self.ptr.as_ptr()) };
        trace!("dropped");
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        let old_rc = inner.rc.fetch_add(1, Relaxed);
        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });
        Self {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Arc;
    use std::sync::atomic::Ordering::Relaxed;

    #[test]
    fn arc_drop() {
        let data = String::from("This is a test");
        let arc = Arc::new(data.clone());
        let arc2 = arc.clone();
        drop(arc);
        drop(arc2);
    }

    #[test]
    fn arc_clone() {
        let data = String::from("This is a test");
        let arc = Arc::new(data.clone());
        let arc2 = arc.clone();
        assert_eq!(*arc, data);
        assert_eq!(*arc2, data);
        let inner = unsafe { arc.ptr.as_ref() };
        assert_eq!(inner.rc.load(Relaxed), 2);
    }

    #[test]
    fn arc_deref() {
        let data = "This is a test";
        let arc = Arc::new(data);
        assert_eq!(*arc, data);
    }

    #[test]
    fn arc_new() {
        let data = 900u32;
        let arc = Arc::new(data);
        let rc = unsafe { arc.ptr.as_ref().rc.load(Relaxed) };
        assert_eq!(rc, 1);
    }
}
