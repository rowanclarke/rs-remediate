pub mod impls;
pub mod with;

use std::{mem, rc::Rc};

pub trait Cast<T>: Sized {
    type Output;

    unsafe fn from_raw(raw: *const T) -> Self::Output;
    unsafe fn into_raw(self) -> *const u8;
    fn size(&self) -> usize;

    fn cast(self) -> Self::Output {
        let pos = self.size() - mem::size_of::<T>();
        self.cast_at(pos)
    }

    fn cast_at(self, pos: usize) -> Self::Output {
        unsafe { Self::from_raw(self.into_raw().add(pos).cast()) }
    }
}

impl<'a, T: 'a> Cast<T> for &'a [u8] {
    type Output = &'a T;

    unsafe fn from_raw(raw: *const T) -> Self::Output {
        &*raw
    }

    unsafe fn into_raw(self) -> *const u8 {
        self.as_ptr()
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl<T: std::fmt::Debug> Cast<T> for Rc<[u8]> {
    type Output = Rc<T>;

    unsafe fn from_raw(raw: *const T) -> Self::Output {
        Rc::from_raw(raw)
    }

    unsafe fn into_raw(self) -> *const u8 {
        Rc::into_raw(self) as *const [u8] as *const u8
    }

    fn size(&self) -> usize {
        self.len()
    }
}
