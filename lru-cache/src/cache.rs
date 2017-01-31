use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;
use list::{ Node, Link, List };

pub struct LruCache<T>
    where T: Hash + Eq + Clone {
    capacity: usize,
    size: usize,
    cache: HashMap<T, Vec<u8>>,
    keys: HashMap<T, Link<T>>,
    history: List<T>
}

impl<T> LruCache<T>
    where T: Hash + Eq + Clone {
    pub fn new(size: usize) -> Self {
        LruCache {
            capacity: size,
            size: 0,
            cache: HashMap::new(),
            keys: HashMap::new(),
            history: List::new()
        }
    }

    pub fn put(&mut self, key: T, value: Vec<u8>) {
        println!("Current size: {:?}", self.size);
        while self.size + value.iter().len() > self.capacity {
            println!("Too big!");
            if let Some(evict) = self.history.pop() {
                let ref e_key = evict.borrow().elem;
                self.keys.remove(&e_key);
                self.cache.remove(&e_key).map(|v| self.size -= v.iter().len());
            } else {
                //not enough space
                return
            }
        }

        if let Some(node) = self.keys.remove(&key) {
            let old_value = self.cache.get(&key).unwrap();
            self.size = self.size - old_value.iter().len() + value.iter().len();
            self.keys.insert(key.clone(), node.clone());
            self.history.promote(node);
        } else {
            self.size += value.iter().len();
            let node = Node::new(key.clone());
            self.keys.insert(key.clone(), node.clone());
            self.history.unshift(node);
        }
        self.cache.insert(key, value);
    }

    pub fn get(&mut self, key: &T) -> Option<&Vec<u8>> {
        self.keys.remove(key)
                 .and_then(move |node| {
                    self.keys.insert(key.clone(), node.clone());
                    self.history.promote(node);
                    self.cache.get(key)
                 })
    }
}

#[cfg(test)]
mod test {
    use super::LruCache;

    #[test]
    fn put_get() {
        let mut cache = LruCache::new(16);
        cache.put("test_1", "string of bytes".to_string().into_bytes());

        assert_eq!(cache.get(&"test_1"), Some(&("string of bytes".to_string().into_bytes())));
    }

    #[test]
    fn overwrite() {
        let mut cache = LruCache::new(16);
        cache.put("test_1", "string of bytes".to_string().into_bytes());
        cache.put("test_1", "another string".to_string().into_bytes());

        assert_eq!(cache.get(&"test_1"), Some(&("another string".to_string().into_bytes())));
    }

    #[test]
    fn evicts_after_size() {
        let mut cache = LruCache::new(8);
        let four: Vec<u8> = vec![1, 2, 3, 4];
        let eight: Vec<u8> = vec![5, 6, 7, 8];
        let twelve: Vec<u8> = vec![9, 10, 11, 12];

        cache.put("four", four.clone());
        cache.put("eight", eight.clone());
        cache.put("twelve", twelve.clone());

        assert_eq!(cache.get(&"twelve"), Some(&twelve));
        assert_eq!(cache.get(&"eight"), Some(&eight));
        assert_eq!(cache.get(&"four"), None);
    }

    #[test]
    fn evicts_least_recently_used() {
        let mut cache = LruCache::new(8);
        let four: Vec<u8> = vec![1, 2, 3, 4];
        let eight: Vec<u8> = vec![5, 6, 7, 8];
        let twelve: Vec<u8> = vec![9, 10, 11, 12];

        cache.put("four", four.clone());
        cache.put("eight", eight.clone());
        cache.get(&"four");
        cache.put("twelve", twelve.clone());

        assert_eq!(cache.get(&"twelve"), Some(&twelve));
        assert_eq!(cache.get(&"eight"), None);
        assert_eq!(cache.get(&"four"), Some(&four));
    }
}
