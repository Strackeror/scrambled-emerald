use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::future::Future;
use core::ops::Deref;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub(crate) fn dummy_raw_waker() -> RawWaker {
    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(|_| dummy_raw_waker(), |_| {}, |_| {}, |_| {});
    RawWaker::new(core::ptr::null(), &VTABLE)
}

pub(crate) fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

pub(crate) struct FuturePoll {
    pub(crate) future: Option<Pin<Box<dyn Future<Output = ()>>>>,
}

impl FuturePoll {
    fn new(obj: impl Future<Output = ()> + 'static) -> FuturePoll {
        FuturePoll { future: Some(Box::pin(obj)) }
    }

    fn poll(&mut self) -> Option<Done> {
        let Some(future) = self.future.as_mut() else {
            return Some(Done);
        };

        let waker = dummy_waker();
        let mut context = Context::from_waker(&waker);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(()) => {
                self.future = None;
                return Some(Done);
            }
            Poll::Pending => {}
        }

        None
    }
}

pub(crate) struct Done;

// Workarounds weeee
#[repr(transparent)]
pub struct UnsafeSyncCell<T: ?Sized>(UnsafeCell<T>);
unsafe impl<T: ?Sized> Sync for UnsafeSyncCell<T> {}
impl<T> UnsafeSyncCell<T> {
    const fn new(arg: T) -> Self {
        UnsafeSyncCell(UnsafeCell::new(arg))
    }
}
impl<T> Deref for UnsafeSyncCell<T> {
    type Target = UnsafeCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct Executor(UnsafeSyncCell<FuturePoll>);

impl Executor {
    pub const fn new() -> Executor {
        Executor(UnsafeSyncCell(UnsafeCell::new(FuturePoll { future: None })))
    }

    pub fn set<T: Future<Output = ()> + 'static>(&self, future: T) {
        unsafe { (*self.0.get()).future = Some(Box::pin(future)) }
    }

    pub fn poll(&self) -> Option<Done> {
        unsafe { (*self.0.get()).poll() }
    }
}
