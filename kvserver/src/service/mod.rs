mod command_service;

use crate::pb::abi::CommandResponse;
use crate::storage::Storage;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
