// This is free and unencumbered software released into the public domain.

use super::prelude::Result;
use dart_sys::Dart_CObject;
use std::ffi::{CStr, CString};

/// See: https://api.dart.dev/stable/3.3.3/dart-isolate/SendPort-class.html
pub trait SendPort {
    fn post_null(&self) -> Result<()>;
    fn post_bool(&self, value: bool) -> Result<()>;
    fn post_double(&self, value: f64) -> Result<()>;
    fn post_integer(&self, value: i64) -> Result<()>;
    fn post_integers(&self, values: &[i64]) -> Result<()>;
    fn post_string(&self, value: impl AsRef<str>) -> Result<()>;
    fn post_cstr(&self, value: &CStr) -> Result<()>;
    fn post_cstring(&self, value: &CString) -> Result<()>;
    fn post_cobject(&self, value: Dart_CObject) -> Result<()>;
}
