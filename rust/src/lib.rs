#![no_std]
extern crate alloc;
use core::alloc::GlobalAlloc;
use core::panic::PanicInfo;

use arrayvec::ArrayVec;
use pokeemerald::{gTasks, Alloc_, Free, MgbaPrintf, Task};
use slice_write::Write as _;

mod charmap;
mod future;
mod party_screen;
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

unsafe fn g_tasks(task_id: u8) -> *mut Task {
    #[allow(static_mut_refs)]
    gTasks.as_mut_ptr().add(task_id as usize)
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
