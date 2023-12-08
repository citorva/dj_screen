use std::sync::{Arc, Mutex, MutexGuard};

pub struct SharedBuffer<T>(Arc<Mutex<T>>);

impl<T> SharedBuffer<T> {
    pub fn new(data : T) -> Self {
        SharedBuffer(Arc::new(Mutex::new(data)))
    }

    pub fn lock(&self) -> MutexGuard<T> {
        match self.0.lock() {
            Ok(guard) => guard,
            Err(err) => err.into_inner()
        }
    }
}

impl<T> Clone for SharedBuffer<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}