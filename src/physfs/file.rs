use primitives::*;
use super::{PhysFSContext, PHYSFS_LOCK};

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
    fn PHYSFS_read(file : *const RawFile, buffer : *mut ::libc::c_void, obj_size : PHYSFS_uint32, obj_count : PHYSFS_uint32) -> PHYSFS_sint64;

    //Number of bytes written on success, -1 on failure.
    fn PHYSFS_write(file : *const RawFile, buffer : *const ::libc::c_void, obj_size : PHYSFS_uint32, obj_count : PHYSFS_uint32) -> PHYSFS_sint64;
}
///Possible ways to open a file.
#[derive(Copy)]
pub enum Mode
{
    ///Append to the end of the file.
    Append,
    ///Read from the file.
    Read,
    ///Write to the file, overwriting previous data.
    Write,
}
///A wrapper for the PHYSFS_File type.
#[repr(C)]
struct RawFile {
    opaque : *const ::libc::c_void,
}

///A file handle.
pub struct File<'f> {
    raw : *const RawFile,
    mode : Mode,
    context : &'f PhysFSContext,
}

impl <'f> File<'f> {
    ///Opens a file with a specific mode.
    pub fn open<'g>(context : &'g PhysFSContext, filename : String, mode : Mode) -> Result<File<'g>, String> {
        let _g = unsafe{ PHYSFS_LOCK.lock()};
        let as_c_str : *const ::libc::c_char = filename.as_slice().as_ptr() as *const ::libc::c_char;
        let raw = match mode {
            Mode::Append => unsafe{ PHYSFS_openAppend(as_c_str) },
            Mode::Read => unsafe{ PHYSFS_openRead(as_c_str) },
            Mode::Write => unsafe{ PHYSFS_openWrite(as_c_str) }
        };
        if raw.is_null() {Err(PhysFSContext::get_last_error())}
        else {Ok(File{raw : raw, mode : mode, context : context})}
    }
    ///Closes a file handle.
    fn close(&self) -> Result<(), String> {
        match unsafe {
            let _g = PHYSFS_LOCK.lock();
            PHYSFS_close(self.raw)
        } {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }
    }

    ///Reads from a file.
    pub fn read(&self, buf : &mut [u8], obj_size : u32, obj_count : u32) -> Result<u64, String> {
        let ret = unsafe {
            let _g = PHYSFS_LOCK.lock();
            PHYSFS_read(
                self.raw, 
                buf.as_ptr() as *mut ::libc::c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(PhysFSContext::get_last_error()),
            _ => Ok(ret as u64)
        }
    }

    ///Writes to a file.
    ///This code performs no safety checks to ensure
    ///that the buffer is the correct length.
    pub fn write(&self, buf : &[u8], obj_size : u32, obj_count : u32) -> Result<u64, String> {
        let ret = unsafe {
            let _g = PHYSFS_LOCK.lock();
            PHYSFS_write(
                self.raw,
                buf.as_ptr() as *const ::libc::c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(PhysFSContext::get_last_error()),
            _ => Ok(ret as u64)
        }
    }
}
#[unsafe_destructor]
impl <'f> Drop for File<'f> {
    fn drop(&mut self) {
        match self.close() {
            _ => {}
        }
    }
}
