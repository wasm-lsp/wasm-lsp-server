#![allow(missing_docs)]

use async_trait::async_trait;
use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};
use std::collections::{hash_map::RandomState, HashMap};

pub type Map<K, V, S = RandomState> = crate::core::RwLock<HashMap<K, crate::core::SharedValue<V>, S>>;

#[async_trait]
pub trait MapExt<'a, K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    async fn get<Q>(&'a self, key: &'a Q) -> Option<crate::core::Ref<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync;

    async fn get_mut<Q>(&'a self, key: &'a Q) -> Option<crate::core::RefMut<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync;

    async fn insert(&self, key: K, value: V) -> Option<V>;

    async fn remove<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync;
}

#[async_trait]
impl<'a, K, V, S> MapExt<'a, K, V, S> for Map<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    async fn get<Q>(&'a self, key: &'a Q) -> Option<crate::core::Ref<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync,
    {
        let guard = self.read().await;

        if let Some((key, value)) = guard.get_key_value(key) {
            #[allow(unsafe_code)]
            unsafe {
                let key: *const K = key;
                let value: *const V = value.get();
                Some(crate::core::Ref::new(guard, key, value))
            }
        } else {
            None
        }
    }

    async fn get_mut<Q>(&'a self, key: &'a Q) -> Option<crate::core::RefMut<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync,
    {
        let guard = self.write().await;

        if let Some((key, value)) = guard.get_key_value(key) {
            #[allow(unsafe_code)]
            unsafe {
                let key: *const K = key;
                let value: *mut V = value.as_ptr();
                Some(crate::core::RefMut::new(guard, key, value))
            }
        } else {
            None
        }
    }

    async fn insert(&self, key: K, value: V) -> Option<V> {
        self.write()
            .await
            .insert(key, crate::core::SharedValue::new(value))
            .map(crate::core::SharedValue::into_inner)
    }

    async fn remove<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized + Sync,
    {
        self.write().await.remove_entry(key).map(|(k, v)| (k, v.into_inner()))
    }
}
