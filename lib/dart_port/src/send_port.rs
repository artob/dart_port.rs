// This is free and unencumbered software released into the public domain.

use super::{
    prelude::{AtomicDartPortID, DartPortID, Result, ATOMIC_ORDERING, ILLEGAL_PORT_ID},
    Error, NativePort,
};
use dart_sys::Dart_CObject;
use std::{
    ffi::{CStr, CString},
    panic::RefUnwindSafe,
};

/// Sends messages to its [ReceivePort]s.
///
/// See: https://api.flutter.dev/flutter/dart-isolate/SendPort-class.html
#[derive(Debug, Default)]
pub struct SendPort {
    pub id: AtomicDartPortID,
}

unsafe impl Sync for SendPort {}

impl RefUnwindSafe for SendPort {}

impl NativePort for SendPort {
    fn id(&self) -> DartPortID {
        self.id.load(ATOMIC_ORDERING)
    }

    fn close(&self) -> Result<()> {
        let id = self.id.swap(ILLEGAL_PORT_ID, ATOMIC_ORDERING);
        if id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let close = dart_sys::Dart_CloseNativePort_DL.expect("Dart API initialized");
            if !close(id) {
                // See: PortMap::ClosePort in sdk/runtime/vm/port.cc
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/port.cc#L90
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/native_api_impl.cc#L104
                return Err(Error::InvalidPort);
            }
        }
        Ok(())
    }
}

#[allow(unused)]
impl SendPort {
    /// Returns the main port for the current isolate.
    #[cfg(feature = "static")]
    pub fn main() -> Self {
        use dart_sys::Dart_GetMainPortId;
        Self::open(unsafe { Dart_GetMainPortId() })
    }

    pub fn open(id: DartPortID) -> Self {
        Self {
            id: AtomicDartPortID::new(id),
        }
    }

    pub fn is_open(&self) -> bool {
        self.id() != ILLEGAL_PORT_ID
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

    #[track_caller]
    pub fn post_null(&self) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kNull,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_bool: false },
        })
    }

    #[track_caller]
    pub fn post_bool(&self, value: bool) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kBool,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_bool: value },
        })
    }

    #[track_caller]
    pub fn post_double(&self, value: f64) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kDouble,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_double: value },
        })
    }

    #[track_caller]
    pub fn post_integer(&self, value: i64) -> Result<()> {
        let port_id = self.id.load(ATOMIC_ORDERING);
        if port_id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let post = dart_sys::Dart_PostInteger_DL.expect("Dart API initialized");
            if !post(port_id, value) {
                // See: PortMap::PostMessage in sdk/runtime/vm/port.cc
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/port.cc#L152
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/native_api_impl.cc#L63
                return Err(Error::InvalidPort);
            }
        }
        Ok(())
    }

    #[track_caller]
    pub fn post_integers(&self, values: &[i64]) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kTypedData,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_typed_data: dart_sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_4 {
                    type_: dart_sys::Dart_TypedData_Type_Dart_TypedData_kInt64,
                    length: values.len() as _,
                    values: values.as_ptr().cast(),
                },
            },
        })
    }

    #[track_caller]
    pub fn post_string(&self, value: impl AsRef<str>) -> Result<()> {
        self.post_cstring(
            &CString::new(value.as_ref()).expect("string must not contain embedded NULs"),
        )
    }

    #[track_caller]
    pub fn post_cstr(&self, value: &CStr) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kString,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_string: value.as_ptr() as _,
            },
        })
    }

    #[track_caller]
    pub fn post_cstring(&self, value: &CString) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kString,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_string: value.as_ptr() as _,
            },
        })
    }

    #[track_caller]
    pub fn post_port(&self, port: &Self) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kSendPort,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_send_port: dart_sys::_Dart_CObject__bindgen_ty_1__bindgen_ty_1 {
                    id: port.id.load(ATOMIC_ORDERING),
                    origin_id: ILLEGAL_PORT_ID,
                },
            },
        })
    }

    pub fn post_cobject(&self, mut value: Dart_CObject) -> Result<()> {
        let port_id = self.id.load(ATOMIC_ORDERING);
        if port_id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let post = dart_sys::Dart_PostCObject_DL.expect("Dart API initialized");
            if !post(port_id, &mut value) {
                // See: PortMap::PostMessage in sdk/runtime/vm/port.cc
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/port.cc#L152
                // See: https://github.com/dart-lang/sdk/blob/3.3.3/runtime/vm/native_api_impl.cc#L59
                return Err(Error::InvalidPort);
            }
        }
        Ok(())
    }
}
