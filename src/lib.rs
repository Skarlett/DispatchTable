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
//!   let dispatchtable = DispatchTable::new();
//! 
//!   dispatchtable.insert("add", Box::new(add));
//!   dispatchtable.insert("sub", Box::new(sub));
//! 
//!   assert_eq!(dispatchtable.call("sub", &(10, 5)), Some(5));
//! }
//! ```



#[cfg(test)]
mod test;
mod core;

use std::collections::HashMap;
pub use crate::core::{DispatchFunction, Dispatcher as Dispatch};

pub struct DispatchTable<K, P, R> {
    inner: HashMap<K, Box<dyn DispatchFunction<P, R>>>
}

impl<K, P, R> DispatchTable<K, P, R> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
}

impl<K, P, R> Dispatch<K, P, R> for DispatchTable<K, P, R>
where
    K: std::hash::Hash + std::cmp::Eq
{   
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    fn remove(&mut self, key: &K) {
        self.inner.remove(key);
    }
    fn insert(&mut self, key: K, item: Box<dyn DispatchFunction<P, R>>) {
        self.inner.insert(key, item);
    }

    fn get(&self, key: &K) -> Option<&Box<dyn DispatchFunction<P, R>>> {
        self.inner.get(key)
    }

    fn into_vec(self) -> Vec<(K, Box<dyn DispatchFunction<P, R>> )> {
        self.inner.into_iter().collect()
    }
}