mod file;

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
///initializes the PhysFS library.
pub fn init() -> Result<(), String> {
    //Initializing multiple times is undefined behavior
    if is_init() { return Ok(()); }
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
        0 => Err(get_last_error()),
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
pub fn de_init() -> Result<(), String>
{
    match unsafe{PHYSFS_deinit()}
    {
        0 => Err(get_last_error()),
        _ => Ok(())
    }
}
///Adds an archive or directory to the search path.
///mountPoint is the location in the tree to mount it to.
pub fn mount(newDir : String, mountPoint : String, appendToPath : bool) -> Result<(), String>
{
    match unsafe {
        PHYSFS_mount(
            newDir.as_slice().as_ptr() as *const ::libc::c_char,
            mountPoint.as_slice().as_ptr() as *const ::libc::c_char,
            appendToPath as ::libc::c_int
        )
    } {
        0 => Err(get_last_error()),
        _ => Ok(())
    }

}

///Gets the last error message in a human-readable format
///This message may be localized, so do not expect it to 
///match a specific string of characters.
fn get_last_error() -> String {
    let mut ptr : *const ::libc::c_char = unsafe {
        PHYSFS_getLastError() 
    };
    let mut err : String = String::new();
    while ptr.is_not_null() {
        let ch : char = unsafe{::std::ptr::read(ptr) as u8 as char};
        err.push_char(ch);        
        ptr = unsafe{ ptr.offset(1) };
    }
    err
}