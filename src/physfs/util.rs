use std::io::{Error, ErrorKind};
use super::get_last_error;

pub fn physfs_error_as_io_error() -> Error {
    Error::new(ErrorKind::Other,
               "PhysicsFS Error",
               get_last_error())
}

