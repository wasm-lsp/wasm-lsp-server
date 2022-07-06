#![allow(missing_docs)]

use core::{
    cell::UnsafeCell,
    hash::Hash,
    ops::{Deref, DerefMut},
};
use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};

/// A simple wrapper around `T`
///
/// This is to prevent UB when using `HashMap::get_key_value`, because
/// `HashMap` doesn't expose an api to get the key and value, where
/// the value is a `&mut T`.
///
/// See [#10](https://github.com/xacrimon/dashmap/issues/10) for details
#[repr(transparent)]
pub struct SharedValue<T> {
    value: UnsafeCell<T>,
}

impl<T: Clone> Clone for SharedValue<T> {
    fn clone(&self) -> Self {
        let inner = self.get().clone();
        Self {
            value: UnsafeCell::new(inner),
        }
    }
}

#[allow(unsafe_code)]
unsafe impl<T: Send> Send for SharedValue<T> {
}

#[allow(unsafe_code)]
unsafe impl<T: Sync> Sync for SharedValue<T> {
}

impl<T> SharedValue<T> {
    /// Create a new `SharedValue<T>`
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Get a shared reference to `T`
    pub fn get(&self) -> &T {
        #[allow(unsafe_code)]
        unsafe {
            &*self.value.get()
        }
    }

    /// Get an unique reference to `T`
    pub fn get_mut(&mut self) -> &mut T {
        #[allow(unsafe_code)]
        unsafe {
            &mut *self.value.get()
        }
    }

    /// Unwraps the value
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Get a mutable raw pointer to the underlying value
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

pub struct Ref<'a, K, V, S = RandomState> {
    #[allow(unused)]
    guard: crate::core::RwLockReadGuard<'a, HashMap<K, SharedValue<V>, S>>,
    key: *const K,
    value: *const V,
}

impl<'a, K, V, S> Ref<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[allow(unsafe_code)]
    pub(crate) unsafe fn new(
        guard: crate::core::RwLockReadGuard<'a, HashMap<K, SharedValue<V>, S>>,
        key: *const K,
        value: *const V,
    ) -> Self {
        Self { guard, key, value }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn pair(&self) -> (&K, &V) {
        #[allow(unsafe_code)]
        unsafe {
            (&*self.key, &*self.value)
        }
    }
}

#[allow(unsafe_code)]
unsafe impl<'a, K, V, S> Send for Ref<'a, K, V, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher,
    V: Sync,
{
}
#[allow(unsafe_code)]
unsafe impl<'a, K, V, S> Sync for Ref<'a, K, V, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher,
    V: Sync,
{
}

impl<'a, K, V, S> std::fmt::Debug for Ref<'a, K, V, S>
where
    K: std::fmt::Debug + Eq + Hash,
    S: BuildHasher,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Ref")
            .field("k", &self.key)
            .field("v", &self.value)
            .finish()
    }
}

impl<'a, K, V, S> Deref for Ref<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

pub struct RefMut<'a, K, V, S = RandomState> {
    #[allow(unused)]
    guard: crate::core::RwLockWriteGuard<'a, HashMap<K, SharedValue<V>, S>>,
    k: *const K,
    v: *mut V,
}

impl<'a, K, V, S> RefMut<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[allow(unsafe_code)]
    pub(crate) unsafe fn new(
        guard: crate::core::RwLockWriteGuard<'a, HashMap<K, SharedValue<V>, S>>,
        k: *const K,
        v: *mut V,
    ) -> Self {
        Self { guard, k, v }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.pair_mut().1
    }

    pub fn pair(&self) -> (&K, &V) {
        #[allow(unsafe_code)]
        unsafe {
            (&*self.k, &*self.v)
        }
    }

    pub fn pair_mut(&mut self) -> (&K, &mut V) {
        #[allow(unsafe_code)]
        unsafe {
            (&*self.k, &mut *self.v)
        }
    }
}

#[allow(unsafe_code)]
unsafe impl<'a, K, V, S> Send for RefMut<'a, K, V, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher,
    V: Sync,
{
}
#[allow(unsafe_code)]
unsafe impl<'a, K, V, S> Sync for RefMut<'a, K, V, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher,
    V: Sync,
{
}

impl<'a, K, V, S> std::fmt::Debug for RefMut<'a, K, V, S>
where
    K: std::fmt::Debug + Eq + Hash,
    S: BuildHasher,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefMut")
            .field("k", &self.k)
            .field("v", &self.v)
            .finish()
    }
}

impl<'a, K, V, S> Deref for RefMut<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K, V, S> DerefMut for RefMut<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}
