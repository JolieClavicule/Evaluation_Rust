use std::collections::HashMap;
use std::collections::VecDeque;
use super::cache_trait::Cache;

pub struct LRUCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    // Création d'un nouveau cache LRU
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            if let Some(least_used) = self.order.pop_front() {
                self.map.remove(&least_used);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn size(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn remove(&mut self, key: &K) {
        self.map.remove(key);
        self.order.retain(|k| k != key);
    }
}