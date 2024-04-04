// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("illegal port ID (port not opened?)")]
    IllegalPort,

    #[error("invalid port ID (port already closed?)")]
    InvalidPort,

    #[error("{0}")]
    Other(String),
}
