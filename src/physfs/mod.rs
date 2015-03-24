use std::ffi::{ CString, CStr };
use std::io::Result;
use std::path::AsPath;
use std::sync::{ StaticMutex, MUTEX_INIT };
use libc::{ c_int, c_char };

/// For locking physfs operations
static PHYSFS_LOCK: StaticMutex = MUTEX_INIT;
/// Keep track of the number of global contexts.
static mut NUM_CONTEXTS: usize = 0;

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

/// The access point for PhysFS function calls.
///
/// It aims to be thread-safe.
pub struct PhysFSContext;

unsafe impl Send for PhysFSContext {}

impl PhysFSContext {
    /// Creates a new PhysFS context.
    pub fn new() -> Result<Self> {
        // grab the lock before doing any of this.
        let _g = PHYSFS_LOCK.lock();

        let con = PhysFSContext;
        match PhysFSContext::init() {
            Err(e) => Err(e),
            _ => {
                // Everything's gone right so far
                // now, increment the instance counter
                unsafe {
                    NUM_CONTEXTS += 1;
                }
                // and return the newly created context
                Ok(con)
            }
        }
    }

    /// initializes the PhysFS library.
    fn init() -> Result<()> {
        // Initializing multiple times throws an error. So let's not!
        if PhysFSContext::is_init() { return Ok(()); }

        let mut args = ::std::env::args();
        let default_arg0 = "".to_string();
        let arg0 = args.next().unwrap_or(default_arg0);
        let c_arg0 = try!(CString::new(arg0));
        let ret = unsafe { PHYSFS_init(c_arg0.as_ptr()) };

        match ret {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }

    /// Checks if PhysFS is initialized
    pub fn is_init() -> bool {
        unsafe { PHYSFS_isInit() != 0 }
    }

    /// De-initializes PhysFS. It is recommended to close
    /// all file handles manually before calling this.
    fn de_init() {
        // de_init'ing more than once can cause a double-free -- do not want.
        if !PhysFSContext::is_init() { return }
        unsafe {
            PHYSFS_deinit();
        }
    }
    /// Adds an archive or directory to the search path.
    /// mount_point is the location in the tree to mount it to.
    pub fn mount<P: AsPath>(&self, new_dir: P, mount_point: String, append_to_path: bool) -> Result<()> {
        let _g = PHYSFS_LOCK.lock();
        let c_new_dir = try!(CString::new(new_dir.as_path().to_str().unwrap()));
        let c_mount_point = try!(CString::new(mount_point));
        match unsafe {
            PHYSFS_mount(
                c_new_dir.as_ptr(),
                c_mount_point.as_ptr(),
                append_to_path as c_int
            )
        } {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }

    /// Gets the last error message in a human-readable format
    /// This message may be localized, so do not expect it to
    /// match a specific string of characters.
    pub fn get_last_error() -> Option<String> {
        let ptr: *const c_char = unsafe {
            PHYSFS_getLastError()
        };
        if ptr.is_null() {
            return None
        }

        let buf = unsafe { CStr::from_ptr(ptr).to_bytes().to_vec() };
        let err = String::from_utf8(buf).unwrap();
        Some(err)
    }

    /// Sets a new write directory.
    /// This method will fail if the current write dir
    /// still has open files in it.
    pub fn set_write_dir<P: AsPath>(&self, write_dir: P) -> Result<()> {
        let _g = PHYSFS_LOCK.lock();
        let write_dir = try!(CString::new(write_dir.as_path().to_str().unwrap()));
        let ret = unsafe {
            PHYSFS_setWriteDir(write_dir.as_ptr())
        };

        match ret {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }

    /// Creates a new dir relative to the write_dir.
    pub fn mkdir(&self, dir_name: &str) -> Result<()> {
        let _g = PHYSFS_LOCK.lock();
        let c_dir_name = try!(CString::new(dir_name));
        let ret = unsafe {
            PHYSFS_mkdir(c_dir_name.as_ptr())
        };

        match ret {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }

    /// Checks if given path exists
    pub fn exists(&self, path: &str) -> Result<()> {
        let _g = PHYSFS_LOCK.lock();
        let c_path = try!(CString::new(path));
        let ret = unsafe { PHYSFS_exists(c_path.as_ptr()) };

        match ret {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }

    /// Checks if given path is a directory
    pub fn is_directory(&self, path: &str) -> Result<()> {
        let _g = PHYSFS_LOCK.lock();
        let c_path = try!(CString::new(path));
        let ret = unsafe { PHYSFS_isDirectory(c_path.as_ptr()) };

        match ret {
            0 => Err(util::physfs_error_as_io_error()),
            _ => Ok(())
        }
    }
}

impl Drop for PhysFSContext {
    fn drop(&mut self) {
        // grab the lock before doing any of this!
        let _g = PHYSFS_LOCK.lock();

        // decrement NUM_CONTEXTS
        unsafe {
            NUM_CONTEXTS -= 1;
        }
        // and de_init if there aren't any contexts left.
        if unsafe { NUM_CONTEXTS == 0 } {
            PhysFSContext::de_init();
        }
    }
}
