#![allow(non_camel_case_types)]

pub type PHYSFS_uint8  = ::libc::c_uchar;
pub type PHYSFS_sint8  = ::libc::c_char;
pub type PHYSFS_uint16 = ::libc::c_ushort;
pub type PHYSFS_sint16 = ::libc::c_short;
pub type PHYSFS_uint32 = ::libc::c_uint;
pub type PHYSFS_sint32 = ::libc::c_int;

#[cfg(target_word_size = "64")] 
pub type PHYSFS_uint64 = ::libc::c_ulonglong;
#[cfg(target_word_size = "64")] 
pub type PHYSFS_sint64 = ::libc::c_longlong;

#[cfg(target_word_size = "32")]
pub type PHYSFS_uint64 = ::libc::c_uint;
#[cfg(target_word_size = "32")]
pub type PHYSFS_sint64 = ::libc::c_int;