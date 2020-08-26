//! # OpenAL Rust bindings

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::excessive_precision)]

#[cfg(not(feature = "generate"))]
mod openal;
#[cfg(feature = "generate")]
mod openal {
    include!(concat!(env!("OUT_DIR"), "/openal.rs"));
}

pub use openal::*;
