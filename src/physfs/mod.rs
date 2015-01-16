use std::sync::{StaticMutex, MUTEX_INIT};
use std::ffi::{CString, c_str_to_bytes};

///For locking physfs operations
static mut PHYSFS_LOCK : StaticMutex = MUTEX_INIT;
///Keep track of the number of global contexts.
static mut NUM_CONTEXTS : usize = 0;
///File operations
pub mod file;

#[link(name = "physfs")]
extern {
    //nonzero on success, zero on error.
    fn PHYSFS_init(arg0 : *const ::libc::c_char) -> ::libc::c_int;
    //nonzero if initialized, zero if not.
    fn PHYSFS_isInit() -> ::libc::c_int;
    //nonzero if success, zero if error.
    fn PHYSFS_deinit() -> ::libc::c_int;
    //string if success, NULL if error.
    fn PHYSFS_getLastError() -> *const ::libc::c_char;
    //nonzero if success, zero if error
    fn PHYSFS_mount(new_dir : *const ::libc::c_char, mount_point : *const ::libc::c_char, append_to_path : ::libc::c_int) -> ::libc::c_int;
    //nonzero if success, zero if error.
    fn PHYSFS_setWriteDir(write_dir : *const ::libc::c_char) -> ::libc::c_int;
    //nonzero on success, zero on error.
    fn PHYSFS_mkdir(dir_name : *const ::libc::c_char) -> ::libc::c_int;
    //Checks if a given path exists; returns nonzero if true
    fn PHYSFS_exists(path: *const ::libc::c_char) -> ::libc::c_int;
}
///The access point for PhysFS function calls.
///
///It aims to be thread-safe.
pub struct PhysFSContext;

unsafe impl Send for PhysFSContext {}

impl PhysFSContext {
    ///Creates a new PhysFS context.
    pub fn new() -> Result<PhysFSContext, String> {
        //grab the lock before doing any of this.
        let _g = unsafe{ PHYSFS_LOCK.lock() };

        let con = PhysFSContext;
        match PhysFSContext::init() {
            Err(msg) => Err(msg),
            _ => { 
                //Everything's gone right so far
                //now, increment the instance counter
                unsafe { 
                    NUM_CONTEXTS += 1;
                }
                //and return the newly created context
                Ok(con)
            }
        }
    }

    ///initializes the PhysFS library.
    fn init() -> Result<(), String> {
        //Initializing multiple times throws an error. So let's not!
        if PhysFSContext::is_init() { return Ok(()); }

        let args = ::std::os::args();
        let arg0 : *const ::libc::c_char = if args.len() > 0 {
            CString::from_slice(args[0].as_bytes()).as_ptr()
        } else {
            ::std::ptr::null()
        };

        let ret = unsafe {
            PHYSFS_init(arg0)
        };

        match ret {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }
    }

    ///Checks if PhysFS is initialized
    pub fn is_init() -> bool
    {
        unsafe {PHYSFS_isInit() != 0}
    }

    ///De-initializes PhysFS. It is recommended to close
    ///all file handles manually before calling this.
    fn de_init()
    {
        //de_init'ing more than once can cause a double-free -- do not want.
        if !PhysFSContext::is_init() { return; }
        unsafe {
            PHYSFS_deinit();
        }
    }
    ///Adds an archive or directory to the search path.
    ///mount_point is the location in the tree to mount it to.
    pub fn mount(&self, new_dir : String, mount_point : String, append_to_path : bool) -> Result<(), String>
    {
        let c_new_dir = CString::from_slice(new_dir.as_bytes());
        let c_mount_point = CString::from_slice(mount_point.as_bytes());
        match unsafe {
            let _g = PHYSFS_LOCK.lock();
            PHYSFS_mount(
                c_new_dir.as_ptr(),
                c_mount_point.as_ptr(),
                append_to_path as ::libc::c_int
            )
        } {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }

    }

    ///Gets the last error message in a human-readable format
    ///This message may be localized, so do not expect it to 
    ///match a specific string of characters.
    pub fn get_last_error() -> String {
        let ptr : *const ::libc::c_char = unsafe {
            PHYSFS_getLastError() 
        };
        if ptr.is_null() {
            return "".to_string();
        }

        let bytes: &[u8] = unsafe { c_str_to_bytes(&ptr) };

        let mut err = String::new();

        let mut it = bytes.iter().map(|&x| x as u8 as char);
        for c in it {
            err.push(c);
        }
        err
    }

    ///Sets a new write directory.
    ///This method will fail if the current write dir
    ///still has open files in it.
    pub fn set_write_dir(&self, write_dir: &str) -> Result<(), String> {
        let _g = unsafe { PHYSFS_LOCK.lock() };
        let write_dir = CString::from_slice(write_dir.as_bytes());
        let ret = unsafe {
            PHYSFS_setWriteDir(write_dir.as_ptr())
        };

        match ret {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }
    }

    ///Creates a new dir relative to the write_dir.
    pub fn mkdir(dir_name: &str) -> Result<(), String> {
        let _g = unsafe { PHYSFS_LOCK.lock() };
        let c_dir_name = CString::from_slice(dir_name.as_bytes());
        let ret = unsafe {
            PHYSFS_mkdir(c_dir_name.as_ptr())
        };

        match ret {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }
    }

    ///Checks if given path exists
    pub fn exists(&self, path: &str) -> Result<(), String> {
        let _g = unsafe { PHYSFS_LOCK.lock() };
        let c_path = CString::from_slice(path.as_bytes());
        let ret = unsafe { PHYSFS_exists(c_path.as_ptr()) };

        if ret == 0 {
            Err(PhysFSContext::get_last_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for PhysFSContext {
    fn drop(&mut self) {
        //grab the lock before doing any of this!
        let _g = unsafe{ PHYSFS_LOCK.lock() };

        //decrement NUM_CONTEXTS
        unsafe {
            NUM_CONTEXTS -= 1;
        }
        //and de_init if there aren't any contexts left.
        if unsafe{NUM_CONTEXTS == 0} {
            PhysFSContext::de_init();
        }
    }
}
