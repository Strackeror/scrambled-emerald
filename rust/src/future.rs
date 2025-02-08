use alloc::boxed::Box;
use core::cell::{RefCell, UnsafeCell};
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
    fn new(obj: Box<dyn Future<Output = ()>>) -> FuturePoll {
        FuturePoll {
            future: Some(Pin::from(obj)),
        }
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
pub struct RefCellSync<T: ?Sized>(RefCell<T>);
unsafe impl<T: ?Sized> Sync for RefCellSync<T> {}
impl<T> RefCellSync<T> {
    pub const fn new(arg: T) -> Self {
        RefCellSync(RefCell::new(arg))
    }
}
impl<T> Deref for RefCellSync<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct Executor(RefCellSync<FuturePoll>);

impl Executor {
    pub const fn new() -> Executor {
        Executor(RefCellSync::new(FuturePoll { future: None }))
    }

    pub fn set(&self, fut: Box<dyn Future<Output = ()>>) {
        *self.0.borrow_mut() = FuturePoll::new(fut)
    }

    pub fn poll(&self) -> Option<Done> {
        self.0.borrow_mut().poll()
    }
}
