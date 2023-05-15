use crate::Value;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),

    #[error("Can not convert value: {:0} to {1}")]
    ConvertError(Value, &'static str),

    #[error("Cannot process command {0} with table: {1}, key: {2}, Error: {}")]
    StorageError(&'static str, String, String, String),

    #[error("Not Implement yet")]
    NotImplemented(String),
}
