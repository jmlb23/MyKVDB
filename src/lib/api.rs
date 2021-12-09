use crate::store_api::Store;
use jsonrpc_core::Error;
use jsonrpc_core::ErrorCode::ServerError;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Api {
    #[rpc(name = "ping")]
    fn ping(&self) -> Result<String>;
    #[rpc(name = "delete")]
    fn delete(&self, key: String) -> Result<()>;
    #[rpc(name = "get")]
    fn get(&self, key: String) -> Result<String>;
    #[rpc(name = "set")]
    fn set(&self, key: String, value: String) -> Result<()>;
    #[rpc(name = "update")]
    fn update(&self, key: String, new_value: String) -> Result<()>;
    #[rpc(name = "ttl")]
    fn ttl(&self, key: String, ttl: u32) -> Result<u32>;
}

pub struct ApiImpl {
    store: Store,
}

impl ApiImpl {
    pub fn new() -> Self {
        ApiImpl {
            store: Store::new(),
        }
    }
}

impl Api for ApiImpl {
    fn ping(&self) -> Result<String> {
        Ok(String::from("PONG"))
    }
    fn delete(&self, key: String) -> Result<()> {
        Ok(self.store.remove(&key))
    }
    fn set(&self, key: String, value: String) -> Result<()> {
        Ok(self.store.insert(&key, &value))
    }
    fn update(&self, key: String, new_value: String) -> Result<()> {
        &self.store.remove(&key);
        &self.store.insert(&key, &new_value);
        Ok(())
    }
    fn ttl(&self, key: String, ttl: u32) -> Result<u32> {
        match &self.store.ttl(&key, ttl) {
            Ok(_) => Ok(ttl),
            _ => Err(Error::new(ServerError(404))),
        }
    }
    fn get(&self, key: String) -> Result<String> {
        self.store.get(&key).ok_or(Error::new(ServerError(404)))
    }
}
