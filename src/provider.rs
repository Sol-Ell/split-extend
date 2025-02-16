use core::{
    marker::PhantomData,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct NoProvider<T>(PhantomData<T>);
pub struct LocalProvider<T>(*mut T);
pub struct DefaultProvider<T>(AtomicPtr<T>);

/// Responsible for updating current address of the list and
/// informing arbitrary number of [`Head`](super::Head).
pub trait Provider {
    type Item;

    fn update(&mut self, new_list_addr: *mut Self::Item);
    fn get(&self) -> *mut Self::Item;
}

impl<T> Provider for NoProvider<T> {
    type Item = T;

    fn update(&mut self, _new_list_addr: *mut Self::Item) {}

    fn get(&self) -> *mut Self::Item {
        panic!("NoProvider doesn't support syncing of the address.")
    }
}

impl<T> Provider for LocalProvider<T> {
    type Item = T;

    fn update(&mut self, new_list_addr: *mut Self::Item) {
        self.0 = new_list_addr;
    }

    fn get(&self) -> *mut Self::Item {
        self.0
    }
}

impl<T> Provider for DefaultProvider<T> {
    type Item = T;

    fn update(&mut self, new_list_addr: *mut Self::Item) {
        self.0.store(new_list_addr, Ordering::SeqCst);
    }

    fn get(&self) -> *mut Self::Item {
        self.0.load(Ordering::Relaxed)
    }
}
