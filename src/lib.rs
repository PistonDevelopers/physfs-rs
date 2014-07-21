//!PhysFS bindings for Rust
#![crate_name = "physfs"]
#![crate_type = "lib"]
#![license = "zlib"]

#![deny(missing_doc)]
#![allow(dead_code)]
#![feature(globs)]
extern crate libc;

pub use physfs::*;

mod physfs;
mod primitives;