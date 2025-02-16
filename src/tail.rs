use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use alloc::{
    alloc::{Allocator, Global},
    vec::Vec,
};

use crate::{index_out_of_range, Head, NoProvider, Provider};

/// Extension trait to create new [`Tail`] from mutable reference to [`Vec`].
pub trait SplitExtend<'a, A: Allocator> {
    type Item;

    fn split_extend<P: Provider>(
        &'a mut self,
        offset: usize,
    ) -> (Head<'a, Self::Item, P>, Tail<'a, Self::Item, P, A>);

    fn tail(&'a mut self, offset: usize) -> Tail<'a, Self::Item, A, NoProvider<Self::Item>>;
}

/// Allows to extend [`Vec`] while hiding values before some index.
pub struct Tail<'a, T, P: Provider, A: Allocator = Global> {
    offset: usize,
    list: &'a mut Vec<T, A>,
    // Captures lifetime of the parent `Tail`.
    phantom: PhantomData<&'a mut Tail<'a, T, P, A>>,
}

impl<T, P: Provider, A: Allocator> Tail<'_, T, P, A> {
    pub fn len(&self) -> usize {
        self.list.len() - self.offset
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.list.capacity() - self.offset
    }

    pub fn clear(&mut self) {
        self.list.truncate(self.offset);
    }

    pub fn push(&mut self, element: T) {
        self.list.push(element);
    }
}

impl<'a, T, A: Allocator> SplitExtend<'a, A> for Vec<T, A> {
    type Item = T;

    fn split_extend<P: Provider>(
        &'a mut self,
        offset: usize,
    ) -> (Head<'a, Self::Item, P>, Tail<'a, Self::Item, P, A>) {
        if offset > self.len() {
            index_out_of_range();
        }

        let head = Head::new_unchecked(provider, offset, len);
    }

    fn tail(&'a mut self, offset: usize) -> Tail<'a, Self::Item, A> {
        if offset > self.len() {
            index_out_of_range();
        }

        Tail {
            offset,
            list: self,
            phantom: PhantomData,
        }
    }
}

// impl<'a, T, A: Allocator> TailExt<'a, A> for Tail<'a, T, A> {
//     type Item = T;

//     fn tail(&'a mut self, mut offset: usize) -> Tail<'a, Self::Item, A> {
//         if offset > self.len() {
//             index_out_of_range();
//         }

//         offset += self.offset;

//         Tail {
//             offset,
//             // SAFETY: Guarded by mutable reference to self.
//             list: unsafe { &mut *(self.list as *mut Vec<T, A>) },
//             phantom: PhantomData,
//         }
//     }
// }

impl<T, P: Provider, A: Allocator> Deref for Tail<'_, T, P, A> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.list[self.offset..]
    }
}

impl<T, P: Provider, A: Allocator> DerefMut for Tail<'_, T, P, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list[self.offset..]
    }
}

impl<T, P: Provider, A: Allocator> Extend<T> for Tail<'_, T, P, A> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        Extend::extend(self.list, iter);
    }
}

impl<T: Debug, P: Provider, A: Allocator> Debug for Tail<'_, T, P, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&**self, f)
    }
}
