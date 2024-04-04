// This is free and unencumbered software released into the public domain.

use super::Error;

pub type Result<T> = std::result::Result<T, Error>;
