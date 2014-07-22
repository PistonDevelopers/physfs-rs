//!PhysFS bindings for Rust
#![crate_name = "physfs"]
#![crate_type = "lib"]
#![license = "zlib"]

#![deny(missing_doc)]
#![allow(dead_code)]
#![feature(globs)]
#![feature(unsafe_destructor)]
extern crate libc;
extern crate sync;

pub use physfs::*;
pub use physfs::file::*;

///PhysFS bindings
mod physfs;
///Definitions for the PhysFS primitives
mod primitives;