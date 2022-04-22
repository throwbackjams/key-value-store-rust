use std::collections::HashMap;

pub struct KvStore{
    kv: HashMap<String, String>,
}

impl KvStore{

    pub fn new() -> KvStore {
        KvStore{
            kv: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.kv.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.kv.get(&key) {
            Some(key) => {
                Some(key.clone())
            },
            None => None,
        }
    }

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
        println!("{:?}", store.get("apple".to_owned()));

        
    }
}