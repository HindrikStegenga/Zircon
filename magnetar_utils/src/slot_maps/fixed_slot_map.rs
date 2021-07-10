use super::slot_map::*;
use crate::handles::{Handle, HandleType};

pub struct FixedSlotMap<T, K = u32>
where
    K: HandleType,
{
    slot_map: SlotMap<T, K>,
    max_size: K,
}

impl<T, K> FixedSlotMap<T, K>
where
    K: HandleType,
{
    pub fn new(max_size: K) -> FixedSlotMap<T, K> {
        FixedSlotMap {
            slot_map: SlotMap::new(),
            max_size,
        }
    }

    pub fn add(&mut self, item: T) -> Option<Handle<T, K>> {
        if self.len() >= self.max_size.to_usize().unwrap() {
            return None;
        }
        Some(self.slot_map.add(item))
    }

    pub fn get(&self, key: Handle<T, K>) -> Option<&T> {
        self.slot_map.get(key)
    }

    pub fn get_mut(&mut self, key: Handle<T, K>) -> Option<&mut T> {
        self.slot_map.get_mut(key)
    }

    pub fn remove(&mut self, key: Handle<T, K>) -> Option<T> {
        self.slot_map.remove(key)
    }

    pub fn clear(&mut self) {
        self.slot_map.clear()
    }

    pub fn len(&self) -> usize {
        return self.slot_map.len();
    }

    pub fn capacity(&self) -> usize {
        return self.slot_map.capacity();
    }

    pub fn data(&self) -> &[T] {
        self.slot_map.data()
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        self.slot_map.data_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        return self.slot_map.iter();
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        return self.slot_map.iter_mut();
    }

    pub fn iter_keys<'a>(&'a self) -> impl Iterator<Item = Handle<T, K>> + 'a {
        return SlotMapKeyIterator::from(&self.slot_map);
    }

    pub fn iter_key_values(&self) -> impl Iterator<Item = (Handle<T, K>, &T)> {
        return SlotMapKeyValueIterator::from(&self.slot_map);
    }
}

unsafe impl<T, K> Send for FixedSlotMap<T, K>
where
    T: Send,
    K: HandleType,
{
}

impl<T, K> Clone for FixedSlotMap<T, K>
where
    T: Clone,
    K: HandleType,
{
    fn clone(&self) -> Self {
        return Self {
            slot_map: self.slot_map.clone(),
            max_size: self.max_size,
        };
    }
}

impl<'a, T, K> IntoIterator for &'a FixedSlotMap<T, K>
where
    K: HandleType,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.slot_map.iter();
    }
}

impl<'a, T, K> IntoIterator for &'a mut FixedSlotMap<T, K>
where
    K: HandleType,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.slot_map.iter_mut();
    }
}
