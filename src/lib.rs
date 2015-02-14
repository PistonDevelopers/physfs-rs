//! PhysFS bindings for Rust
#![crate_name = "physfs"]
#![crate_type = "lib"]

#![deny(missing_docs)]
#![feature(core)]
#![feature(env)]
#![feature(io)]
#![feature(libc)]
#![feature(std_misc)]
#![feature(unsafe_destructor)]
extern crate libc;

pub use physfs::*;
pub use physfs::file::*;

/// PhysFS bindings
mod physfs;
/// Definitions for the PhysFS primitives
mod primitives;

