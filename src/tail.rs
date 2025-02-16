use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use alloc::{
    alloc::{Allocator, Global},
    vec::Vec,
};

use crate::{index_out_of_range, Head, Provider, SplitExtend};

/// Allows to extend [`Vec`] while hiding values before some index.
pub struct Tail<'a, T, P, A: Allocator = Global> {
    from: usize,
    list: &'a mut Vec<T, A>,
    provider: P,
    // Captures lifetime of the parent `Tail`.
    phantom: PhantomData<&'a mut Tail<'a, T, P, A>>,
}

impl<'a, T, P, A: Allocator> Tail<'a, T, P, A> {
    pub(crate) fn new_unchecked(from: usize, list: &'a mut Vec<T, A>, provider: P) -> Self {
        Self {
            from,
            list,
            provider,
            phantom: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.list.len() - self.from
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.list.capacity() - self.from
    }

    pub fn clear(&mut self) {
        self.list.truncate(self.from);
    }
}

impl<T, P: Provider<Item = T>, A: Allocator> Tail<'_, T, P, A> {
    pub fn push(&mut self, element: T) {
        self.list.push(element);
        self.provider.update(self.list.as_mut_ptr());
    }
}

impl<T, P: Provider<Item = T>, A: Allocator> SplitExtend<P, A> for Tail<'_, T, P, A> {
    type Item = T;

    fn split_extend(&mut self, at: usize) -> (Head<'_, Self::Item, P>, Tail<'_, Self::Item, P, A>) {
        if at > self.len() {
            index_out_of_range();
        }

        let head = Head::new_unchecked(self.provider.clone(), self.from, at);

        // SAFETY: Guarded by mutable reference to self.
        let tail = Tail::new_unchecked(
            self.from + at,
            unsafe { &mut *(self.list as *mut Vec<T, A>) },
            self.provider.clone(),
        );

        (head, tail)
    }
}

impl<T, P: Provider, A: Allocator> Deref for Tail<'_, T, P, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.list[self.from..]
    }
}

impl<T, P: Provider, A: Allocator> DerefMut for Tail<'_, T, P, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list[self.from..]
    }
}

impl<T, P: Provider<Item = T>, A: Allocator> Extend<T> for Tail<'_, T, P, A> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        Extend::extend(self.list, iter);
        self.provider.update(self.list.as_mut_ptr());
    }
}

impl<T: Debug, P: Provider, A: Allocator> Debug for Tail<'_, T, P, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}
