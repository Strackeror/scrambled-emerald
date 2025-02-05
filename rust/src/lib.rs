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

mod resources {
    use alloc::boxed::Box;
    use alloc::vec;
    use core::ffi::c_void;

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
        pub fn buffer(&self) -> *mut c_void {
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
