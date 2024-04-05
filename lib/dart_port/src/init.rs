// This is free and unencumbered software released into the public domain.

#[allow(unused)]
#[no_mangle]
pub unsafe extern "C" fn DartPort_InitializeApiDL(
    data: *mut ::core::ffi::c_void,
    port: i64,
) -> isize {
    return dart_sys::Dart_InitializeApiDL(data);
}
