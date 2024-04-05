// This is free and unencumbered software released into the public domain.

use super::Error;
use dart_sys::{Dart_CObject, Dart_Port_DL};
use std::{
    mem::size_of,
    sync::atomic::{AtomicI64, Ordering},
};

pub type Result<T> = std::result::Result<T, Error>;

pub type DartPortID = Dart_Port_DL;

pub type AtomicDartPortID = AtomicI64;

pub type DartPortMessageHandler = extern "C" fn(message: Dart_CObject);

const _: () = assert!(
    size_of::<DartPortID>() == size_of::<AtomicDartPortID>(),
    "AtomicDartPortID size mismatch"
);

/// A port number guaranteed never to be associated with a valid port.
pub const ILLEGAL_PORT_ID: DartPortID = 0;

pub const ATOMIC_ORDERING: std::sync::atomic::Ordering = Ordering::SeqCst;
