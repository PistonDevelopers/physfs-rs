use sync::mutex::{StaticMutex, MUTEX_INIT, Guard};
use std::c_str::CString;

///For locking physfs operations
static mut PHYSFS_LOCK : StaticMutex = MUTEX_INIT;
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
#[deriving(Send)]
pub struct PhysFSContext<'g> {
    _g : Guard<'g>,
}

impl <'g> PhysFSContext<'g> {
    ///Creates a new PhysFS context.
    pub fn new() -> Result<PhysFSContext<'g>, String> {
        let con = PhysFSContext{_g : unsafe{PHYSFS_LOCK.lock()}};
        match con.init() {
            Err(msg) => Err(msg),
            _ => {
                Ok(con)
            }
        }
    }

    ///initializes the PhysFS library.
    fn init(&self) -> Result<(), String> {
        //Initializing multiple times throws an error. So let's not!
        if self.is_init() { return Ok(()); }

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
            0 => Err(self.get_last_error()),
            _ => Ok(())
        }
    }

    ///Checks if PhysFS is initialized
    pub fn is_init(&self) -> bool
    {
        unsafe {PHYSFS_isInit() != 0}
    }

    ///De-initializes PhysFS. It is recommended to close
    ///all file handles manually before calling this.
    fn de_init(&self)
    {
        //de_init'ing more than once can cause a double-free -- do not want.
        if !self.is_init() { return; }
        unsafe {
            PHYSFS_deinit();
        }
    }
    ///Adds an archive or directory to the search path.
    ///mountPoint is the location in the tree to mount it to.
    pub fn mount(&self, newDir : String, mountPoint : String, appendToPath : bool) -> Result<(), String>
    {
        match unsafe {
            PHYSFS_mount(
                newDir.as_slice().as_ptr() as *const ::libc::c_char,
                mountPoint.as_slice().as_ptr() as *const ::libc::c_char,
                appendToPath as ::libc::c_int
            )
        } {
            0 => Err(self.get_last_error()),
            _ => Ok(())
        }

    }

    ///Gets the last error message in a human-readable format
    ///This message may be localized, so do not expect it to 
    ///match a specific string of characters.
    pub fn get_last_error(&self) -> String {
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
            err.push_char(c);
        }
        err
    }
}