// This is free and unencumbered software released into the public domain.

#[allow(unused)]
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "dynamic")]
    "dynamic",
    #[cfg(feature = "static")]
    "static",
];
