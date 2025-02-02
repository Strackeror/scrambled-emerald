#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::future::Future;
use core::ops::Deref;
use core::panic::PanicInfo;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use core::{alloc::GlobalAlloc, pin::Pin};

use arrayvec::ArrayVec;
use pokeemerald::{
    gTasks, AddTextPrinterParameterized, Alloc_, Free, IsTextPrinterActive, MgbaPrintf,
    RunTextPrinters, Task, FONT_NORMAL,
};
use slice_write::Write as _;

mod charmap;
mod slice_write;

struct PokeAllocator;
unsafe impl GlobalAlloc for PokeAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        Alloc_(layout.size() as u32, c"RUST".as_ptr()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        Free(ptr as *mut _);
    }
}
#[global_allocator]
static GLOBAL: PokeAllocator = PokeAllocator;

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

#[unsafe(link_section = ".ewram")]
static EXECUTOR: UnsafeSyncCell<FuturePoll> = UnsafeSyncCell::new(FuturePoll { future: None });

unsafe fn g_tasks(task_id: u8) -> *mut Task {
    #[allow(static_mut_refs)]
    gTasks.as_mut_ptr().add(task_id as usize)
}

struct WaitForTextPrinter(pub u8);
impl Future for WaitForTextPrinter {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { RunTextPrinters() };
        match unsafe { IsTextPrinterActive(self.0) } {
             0 => Poll::Ready(()),
             _ => Poll::Pending,
        }
    }
}

fn sleep(frames: usize) -> impl Future<Output = ()> {
    struct WaitUntil(usize);
    impl Future for WaitUntil {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 == 0 {
                return Poll::Ready(())
            }
            self.0 -= 1;
            Poll::Pending
        }
    }
    WaitUntil(frames)
}

async fn print(window_id: u8, font: u8, text: &[u8]) {
    unsafe {
        AddTextPrinterParameterized(window_id, font, text.as_ptr(), 0, 0, 1, None);
    };
    WaitForTextPrinter(window_id).await
}

async fn cool_rust_dialogue() {
    print(0, FONT_NORMAL as _, &pokestr!(b"Hello from rust!")).await;
    sleep(60).await;
    print(0, FONT_NORMAL as _, &pokestr!(b"Hello from rust again!")).await;
}

#[no_mangle]
pub extern "C" fn Task_HandleRust(task_id: u8) {
    unsafe {
        if g_tasks(task_id).as_ref().unwrap().data[15] == 0 {
            MgbaPrintf(2, c"Set future".as_ptr());
            *EXECUTOR.get() = FuturePoll::new(Box::pin(cool_rust_dialogue()));
            g_tasks(task_id).as_mut().unwrap().data[15] = 1;
            return;
        }
        (*EXECUTOR.get()).poll();
    }
}

fn dummy_raw_waker() -> RawWaker {
    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(|_| dummy_raw_waker(), |_| {}, |_| {}, |_| {});
    RawWaker::new(core::ptr::null(), &VTABLE)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

struct FuturePoll {
    future: Option<Pin<Box<dyn Future<Output = ()>>>>,
}

struct Done;

impl FuturePoll {
    fn new(obj: impl Future<Output = ()> + 'static) -> FuturePoll {
        FuturePoll {
            future: Some(Box::pin(obj)),
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        MgbaPrintf(1, c"PANIC".as_ptr());
        let mut text: ArrayVec<u8, 256> = Default::default();
        _ = write!(text, "{info:?}\0");
        MgbaPrintf(0, text.as_ptr());
    }
    loop {}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod pokeemerald {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
