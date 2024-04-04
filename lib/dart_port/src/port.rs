// This is free and unencumbered software released into the public domain.

use super::{prelude::Result, Error, SendPort};
use dart_sys::{
    Dart_CObject, Dart_CloseNativePort_DL, Dart_Port_DL, Dart_PostCObject_DL, Dart_PostInteger_DL,
};
use std::{
    ffi::{CStr, CString},
    mem::size_of,
    sync::atomic::{AtomicI64, Ordering},
};

pub type DartPortID = Dart_Port_DL;
pub type AtomicDartPortID = AtomicI64;

/// A port number guaranteed never to be associated with a valid port.
const ILLEGAL_PORT_ID: DartPortID = 0;

const ATOMIC_ORDERING: std::sync::atomic::Ordering = Ordering::SeqCst;

const _: () = assert!(
    size_of::<DartPortID>() == size_of::<AtomicDartPortID>(),
    "AtomicDartPortID size mismatch"
);

/// A port is used to send or receive inter-isolate messages.
#[derive(Debug, Default)]
pub struct DartPort {
    pub id: AtomicDartPortID,
}

#[allow(unused)]
impl DartPort {
    pub const fn new() -> Self {
        Self::from_id(ILLEGAL_PORT_ID)
    }

    pub const fn from_id(id: DartPortID) -> Self {
        Self {
            id: AtomicDartPortID::new(id),
        }
    }

    /// Returns the main port for the current isolate.
    #[cfg(feature = "static")]
    pub fn main() -> Self {
        use dart_sys::Dart_GetMainPortId;
        Self::from_id(unsafe { Dart_GetMainPortId() })
    }

    pub fn is_open(&self) -> bool {
        self.id.load(ATOMIC_ORDERING) != ILLEGAL_PORT_ID
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

    pub fn open(&self, id: DartPortID) {
        self.id.store(id, ATOMIC_ORDERING);
    }

    pub fn close(&self) -> Result<()> {
        let id = self.id.swap(ILLEGAL_PORT_ID, ATOMIC_ORDERING);
        if id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let close = Dart_CloseNativePort_DL.expect("Dart API initialized");
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

impl SendPort for DartPort {
    #[track_caller]
    fn post_null(&self) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kNull,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_bool: false },
        })
    }

    #[track_caller]
    fn post_bool(&self, value: bool) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kBool,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_bool: value },
        })
    }

    #[track_caller]
    fn post_double(&self, value: f64) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kDouble,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 { as_double: value },
        })
    }

    #[track_caller]
    fn post_integer(&self, value: i64) -> Result<()> {
        let port_id = self.id.load(ATOMIC_ORDERING);
        if port_id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let post = Dart_PostInteger_DL.expect("Dart API initialized");
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
    fn post_integers(&self, values: &[i64]) -> Result<()> {
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
    fn post_string(&self, value: impl AsRef<str>) -> Result<()> {
        self.post_cstring(
            &CString::new(value.as_ref()).expect("string must not contain embedded NULs"),
        )
    }

    #[track_caller]
    fn post_cstr(&self, value: &CStr) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kString,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_string: value.as_ptr() as _,
            },
        })
    }

    #[track_caller]
    fn post_cstring(&self, value: &CString) -> Result<()> {
        self.post_cobject(Dart_CObject {
            type_: dart_sys::Dart_CObject_Type_Dart_CObject_kString,
            value: dart_sys::_Dart_CObject__bindgen_ty_1 {
                as_string: value.as_ptr() as _,
            },
        })
    }

    fn post_cobject(&self, mut value: Dart_CObject) -> Result<()> {
        let port_id = self.id.load(ATOMIC_ORDERING);
        if port_id == ILLEGAL_PORT_ID {
            return Err(Error::IllegalPort);
        }
        unsafe {
            let post = Dart_PostCObject_DL.expect("Dart API initialized");
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
