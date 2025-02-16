use core::{fmt::Debug, marker::PhantomData, slice};

use crate::Provider;

pub struct Head<'a, T, P: Provider> {
    offset: usize,
    len: usize,
    provider: P,
    phantom: PhantomData<&'a T>,
}

impl<T, P: Provider> Head<'_, T, P> {
    pub(crate) fn new_unchecked(provider: P, from: usize, len: usize) -> Self {
        Self {
            offset: from,
            len,
            provider,
            phantom: PhantomData,
        }
    }

    pub fn map<U, F>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut [T]) -> U,
    {
        f(unsafe { self.get_slice_mut() })
    }

    pub fn inspect<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&[T]) -> U,
    {
        f(unsafe { self.get_slice() })
    }

    /// # Safety
    ///
    /// Don't store it for later use. Returned slice will become invalid
    /// if corresponding [`Tail`](super::Tail) reallocates.
    pub unsafe fn get_slice(&self) -> &[T] {
        let data = self.provider.get().add(self.offset) as *const T;
        slice::from_raw_parts(data, self.len)
    }

    /// # Safety
    ///
    /// Don't store it for later use. Returned slice will become invalid
    /// if corresponding [`Tail`](super::Tail) reallocates.
    pub unsafe fn get_slice_mut(&mut self) -> &mut [T] {
        let data = self.provider.get().add(self.offset) as *mut T;
        slice::from_raw_parts_mut(data, self.len)
    }
}

impl<T: Debug, P: Provider> Debug for Head<'_, T, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(unsafe { self.get_slice() }, f)
    }
}
