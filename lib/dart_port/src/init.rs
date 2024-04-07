// This is free and unencumbered software released into the public domain.

/// # Safety
#[allow(unused)]
#[no_mangle]
pub unsafe extern "C" fn DartPort_InitializeApiDL(data: *mut ::core::ffi::c_void) -> isize {
    dart_sys::Dart_InitializeApiDL(data)
}
