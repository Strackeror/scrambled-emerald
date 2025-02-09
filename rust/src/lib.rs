#![no_std]
extern crate alloc;
use core::alloc::GlobalAlloc;
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
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
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
    use core::ffi::c_void;
    use core::ops::Deref;
    use core::slice;

    use crate::pokeemerald::LZ77UnCompWram;

    pub enum Resource {
        Compressed { len: usize, data: &'static [u8] },
        Direct(&'static [u8]),
    }

    pub enum LoadedResource {
        Compressed(Box<[u8]>),
        Direct(&'static [u8]),
    }

    impl Resource {
        pub const fn from_lz_ptr(data: *const u8, len: usize) -> Resource {
            unsafe {
                Resource::Compressed {
                    len: len,
                    data: slice::from_raw_parts(data, len),
                }
            }
        }

        pub const fn from_ptr(data: *const u8, len: usize) -> Resource {
            unsafe { Resource::Direct(slice::from_raw_parts(data, len)) }
        }

        pub const fn len(&self) -> usize {
            match self {
                Resource::Compressed { len, .. } => *len,
                Resource::Direct(items) => items.len(),
            }
        }

        pub fn buffer(&self) -> *const c_void {
            match self {
                Resource::Compressed { data, .. } => data.as_ptr() as *const _,
                Resource::Direct(data) => data.as_ptr() as *const _,
            }
        }

        pub fn load(&self) -> LoadedResource {
            match self {
                Resource::Compressed { len, data } => {
                    let mut load = vec![0; *len];
                    unsafe {
                        LZ77UnCompWram(data.as_ptr() as *const _, load.as_mut_ptr() as *mut _)
                    };
                    LoadedResource::Compressed(load.into_boxed_slice())
                }
                Resource::Direct(data) => LoadedResource::Direct(data),
            }
        }
    }

    impl LoadedResource {
        pub fn as_ptr(&self) -> *mut c_void {
            match self {
                LoadedResource::Compressed(items) => items.as_ptr() as *mut _,
                LoadedResource::Direct(items) => items.as_ptr() as *mut _,
            }
        }

        pub fn len(&self) -> usize {
            match self {
                LoadedResource::Compressed(items) => items.len(),
                LoadedResource::Direct(items) => items.len(),
            }
        }

        pub fn buffer(&self) -> &[u8] {
            match self {
                LoadedResource::Compressed(items) => &items,
                LoadedResource::Direct(items) => *items,
            }
        }
    }

    #[macro_export]
    macro_rules! include_res_lz {
        ($path:literal) => {
            $crate::resources::Resource::Compressed {
                len: include_bytes!($path).len(),
                data: include_bytes!(concat!($path, ".lz")),
            }
        };
    }
    #[macro_export]
    macro_rules! include_res {
        ($path:literal) => {
            $crate::resources::Resource::Direct(include_bytes($path))
        };
    }

    #[derive(Debug)]
    pub enum MayOwn<'a, T> {
        Ref(&'a T),
        Owned(T),
    }

    impl<T> From<T> for MayOwn<'_, T> {
        fn from(value: T) -> Self {
            MayOwn::Owned(value)
        }
    }
    impl<'a, T> From<&'a T> for MayOwn<'a, T> {
        fn from(value: &'a T) -> Self {
            MayOwn::Ref(value)
        }
    }
    impl<'a, T> Deref for MayOwn<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            match self {
                MayOwn::Ref(r) => r,
                MayOwn::Owned(o) => &o,
            }
        }
    }

    pub trait CheckPtrCast<T> {
        fn c_cast(self) -> *const T;
    }

    pub trait CheckPtrCastMut<T> {
        fn c_cast_mut(self) -> *mut T;
    }

    impl<T> CheckPtrCast<T> for *const c_void {
        fn c_cast(self) -> *const T {
            self as _
        }
    }
    impl<T> CheckPtrCast<T> for *mut c_void {
        fn c_cast(self) -> *const T {
            self as _
        }
    }
    impl<T> CheckPtrCastMut<T> for *mut c_void {
        fn c_cast_mut(self) -> *mut T {
            self as _
        }
    }

    macro_rules! checked_cast {
        ($src:ty, $dest:ty) => {
            impl CheckPtrCast<$dest> for *const $src {
                fn c_cast(self) -> *const $dest {
                    self as _
                }
            }
            impl CheckPtrCast<$dest> for *mut $src {
                fn c_cast(self) -> *const $dest {
                    self as _
                }
            }
            impl CheckPtrCastMut<$dest> for *mut $src {
                fn c_cast_mut(self) -> *mut $dest {
                    self as _
                }
            }
        };
    }
    checked_cast!(u32, u8);
    checked_cast!(u32, u16);
    checked_cast!(u16, u8);
    checked_cast!(u32, c_void);
    checked_cast!(u16, c_void);
    checked_cast!(u8, c_void);
    checked_cast!(i32, c_void);
    checked_cast!(i16, c_void);
    checked_cast!(i8, c_void);

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
