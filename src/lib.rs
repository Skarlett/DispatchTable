//! dispatchtable is a runtime key based collection that contains references to functions.
//!
//! # Quick Start
//! ```
//! use dispatchtable::{Dispatch, DispatchTable};
//!
//! fn add(p: &(u8, u8)) -> u8 { p.0 + p.1 }
//! fn sub(p: &(u8, u8)) -> u8 { p.0 - p.1 }
//!
//! fn main() {
//!   let mut dispatchtable = DispatchTable::new();
//!
//!   dispatchtable.insert("add".to_string(), Box::new(add));
//!   dispatchtable.insert("sub".to_string(), Box::new(sub));
//!
//!   assert_eq!(dispatchtable.call("sub", &(10, 5)), Some(5));
//! }
//! ```

mod core;
#[cfg(test)]
mod test;

pub use crate::core::{DispatchFunction, Dispatcher as Dispatch};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub struct DispatchTable<K, P: ?Sized, R> {
    inner: HashMap<K, Box<dyn DispatchFunction<P, R>>>,
}

impl<K: Hash + Eq, P: ?Sized, R> DispatchTable<K, P, R> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<K, P: ?Sized, R> Dispatch<K, P, R> for DispatchTable<K, P, R>
where
    K: std::hash::Hash + std::cmp::Eq,
{
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.inner.contains_key(key)
    }

    fn remove<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.inner.remove(key);
    }
    fn insert(&mut self, key: K, item: Box<dyn DispatchFunction<P, R>>) {
        self.inner.insert(key, item);
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Box<dyn DispatchFunction<P, R>>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.inner.get(key)
    }

    fn into_vec(self) -> Vec<(K, Box<dyn DispatchFunction<P, R>>)> {
        self.inner.into_iter().collect()
    }
}
