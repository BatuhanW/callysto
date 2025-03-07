use std::str::Utf8Error;
use std::{io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, CallystoError>;

#[derive(Error, Debug)]
pub enum CallystoError {
    #[error("Failed to parse the configuration from env: `{0}`")]
    InvalidConfig(String),

    #[error("IO Error")]
    IO(#[from] io::Error),

    #[error("General error: {0}")]
    GeneralError(String),

    #[error("Binary Serialization error: {0}")]
    BinSerializationError(#[from] bincode::Error),

    #[error("No Consumer Stat: {0}")]
    ConsumerNoStat(String),

    #[error("No Topic named: {0}. Err: {1}")]
    NoTopic(String, String),

    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Kafka error: {0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error("UTF-8 Conversion error: {0}")]
    UTF8ConversionError(#[from] Utf8Error),

    #[error("RocksDB Error")]
    RocksDBError(#[from] rocksdb::Error),
}
