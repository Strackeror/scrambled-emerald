#![no_std]
extern crate alloc;
use core::alloc::GlobalAlloc;
use core::ffi::c_void;
use core::fmt::Arguments;
use core::panic::PanicInfo;

use arrayvec::ArrayVec;
use future::RefCellSync;
use pokeemerald::{Alloc_, Free, MgbaPrintf};
use slice_write::Write as _;

mod charmap;
mod future;
mod party_screen;
mod slice_write;

struct PokeAllocator;
unsafe impl GlobalAlloc for PokeAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = Alloc_(layout.size() as u32, c"RUST".as_ptr()) as *mut u8;
        if ptr.is_null() {
            panic!("heap overflow")
        }
        // mgba_warn!("Alloc 0x{:x?} : {ptr:?}", layout.size());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        // mgba_warn!("Free: {:?}", ptr);
        Free(ptr as *mut _);
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
    unsafe { MgbaPrintf(level, buf.as_ptr()) }
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

mod resources {
    use alloc::boxed::Box;
    use alloc::vec;
    use core::cell::{Ref, RefCell, RefMut};
    use core::marker::PhantomData;
    use core::ops::Deref;

    use crate::pokeemerald::LZ77UnCompWram;

    pub struct AllocBuf<T: Sized> {
        data: RefCell<Box<[u8]>>,
        _p: PhantomData<T>,
    }

    impl<T> AllocBuf<T> {
        pub fn new(alloc: Box<[u8]>) -> Self {
            AllocBuf {
                data: RefCell::new(alloc),
                _p: PhantomData,
            }
        }

        pub fn size_bytes(&self) -> usize {
            self.data.borrow().len()
        }

        pub fn as_mut_ptr(&self) -> *mut T {
            self.data.borrow_mut().as_mut_ptr().cast()
        }
    }

    pub trait Buffer<T> {
        fn get(&self) -> impl Deref<Target = [T]>;

        fn size_bytes(&self) -> usize {
            self.get().len() * size_of::<T>()
        }
        fn as_ptr(&self) -> *const u8 {
            self.get().as_ptr().cast()
        }
    }

    impl<T> Buffer<T> for AllocBuf<T> {
        fn get(&self) -> impl Deref<Target = [T]> {
            unsafe { Ref::map(self.data.borrow(), |rbox| (&**rbox).align_to().1) }
        }
    }
    impl<T> Buffer<T> for &AllocBuf<T> {
        fn get(&self) -> impl Deref<Target = [T]> {
            (*self).get()
        }
    }
    impl<T> Buffer<T> for [T] {
        fn get(&self) -> impl Deref<Target = [T]> {
            self
        }
    }

    unsafe impl<const C: usize> Sync for CompressedResource<C> {}
    pub struct CompressedResource<const SIZE: usize> {
        data: *const u8,
    }

    pub const fn lz_ptr_res<const SIZE: usize>(data: *const u8) -> CompressedResource<SIZE> {
        CompressedResource { data }
    }

    impl<const SIZE: usize> CompressedResource<SIZE> {
        pub const fn from_ref(data: &'static [u8]) -> Self {
            CompressedResource {
                data: data.as_ptr(),
            }
        }

        pub fn load<T: Sized>(&self) -> AllocBuf<T> {
            const {
                if SIZE % size_of::<T>() != 0 {
                    panic!("Invalid length")
                };
            }

            let mut load = vec![0u8; SIZE];
            let dest = load.as_mut_ptr().cast();
            unsafe { LZ77UnCompWram(self.data.cast(), dest) };
            AllocBuf::new(load.into_boxed_slice())
        }
    }

    #[macro_export]
    macro_rules! include_res_lz {
        ($name:ident, $path:literal) => {
            static $name: $crate::resources::CompressedResource<{ include_bytes!($path).len() }> =
                $crate::resources::CompressedResource::from_ref(include_bytes!(concat!(
                    $path, ".lz"
                )));
        };
    }
    pub struct StaticWrapper<T>(RefCell<*mut T>);
    unsafe impl<T> Sync for StaticWrapper<T> {}
    impl<T> StaticWrapper<T> {
        pub const fn new(ptr: *mut T) -> StaticWrapper<T> {
            StaticWrapper(RefCell::new(ptr))
        }

        pub const fn new_from_arr(ptr: *mut [T; 0]) -> StaticWrapper<T> {
            unsafe { StaticWrapper(RefCell::new((*ptr).as_mut_ptr())) }
        }

        pub fn get_mut(&self) -> RefMut<T> {
            unsafe { RefMut::filter_map(self.0.borrow_mut(), |ptr| Some(&mut **ptr)).unwrap() }
        }

        pub fn get(&self) -> Ref<T> {
            unsafe { Ref::filter_map(self.0.borrow(), |ptr| Some(&**ptr)).unwrap() }
        }

        pub fn index_mut(&self, index: usize) -> RefMut<T> {
            unsafe {
                RefMut::filter_map(self.0.borrow_mut(), |ptr| Some(&mut *ptr.add(index))).unwrap()
            }
        }
    }
}

#[allow(unused)]
#[inline(never)]
fn stack_size() -> isize {
    extern "C" {
        static gAgbMainLoop_sp: *const c_void;
    }
    let mut offset = 0;
    offset = unsafe { (&raw const offset).offset_from(gAgbMainLoop_sp.cast()) };
    offset
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
