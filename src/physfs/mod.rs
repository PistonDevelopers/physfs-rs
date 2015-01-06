use sync::mutex::{StaticMutex, MUTEX_INIT};
use std::c_str::CString;

///For locking physfs operations
static mut PHYSFS_LOCK : StaticMutex = MUTEX_INIT;
///Keep track of the number of global contexts.
static mut NUM_CONTEXTS : uint = 0;
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
    fn PHYSFS_mount(newDir : *const ::libc::c_char, mountPoint : *const ::libc::c_char, appendToPath : ::libc::c_int) -> ::libc::c_int;
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
            args[0].as_slice().as_ptr() as *const ::libc::c_char
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
    ///mountPoint is the location in the tree to mount it to.
    pub fn mount(&self, newDir : String, mountPoint : String, appendToPath : bool) -> Result<(), String>
    {
        match unsafe {
            let _g = PHYSFS_LOCK.lock();
            PHYSFS_mount(
                newDir.as_slice().as_ptr() as *const ::libc::c_char,
                mountPoint.as_slice().as_ptr() as *const ::libc::c_char,
                appendToPath as ::libc::c_int
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

        let cstr = unsafe{ CString::new(ptr, false) };
        let mut err = String::new();

        let mut it = cstr.as_bytes_no_nul().iter().map(|&x| x as u8 as char);
        for c in it {
            err.push(c);
        }
        err
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
