// This is free and unencumbered software released into the public domain.

use super::{
    prelude::{DartPortID, DartPortMessageHandler, Result, ILLEGAL_PORT_ID},
    Error,
};
use dart_sys::Dart_CObject;
use std::ptr::null;

/// A low-level asynchronous message receiver.
///
/// The port cannot be paused. The message handler must be set before
/// the first message is received, otherwise message will be lost.
///
/// See: https://api.flutter.dev/flutter/dart-isolate/RawReceivePort-class.html
#[derive(Debug, Default)]
pub struct RawReceivePort {
    pub id: DartPortID,
}

impl RawReceivePort {
    /// Opens a long-lived port for receiving messages.
    pub fn new(_handler: Option<DartPortMessageHandler>) -> Result<Self> {
        unsafe {
            let make_port = dart_sys::Dart_NewNativePort_DL.expect("Dart API initialized");
            let port_id = make_port(null(), Some(trampoline_callback), false);
            if port_id == ILLEGAL_PORT_ID {
                // See: PortMap::CreatePort in sdk/runtime/vm/port.cc
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/port.cc#L55
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/native_api_impl.cc#L74
                return Err(Error::Other("Dart_NewNativePort_DL".to_string()));
            }
            Ok(Self { id: port_id })
        }
    }

    /// Closes the port.
    ///
    /// After a call to this method, any incoming message is silently
    /// dropped. The handler will never be called again.
    pub fn close(&self) -> Result<()> {
        if self.id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let close = dart_sys::Dart_CloseNativePort_DL.expect("Dart API initialized");
            if !close(self.id) {
                return Err(Error::InvalidPort);
            }
            Ok(())
        }
    }
}

unsafe extern "C" fn trampoline_callback(port_id: i64, message: *mut Dart_CObject) {
    eprintln!("callback {}, {:?}", port_id, message); // TODO
}
