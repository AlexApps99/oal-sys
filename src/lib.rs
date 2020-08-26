//! # OpenAL Rust bindings

#![allow(non_snake_case, non_upper_case_globals)]
#![allow(clippy::excessive_precision, clippy::redundant_static_lifetimes)]

#[cfg(not(feature = "generate"))]
mod openal;
#[cfg(feature = "generate")]
mod openal {
    include!(concat!(env!("OUT_DIR"), "/openal.rs"));
}

pub use openal::*;
