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
        unsafe { Ref::map(self.data.borrow(), |rbox| (**rbox).align_to().1) }
    }
}
impl<T> Buffer<T> for &AllocBuf<T> {
    fn get(&self) -> impl Deref<Target = [T]> {
        (*self).get()
    }
}
impl<T> Buffer<T> for &[T] {
    fn get(&self) -> impl Deref<Target = [T]> {
        *self
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
            $crate::resources::CompressedResource::from_ref(include_bytes!(concat!($path, ".lz")));
    };
}

pub unsafe fn static_deref<'a, T>(target: *mut T) -> &'a mut T {
    unsafe { &mut *target }
}

pub unsafe fn static_index<'a, T>(target: *mut [T; 0], index: usize) -> &'a mut T {
    unsafe { &mut *(*target).as_mut_ptr().add(index) }
}

pub struct StaticWrapper<T>(RefCell<*mut T>);
unsafe impl<T> Sync for StaticWrapper<T> {}
impl<T> StaticWrapper<T> {
    pub const unsafe fn new(ptr: *mut T) -> StaticWrapper<T> {
        StaticWrapper(RefCell::new(ptr))
    }

    pub const unsafe fn new_from_arr(ptr: *mut [T; 0]) -> StaticWrapper<T> {
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
