use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Store {
    store: Arc<Mutex<BTreeMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            store: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn insert(&self, key: &String, value: &String) {
        &self
            .store
            .try_lock()
            .expect("Can't obtain a lock")
            .insert(key.to_owned(), value.to_owned());
    }

    pub fn remove(&self, key: &String) {
        &self
            .store
            .try_lock()
            .expect("Can't obtain a lock")
            .remove(key);
    }

    pub fn get(&self, key: &String) -> Option<String> {
        self.store
            .try_lock()
            .expect("Can't obtain a lock")
            .get(key)
            .map(|x| x.to_owned())
    }
}
