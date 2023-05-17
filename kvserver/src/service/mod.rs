mod command_service;

use std::sync::Arc;

use crate::command_request::RequestData;
use crate::pb::abi::{CommandRequest, CommandResponse};
use crate::storage::Storage;
use crate::{KvError, MemTable};

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store: store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        println!("cmd: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        println!("res: {:?}", res);
        res
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        Some(RequestData::Hexist(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        _ => KvError::NotImplemented("Not implemented".into()).into(),
    }
}
