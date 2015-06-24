//! PhysFS bindings for Rust

#![deny(missing_docs)]
#![feature(convert, static_mutex)]

extern crate libc;

pub use physfs::*;
pub use physfs::file::*;

/// PhysFS bindings
mod physfs;
/// Definitions for the PhysFS primitives
mod primitives;
