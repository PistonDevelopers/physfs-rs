use std::env;
use std::ffi::{CString, CStr};
use std::io::Result;
use std::path::Path;
use std::ptr;
use libc::{c_int, c_char};

/// Utility
mod util;
/// File operations
pub mod file;

#[link(name = "physfs")]
extern {
    // nonzero on success, zero on error.
    fn PHYSFS_init(arg0: *const c_char) -> c_int;
    // nonzero if initialized, zero if not.
    fn PHYSFS_isInit() -> c_int;
    // nonzero if success, zero if error.
    fn PHYSFS_deinit() -> c_int;
    // string if success, NULL if error.
    fn PHYSFS_getLastError() -> *const c_char;
    // nonzero if success, zero if error
    fn PHYSFS_mount(new_dir: *const c_char, mount_point: *const c_char, append_to_path: c_int) -> c_int;
    // nonzero if success, zero if error.
    fn PHYSFS_setWriteDir(write_dir: *const c_char) -> c_int;
    // nonzero on success, zero on error.
    fn PHYSFS_mkdir(dir_name: *const c_char) -> c_int;
    // Checks if a given path exists; returns nonzero if true
    fn PHYSFS_exists(path: *const c_char) -> c_int;
    // Checks if a given path is a directory; returns nonzero if true
    fn PHYSFS_isDirectory(path: *const c_char) -> c_int;
}

/// Initializes the PhysFS library.
/// This must be called before using any other physfs methods (including physfs::File methods).
pub fn init() -> Result<()> {
    // Initializing multiple times throws an error. So let's not!
    if is_init() { return Ok(()); }

    let c_arg0 = match env::args().nth(0) {
        Some(arg0) => CString::new(arg0.as_bytes()).unwrap().as_ptr(),
        None => ptr::null()
    };

    let ret = unsafe { PHYSFS_init(c_arg0) };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

/// Checks if PhysFS is initialized.
pub fn is_init() -> bool
{
    unsafe { PHYSFS_isInit() != 0 }
}

/// De-initializes PhysFS.
/// It is recommended to close all file handles manually before calling this.
pub fn deinit()
{
    // de_init'ing more than once can cause a double-free -- do not want.
    if !is_init() { return; }
    unsafe { PHYSFS_deinit(); }
}

/// Adds an archive or directory to the search path.
/// mount_point is the location in the tree to mount it to.
pub fn mount(new_dir: &Path, mount_point: String, append_to_path: bool) -> Result<()>
{
    let c_new_dir = CString::new(new_dir.to_str().unwrap().as_bytes()).unwrap();
    let c_mount_point = CString::new(mount_point.as_bytes()).unwrap();

    let ret = unsafe {
        PHYSFS_mount(
            c_new_dir.as_ptr(),
            c_mount_point.as_ptr(),
            append_to_path as c_int
        )
    };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

/// Gets the last error message in a human-readable format.
/// This message may be localized, so do not expect it to match a specific string of characters.
pub fn get_last_error() -> Option<String> {
    let ptr: *const c_char = unsafe { PHYSFS_getLastError() };

    if ptr.is_null() {
        return None
    }

    let buf = unsafe { CStr::from_ptr(ptr).to_bytes() };
    let err = String::from_utf8(buf.to_vec()).unwrap();
    Some(err)
}

/// Sets a new write directory.
/// This method will fail if the current write dir still has open files in it.
pub fn set_write_dir(write_dir: &Path) -> Result<()> {
    let write_dir = CString::new(write_dir.to_str().unwrap().as_bytes()).unwrap();

    let ret = unsafe { PHYSFS_setWriteDir(write_dir.as_ptr()) };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

/// Creates a new dir relative to the write_dir.
pub fn mkdir(dir_name: &str) -> Result<()> {
    let c_dir_name = CString::new(dir_name.as_bytes()).unwrap();

    let ret = unsafe { PHYSFS_mkdir(c_dir_name.as_ptr()) };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

/// Checks if given path exists.
pub fn exists(path: &str) -> Result<()> {
    let c_path = CString::new(path.as_bytes()).unwrap();

    let ret = unsafe { PHYSFS_exists(c_path.as_ptr()) };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

/// Checks if given path is a directory.
pub fn is_directory(path: &str) -> Result<()> {
    let c_path = CString::new(path.as_bytes()).unwrap();

    let ret = unsafe { PHYSFS_isDirectory(c_path.as_ptr()) };

    match ret {
        0 => Err(util::physfs_error_as_io_error()),
        _ => Ok(())
    }
}

