//! PhysFS bindings for Rust

#![deny(missing_docs)]

#![feature(convert, std_misc, unsafe_destructor)]

extern crate libc;

pub use physfs::*;
pub use physfs::file::*;

/// PhysFS bindings
mod physfs;
/// Definitions for the PhysFS primitives
mod primitives;
