use primitives::*;
use super::{PhysFSContext, PHYSFS_LOCK};
use std::ffi::CString;
use libc::{c_int, c_char, c_void};
use std::io::{Reader, Writer, Seek, SeekStyle, IoResult, IoError, IoErrorKind};

#[link(name = "physfs")]
extern {
    // valid filehandle on success, NULL on failure
    fn PHYSFS_openAppend(filename: *const c_char) -> *const RawFile;
    fn PHYSFS_openRead(filename: *const c_char) -> *const RawFile;
    fn PHYSFS_openWrite(filename: *const c_char) -> *const RawFile;

    // nonzero on success, 0 on failure (and the handle stays open)
    // The docs make it sound like failure is rare.
    fn PHYSFS_close(file: *const RawFile) -> c_int;

    // Number of bytes read on success, -1 on failure.
    fn PHYSFS_read(file: *const RawFile, buffer: *mut c_void, obj_size: PHYSFS_uint32, obj_count: PHYSFS_uint32) -> PHYSFS_sint64;

    // Number of bytes written on success, -1 on failure.
    fn PHYSFS_write(file: *const RawFile, buffer: *const c_void, obj_size: PHYSFS_uint32, obj_count: PHYSFS_uint32) -> PHYSFS_sint64;

    // Current position in file, -1 on failure.
    fn PHYSFS_tell(file: *const RawFile) -> PHYSFS_sint64;

    // Seek to position in file; nonzero on succss, zero on error.
    fn PHYSFS_seek(file: *const RawFile, pos: PHYSFS_uint64) -> c_int;

    // nonzero if EOF, zero if not.
    fn PHYSFS_eof(file: *const RawFile) -> c_int;

    // Determine file size; returns -1 if impossible
    fn PHYSFS_fileLength(file: *const RawFile) -> PHYSFS_sint64;
}

/// Possible ways to open a file.
#[derive(Copy)]
pub enum Mode
{
    /// Append to the end of the file.
    Append,
    /// Read from the file.
    Read,
    /// Write to the file, overwriting previous data.
    Write,
}

/// A wrapper for the PHYSFS_File type.
#[repr(C)]
struct RawFile {
    opaque: *const c_void,
}

/// A file handle.
pub struct File<'f> {
    raw: *const RawFile,
    mode: Mode,
    context: &'f PhysFSContext,
}

fn physfs_error_as_ioerror() -> IoError {
    IoError {
        kind: IoErrorKind::OtherIoError,
        desc: "PhysicsFS Error",
        detail: Some(PhysFSContext::get_last_error()),
    }
}

impl <'f> File<'f> {
    /// Opens a file with a specific mode.
    pub fn open<'g>(context: &'g PhysFSContext, filename: String, mode: Mode) -> Result<File<'g>, String> {
        let _g = PHYSFS_LOCK.lock();
        let c_filename = CString::from_slice(filename.as_bytes());
        let raw = match mode {
            Mode::Append => unsafe{ PHYSFS_openAppend(c_filename.as_ptr()) },
            Mode::Read => unsafe{ PHYSFS_openRead(c_filename.as_ptr()) },
            Mode::Write => unsafe{ PHYSFS_openWrite(c_filename.as_ptr()) }
        };
        if raw.is_null() {Err(PhysFSContext::get_last_error())}
        else {Ok(File{raw: raw, mode: mode, context: context})}
    }

    /// Closes a file handle.
    fn close(&self) -> Result<(), String> {
        let _g = PHYSFS_LOCK.lock();
        match unsafe {
            PHYSFS_close(self.raw)
        } {
            0 => Err(PhysFSContext::get_last_error()),
            _ => Ok(())
        }
    }

    /// Reads from a file.
    pub fn read(&self, buf: &mut [u8], obj_size: u32, obj_count: u32) -> Result<u64, String> {
        let _g = PHYSFS_LOCK.lock();
        let ret = unsafe {
            PHYSFS_read(
                self.raw,
                buf.as_ptr() as *mut c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(PhysFSContext::get_last_error()),
            _ => Ok(ret as u64)
        }
    }

    /// Writes to a file.
    /// This code performs no safety checks to ensure
    /// that the buffer is the correct length.
    pub fn write(&self, buf: &[u8], obj_size: u32, obj_count: u32) -> Result<u64, String> {
        let _g = PHYSFS_LOCK.lock();
        let ret = unsafe {
            PHYSFS_write(
                self.raw,
                buf.as_ptr() as *const c_void,
                obj_size as PHYSFS_uint32,
                obj_count as PHYSFS_uint32
            )
        };

        match ret {
            -1 => Err(PhysFSContext::get_last_error()),
            _ => Ok(ret as u64)
        }
    }

    /// Checks whether eof is reached or not.
    pub fn eof(&self) -> bool {
        let _g = PHYSFS_LOCK.lock();
        let ret = unsafe {
            PHYSFS_eof(self.raw)
        };

        ret != 0
    }

    /// Determine length of file, if possible
    pub fn len(&self) -> Result<u64, String> {
        let _g = PHYSFS_LOCK.lock();
        let len = unsafe { PHYSFS_fileLength(self.raw) };

        if len >= 0 {
            Ok(len as u64)
        } else {
            Err(PhysFSContext::get_last_error())
        }
    }
}

impl <'f> Seek for File<'f> {
    /// Determines current position within a file
    fn tell(&self) -> IoResult<u64> {
        let _g = PHYSFS_LOCK.lock();
        let ret = unsafe {
            PHYSFS_tell(self.raw)
        };

        match ret {
            -1 => Err(physfs_error_as_ioerror()),
            _ => Ok(ret as u64)
        }
    }

    /// Seek to a new position within a file
    fn seek(&mut self, pos: i64, style: SeekStyle) -> IoResult<()> {
        let _g = PHYSFS_LOCK.lock();
        let ret = unsafe {
            PHYSFS_seek(
                self.raw,
                pos as PHYSFS_uint64
            )
        };

        match ret {
            -1 => Err(physfs_error_as_ioerror()),
            _ => Ok(())
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
