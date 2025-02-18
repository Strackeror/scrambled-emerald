#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use core::alloc::GlobalAlloc;
use core::ffi::c_void;
use core::fmt::Arguments;
use core::panic::PanicInfo;

use arrayvec::ArrayVec;
use future::RefCellSync;
use pokeemerald::{Alloc_, Free};
use slice_write::Write as _;

pub mod charmap;
pub mod data;
pub mod future;
pub mod graphics;
pub mod input;
pub mod resources;
pub mod slice_write;

#[allow(unused)]
unsafe fn mgba_print(level: i32, bytes: &[u8]) {
    #[cfg(feature = "debug")]
    {
        use pokeemerald::MgbaPrintf;
        unsafe { MgbaPrintf(level, b"%s".as_ptr(), bytes.as_ptr()) }
    }
}

struct PokeAllocator;
unsafe impl GlobalAlloc for PokeAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = unsafe { Alloc_(layout.size() as u32, c"RUST".as_ptr()) } as *mut u8;
        if ptr.is_null() {
            panic!("heap overflow")
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { Free(ptr as *mut _) }
    }
}

#[global_allocator]
static GLOBAL: PokeAllocator = PokeAllocator;

#[unsafe(link_section = ".ewram")]
static PRINT_BUF: RefCellSync<ArrayVec<u8, 0x100>> = RefCellSync::new(ArrayVec::new_const());
pub fn mgba_print_format(level: i32, args: Arguments) {
    let mut buf = PRINT_BUF.borrow_mut();
    buf.clear();
    _ = buf.write_fmt(args);
    match buf.len() {
        0x100.. => buf[0x100] = 0,
        _ => buf.push(0),
    }
    unsafe { mgba_print(level, &buf) }
}

#[macro_export]
macro_rules! mgba_print {
    ($log_level:literal, $($tt:tt)*) => {
        $crate::mgba_print_format($log_level, format_args!($($tt)*))
    };
}
#[macro_export]
macro_rules! mgba_warn {
    ($($tt:tt)*) => {
        $crate::mgba_print_format(2, format_args!($($tt)*))
    };
}

#[allow(unused)]
#[inline(never)]
pub fn stack_size() -> isize {
    unsafe extern "C" {
        static gAgbMainLoop_sp: *const c_void;
    }
    let mut offset = 0;
    offset = unsafe { (gAgbMainLoop_sp).offset_from(&raw const offset as _) };
    offset
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        mgba_print(1, c"PANIC".to_bytes());
        let mut text: ArrayVec<u8, 256> = Default::default();
        _ = write!(text, "{info:?}\0");
        mgba_print(0, &text);
    }
    loop {}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
#[allow(unsafe_op_in_unsafe_fn)]
#[allow(clippy::useless_transmute)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::ptr_offset_with_cast)]
pub mod pokeemerald {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
