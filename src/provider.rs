use core::{cell::Cell, marker::PhantomData};

use alloc::rc::Rc;

pub struct NoProvider<T>(PhantomData<T>);

pub struct LocalProvider<T>(Rc<Cell<*mut T>>);

// pub struct SendProvider<T>(Arc<AtomicPtr<T>>);

/// Responsible for updating current address of the list and
/// informing arbitrary number of [`Head`](super::Head).
pub trait Provider: Sized + Clone + From<*mut Self::Item> {
    type Item;

    fn update_with<F>(&mut self, f: F)
    where
        F: FnOnce() -> *mut Self::Item;
    fn get(&self) -> *mut Self::Item;
}

impl<T> Provider for LocalProvider<T> {
    type Item = T;

    fn update_with<F>(&mut self, f: F)
    where
        F: FnOnce() -> *mut Self::Item,
    {
        let new_list_addr = f();
        self.0.set(new_list_addr);
    }

    fn get(&self) -> *mut Self::Item {
        self.0.get()
    }
}

// impl<T> Provider for SendProvider<T> {
//     type Item = T;

//     fn update_with<F>(&mut self, f: F)
//     where
//         F: FnOnce() -> *mut Self::Item,
//     {
//         self.0.fetch_update(set_order, fetch_order, f)
//         self.0.store(new_list_addr, Ordering::SeqCst);
//     }

//     fn get(&self) -> *mut Self::Item {
//         self.0.load(Ordering::Relaxed)
//     }
// }

impl<T> Clone for LocalProvider<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> From<*mut T> for LocalProvider<T> {
    fn from(value: *mut T) -> Self {
        LocalProvider(Rc::new(Cell::new(value)))
    }
}

// impl<T> Clone for SendProvider<T> {
//     fn clone(&self) -> Self {
//         Self(Arc::clone(&self.0))
//     }
// }

// impl<T> From<*mut T> for SendProvider<T> {
//     fn from(value: *mut T) -> Self {
//         SendProvider(Arc::new(AtomicPtr::new(value)))
//     }
// }
