use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

const NUM_BUCKETS: u64 = 16;

pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K,V>>,
}

impl <K: Hash + PartialEq, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(NUM_BUCKETS as usize);
        for _ in 0..NUM_BUCKETS {
            buckets.push(Bucket { items: Vec::new() });
        }
        Self { buckets }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let items = &mut self.buckets[Self::bucket_index(&key)].items;
        let found = items.iter_mut().find(|pair| pair.0 == key);
        
        match found {
            None => {
                items.push((key, value));
                None
            }
            Some(pair) => {
                let old_pair = std::mem::replace(pair, (key, value));
                Some(old_pair.1)
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let items = &self.buckets[Self::bucket_index(&key)].items;
        let pair = items.iter().find(|pair| pair.0 == *key);

        match pair {
            Some((_, value)) => Some(value),
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        todo!()
    }


    fn bucket_index(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % NUM_BUCKETS) as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // create a new HashMap
        let mut map: HashMap<&str, &str> = HashMap::new();

        // insert key/value pairs into the HashMap
        assert_eq!(map.insert("foo", "bar"), None);
        // if an item already exists for that value, it should return the old value
        assert_eq!(map.insert("foo", "lol"), Some("bar"));

        // get a value based on its key
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // you should be able to do this multiple times
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // if no value exists for a key, return None
        assert_eq!(map.get(&"qux"), None);

        // remove a value for a key
        assert_eq!(map.remove(&"foo"), Some("lol"));
        // once it no longer exists in the map, it should return None
        assert_eq!(map.get(&"foo"), None);
                
        assert_eq!(2 + 2, 4);
    }
}
