use alloc::{
    alloc::{Allocator, Global},
    vec::Vec,
};

use crate::{index_out_of_range, Head, Provider, Tail};

/// Extension trait to create new [`Tail`] from mutable reference to [`Vec`].
pub trait SplitExtend<'a, P: Provider<Item = Self::Item>, A: Allocator = Global> {
    type Item;

    #[allow(clippy::type_complexity)]
    fn split_extend(
        &'a mut self,
        at: usize,
    ) -> (Head<'a, Self::Item, P>, Tail<'a, Self::Item, P, A>);
}

// /// Special case of [`SplitExtend::split_extend`], without [`Head`](crate::Head) part.
// /// It allows save resources by not syncing current buffer address after reallocation.
// pub trait TailExt<'a, A: Allocator = Global> {
//     fn tail(&'a mut self, offset: usize) -> Tail<'a, Self::Item, NoProvider<Self::Item>, A>;
// }

impl<'a, T, P: Provider<Item = T>, A: Allocator> SplitExtend<'a, P, A> for Vec<T, A> {
    type Item = T;

    fn split_extend(
        &'a mut self,
        at: usize,
    ) -> (Head<'a, Self::Item, P>, Tail<'a, Self::Item, P, A>) {
        if at > self.len() {
            index_out_of_range();
        }

        let provider = P::from(self.as_mut_ptr());

        let head = Head::new_unchecked(provider.clone(), 0, at);
        let tail = Tail::new_unchecked(at, self, provider.clone());

        (head, tail)
    }
}
