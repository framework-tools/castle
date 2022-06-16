use std::{any::{TypeId, Any}, collections::HashMap};

pub struct State {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Try borrow a context value
    #[must_use]
    pub fn try_borrow<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>()).and_then(|v| v.downcast_ref::<T>())
    }

    /// Borrow a context value
    /// Panics if the value does not exist
    #[must_use]
    pub fn borrow<T: 'static>(&self) -> &T {
        self.try_borrow().unwrap_or_else(|| {
            panic!("Context value `{}` does not exist", std::any::type_name::<T>())
        })
    }

    /// Try borrow a mutable context value
    #[must_use]
    pub fn try_borrow_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>()).and_then(|v| v.downcast_mut::<T>())
    }

    /// Borrow a mutable context value
    /// Panics if the value does not exist
    #[must_use]
    pub fn borrow_mut<T: 'static>(&mut self) -> &mut T {
        match self.try_borrow_mut() {
            Some(v) => v,
            None => panic!("Context value `{}` does not exist", std::any::type_name::<T>()),
        }
    }

    /// Try take a context value
    #[must_use]
    pub fn try_take<T: 'static>(&mut self) -> Option<T> {
        self.map.remove(&TypeId::of::<T>()).and_then(|v| v.downcast::<T>().map(|v| *v).ok())
    }

    /// Take a context value
    /// Panics if the value does not exist
    #[must_use]
    pub fn take<T: 'static>(&mut self) -> T {
        match self.try_take() {
            Some(v) => v,
            None => panic!("Context value `{}` does not exist", std::any::type_name::<T>()),
        }
    }

    /// Insert a context value
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) -> Option<T> {
        self.map.insert(TypeId::of::<T>(), Box::new(value)).map(|v| *v.downcast::<T>().unwrap())
    }
}