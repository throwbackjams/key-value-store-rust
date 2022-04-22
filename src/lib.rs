#![deny(missing_docs)]
//!An implementation of a key value store in Rust
use std::collections::HashMap;

///Primary struct is a KvStore containing a single HashMap
pub struct KvStore{
    kv: HashMap<String, String>,
}

impl KvStore{

    ///Create a hashmap
    pub fn new() -> KvStore {
        KvStore{
            kv: HashMap::new(),
        }
    }

    ///Use the default hashmap method
    pub fn set(&mut self, key: String, value: String) {
        self.kv.insert(key, value);
    }

    ///Use the default hashmap method
    pub fn get(&self, key: String) -> Option<String> {
        
        self.kv.get(&key).cloned()
    }

    ///Use the default hashmap method
    pub fn remove(&mut self, key: String) {
        self.kv.remove(&key);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_non_existant_kv() {
        let key = "nonexistant".to_owned();
        let store = KvStore::new();

        let result = store.get(key);

        assert_eq!(result, None);


    }

    #[test]
    fn set_kv() {
        let key = "apple".to_owned();
        let value = "red".to_owned();

        let mut store = KvStore::new();

        store.set(key, value);
        
        assert_eq!(store.get("apple".to_owned()), Some("red".to_owned()));
        
        store.set("apple".to_owned(), "brown".to_owned());
        assert_eq!(store.get("apple".to_owned()), Some("brown".to_owned()));

        
    }
}