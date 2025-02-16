use core::{
    cell::Cell,
    marker::PhantomData,
    sync::atomic::{AtomicPtr, Ordering},
};

use alloc::{rc::Rc, sync::Arc};

pub struct NoProvider<T>(PhantomData<T>);

pub struct LocalProvider<T>(Rc<Cell<*mut T>>);

pub struct SendProvider<T>(Arc<AtomicPtr<T>>);

/// Responsible for updating current address of the list and
/// informing arbitrary number of [`Head`](super::Head).
pub trait Provider: Clone + From<*mut Self::Item> {
    type Item;

    fn update(&mut self, new_list_addr: *mut Self::Item);
    fn get(&self) -> *mut Self::Item;
}

impl<T> Provider for LocalProvider<T> {
    type Item = T;

    fn update(&mut self, new_list_addr: *mut Self::Item) {
        self.0.set(new_list_addr);
    }

    fn get(&self) -> *mut Self::Item {
        self.0.get()
    }
}

impl<T> Provider for SendProvider<T> {
    type Item = T;

    fn update(&mut self, new_list_addr: *mut Self::Item) {
        self.0.store(new_list_addr, Ordering::SeqCst);
    }

    fn get(&self) -> *mut Self::Item {
        self.0.load(Ordering::Relaxed)
    }
}

impl<T> Clone for LocalProvider<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Clone for SendProvider<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> From<*mut T> for NoProvider<T> {
    fn from(_value: *mut T) -> Self {
        NoProvider(PhantomData)
    }
}

impl<T> From<*mut T> for LocalProvider<T> {
    fn from(value: *mut T) -> Self {
        LocalProvider(Rc::new(Cell::new(value)))
    }
}

impl<T> From<*mut T> for SendProvider<T> {
    fn from(value: *mut T) -> Self {
        SendProvider(Arc::new(AtomicPtr::new(value)))
    }
}
