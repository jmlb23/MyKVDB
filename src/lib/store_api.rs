use std::cell::RefCell;
use std::sync::Arc;
use std::collections::BTreeMap;

pub struct Store {
    store: Arc<RefCell<BTreeMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            store: Arc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn insert(&self, key: &String, value: &String) {
        &self.store.borrow_mut().insert(key.to_owned(), value.to_owned());
    }

    pub fn remove(&self, key: &String) {
        &self.store.borrow_mut().remove(key);
    }

    pub fn get(&self, key: &String) -> Option<String> {
        self.store.borrow().get(key).map(|x| x.to_owned())
    }
}
