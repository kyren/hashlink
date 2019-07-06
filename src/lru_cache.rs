use std::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};

use hashbrown::hash_map;

use crate::linked_hash_map::{self, LinkedHashMap};

pub struct LruCache<K, V, S = hash_map::DefaultHashBuilder> {
    map: LinkedHashMap<K, V, S>,
    max_size: usize,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        LruCache {
            map: LinkedHashMap::new(),
            max_size: capacity,
        }
    }
}

impl<K: Eq + Hash, V, S> LruCache<K, V, S>
    where S: BuildHasher
{
    #[inline]
    pub fn with_hasher(capacity: usize, hash_builder: S) -> Self {
        LruCache {
            map: LinkedHashMap::with_hasher(hash_builder),
            max_size: capacity,
        }
    }

    #[inline]
    pub fn contains_key<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_mut(key).is_some()
    }

    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old_val = self.map.insert(k, v);
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        old_val
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self.map.raw_entry_mut().from_key(k) {
            linked_hash_map::RawEntryMut::Occupied(mut occupied) => {
                occupied.to_back();
                Some(occupied.into_mut())
            }
            linked_hash_map::RawEntryMut::Vacant(_) => None,
        }
    }

    /// If the returned entry is vacant, it will always have room to insert a single value.  By
    /// using the entry API, you can exceed the configured capacity by 1.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S> {
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        self.map.entry(key)
    }

    #[inline]
    pub fn raw_entry(&self) -> RawEntryBuilder<'_, K, V, S> {
        self.map.raw_entry()
    }

    /// If the returned entry is vacant, it will always have room to insert a single value.  By
    /// using the entry API, you can exceed the configured capacity by 1.
    #[inline]
    pub fn raw_entry_mut(&mut self) -> RawEntryBuilderMut<'_, K, V, S> {
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        self.map.raw_entry_mut()
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(k)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    #[inline]
    pub fn set_capacity(&mut self, capacity: usize) {
        for _ in capacity..self.len() {
            self.remove_lru();
        }
        self.max_size = capacity;
    }

    #[inline]
    pub fn remove_lru(&mut self) -> Option<(K, V)> {
        self.map.pop_front()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.map.iter_mut()
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        self.map.drain()
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher + Clone> Clone for LruCache<K, V, S> {
    #[inline]
    fn clone(&self) -> Self {
        LruCache {
            map: self.map.clone(),
            max_size: self.max_size,
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Extend<(K, V)> for LruCache<K, V, S> {
    #[inline]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> IntoIterator for LruCache<K, V, S> {
    type Item = (K, V);
    type IntoIter = Drain<K, V>;

    #[inline]
    fn into_iter(self) -> Drain<K, V> {
        self.map.into_iter()
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> IntoIterator for &'a LruCache<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> IntoIterator for &'a mut LruCache<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

pub type Drain<K, V> = linked_hash_map::Drain<K, V>;
pub type Iter<'a, K, V> = linked_hash_map::Iter<'a, K, V>;
pub type IterMut<'a, K, V> = linked_hash_map::IterMut<'a, K, V>;
pub type Entry<'a, K, V, S> = linked_hash_map::Entry<'a, K, V, S>;
pub type OccupiedEntry<'a, K, V> = linked_hash_map::OccupiedEntry<'a, K, V>;
pub type VacantEntry<'a, K, V, S> = linked_hash_map::VacantEntry<'a, K, V, S>;
pub type RawEntryBuilder<'a, K, V, S> = linked_hash_map::RawEntryBuilder<'a, K, V, S>;
pub type RawEntryBuilderMut<'a, K, V, S> = linked_hash_map::RawEntryBuilderMut<'a, K, V, S>;
pub type RawOccupiedEntryMut<'a, K, V> = linked_hash_map::RawOccupiedEntryMut<'a, K, V>;
pub type RawlVacantEntryMut<'a, K, V, S> = linked_hash_map::RawVacantEntryMut<'a, K, V, S>;
