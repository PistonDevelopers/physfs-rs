use primitives::*;
use super::get_last_error;
//use physfs::get_last_error;

#[link(name = "physfs")]
extern {
    //valid filehandle on success, NULL on failure
    fn PHYSFS_openAppend(filename : *const ::libc::c_char) -> *const RawFile;
    fn PHYSFS_openRead(filename : *const ::libc::c_char) -> *const RawFile;
    fn PHYSFS_openWrite(filename : *const ::libc::c_char) -> *const RawFile;

    //nonzero on success, 0 on failure (and the handle stays open)
    //The docs make it sound like failure is rare.
    fn PHYSFS_close(file : *const RawFile) -> ::libc::c_int;

    //Number of bytes read on success, -1 on failure.
    fn PHYSFS_read(file : *const RawFile, buffer : *const ::libc::c_void, obj_size : PHYSFS_uint32, obj_count : PHYSFS_uint32) -> PHYSFS_sint64;

    //Number of bytes written on success, -1 on failure.
    fn PHYSFS_write(file : *const RawFile, buffer : *const ::libc::c_void, obj_size : PHYSFS_uint32, obj_count : PHYSFS_uint32) -> PHYSFS_sint64;
}
///Possible ways to open a file.
enum OpenMode
{
    Append,
    Read,
    Write,
}
///A wrapper for the PHYSFS_File type.
struct RawFile {
    opaque : *const ::libc::c_void,
}

///A file handle.
pub struct File {
    raw : *const RawFile,
    mode : OpenMode,
}

impl File {
    ///Opens a file with a specific mode.
    fn open(filename : String, mode : OpenMode) -> Result<File, String> {
        let as_c_str : *const ::libc::c_char = filename.as_slice().as_ptr() as *const ::libc::c_char;
        let raw = match mode {
            Append => unsafe{PHYSFS_openAppend(as_c_str)},
            Read => unsafe{PHYSFS_openRead(as_c_str)},
            Write => unsafe{PHYSFS_openWrite(as_c_str)}
        };
        if raw.is_null() {Err(get_last_error())}
        else {Ok(File{raw : raw, mode : mode})}
    }
    ///Closes a file handle.
    fn close(&self) -> Result<(), String> {   
        match unsafe {PHYSFS_close(self.raw)} {
            0 => Err(get_last_error()),
            _ => Ok(())
        }
    }

    ///Reads from a file.
    fn read(&self, buf : &mut [u8], obj_size : u32, obj_count : u32) -> Result<u64, String> {
        let ret = unsafe {
            PHYSFS_read(
                self.raw, 
                buf.as_ptr() as *const ::libc::c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(get_last_error()),
            _ => Ok(ret as u64)
        }
    }

    ///Writes to a file.
    ///This code performs no safety checks to ensure
    ///that the buffer is the correct length.
    fn write(&self, buf : &[u8], obj_size : u32, obj_count : u32) -> Result<u64, String> {
        let ret = unsafe {
            PHYSFS_write(
                self.raw,
                buf.as_ptr() as *const ::libc::c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(get_last_error()),
            _ => Ok(ret as u64)
        }
    }
}