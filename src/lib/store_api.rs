use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Store {
    store: Arc<Mutex<BTreeMap<String, (String, Option<u64>)>>>,
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
            .try_lock().expect("Can't write")
            .insert(key.to_owned(), (value.to_owned(), None));
    }

    fn insert_ttl(&self, key: &String, value: &String, ttl: u32) {
        &self.store.try_lock().expect("Can't write").insert(
            key.to_owned(),
            (value.to_owned(), Some(epoch() + u64::from(ttl))),
        );
    }
    pub fn remove(&self, key: &String) {
        &self.store.try_lock().expect("Can't write").remove(key);
    }

    pub fn ttl(&self, key: &String, ttl: u32) -> Result<(), String>{
        if let Some(v) = &self.get(key) {
            self.insert_ttl(key, v, ttl);
            Ok(())
        } else {
            Err(String::from("Key not found."))
        }
    }

    pub fn get(&self, key: &String) -> Option<String> {
        let mut store = self.store.try_lock().expect("Can't write");
        if let Some((value, ttl)) = &store.get(key) {
            match ttl {
                None => Some(value.to_owned()),
                Some(ttl) => {
                    if *ttl < epoch(){
                        &store.remove(key);
                        None
                    } else {
                        Some(value.to_owned())
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.store
            .try_lock().expect("Can't write")
            .keys()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    }
}

fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
