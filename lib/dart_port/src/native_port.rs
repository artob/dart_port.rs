// This is free and unencumbered software released into the public domain.

use super::{
    prelude::{DartPortID, Result, ILLEGAL_PORT_ID},
    Error,
};

/// Base class for [ReceivePort] and [SendPort].
///
/// A port is used to send or receive inter-isolate messages.
pub trait NativePort {
    fn id(&self) -> DartPortID;

    /// Closes the port.
    fn close(&self) -> Result<()> {
        if self.id() == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let close = dart_sys::Dart_CloseNativePort_DL.expect("Dart API initialized");
            if !close(self.id()) {
                // See: PortMap::ClosePort in sdk/runtime/vm/port.cc
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/port.cc#L90
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/native_api_impl.cc#L104
                return Err(Error::InvalidPort);
            }
            Ok(())
        }
    }
}
