//! Automatically rewritten from C Header to Rust Module
//! Source: fs/orangefs/protocol.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

// khandle stuff
//
// The 2.9 core will put 64 bit handles in here like this:
// 1234 0000 0000 5678
// The 3.0 and beyond cores will put 128 bit handles in here like this:
// 1234 5678 90AB CDEF
// The kernel module will always use the first four bytes and
// the last four bytes as an inum.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_khandle {
    pub u: [c_uchar; 16],
    pub __aligned(8): },
//
// kernel version of an object ref.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_object_kref {
    pub khandle: orangefs_khandle,
    pub fs_id: __s32,
    pub __pad1: __s32,
}

//
// compare 2 khandles assumes little endian thus from large address to
// small address
//
// pvfs2-types.h
pub const ORANGEFS_SUPER_MAGIC: c_uint = 0x20030528;
//
// ORANGEFS error codes are a signed 32-bit integer. Error codes are negative, but
// the sign is stripped before decoding.
//
// Bit 31 is not used since it is the sign.
//
// Bit 30 specifies that this is a ORANGEFS error. A ORANGEFS error is either an
// encoded errno value or a ORANGEFS protocol error.
//

//
// Bit 29 specifies that this is a ORANGEFS protocol error and not an encoded
// errno value.
//

//
// Bits 9, 8, and 7 specify the error class, which encodes the section of
// server code the error originated in for logging purposes. It is not used
// in the kernel except to be masked out.
//
pub const ORANGEFS_ERROR_CLASS_BITS: c_uint = 0x380;
// Bits 6 - 0 are reserved for the actual error code.
pub const ORANGEFS_ERROR_NUMBER_BITS: c_uint = 0x7f;
// Encoded errno values decoded by PINT_errno_mapping in orangefs-utils.c.
// Our own ORANGEFS protocol error codes.

// permission bits

// no ORANGEFS_U_VTX (sticky bit)

pub const ORANGEFS_ITERATE_START: c_int = 2147483646;
pub const ORANGEFS_ITERATE_END: c_int = 2147483645;

pub const ORANGEFS_MIRROR_FL: c_uint = 0x01000000ULL;

pub const ORANGEFS_XATTR_REPLACE: c_uint = 0x2;
pub const ORANGEFS_XATTR_CREATE: c_uint = 0x1;
pub const ORANGEFS_MAX_SERVER_ADDR_LEN: c_int = 256;
pub const ORANGEFS_NAME_MAX: c_int = 256;
//
// max extended attribute name len as imposed by the VFS and exploited for the
// upcall request types.
// NOTE: Please retain them as multiples of 8 even if you wish to change them
// This is *NECESSARY* for supporting 32 bit user-space binaries on a 64-bit
// kernel. Due to implementation within DBPF, this really needs to be
// ORANGEFS_NAME_MAX, which it was the same value as, but no reason to let it
// break if that changes in the future.
//

// XATTR_NAME_MAX defined
// by <linux/xattr.h>
//

// defined by <linux/xattr.h>
//

// defined by <linux/xattr.h>
//
// ORANGEFS I/O operation types, used in both system and server interfaces.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ORANGEFS_io_type {
    ORANGEFS_IO_READ = 1,
    ORANGEFS_IO_WRITE = 2
}

//
// If this enum is modified the server parameters related to the precreate pool
// batch and low threshold sizes may need to be modified  to reflect this
// change.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum orangefs_ds_type {
    ORANGEFS_TYPE_NONE = 0,
    ORANGEFS_TYPE_METAFILE = (1 << 0),
    ORANGEFS_TYPE_DATAFILE = (1 << 1),
    ORANGEFS_TYPE_DIRECTORY = (1 << 2),
    ORANGEFS_TYPE_SYMLINK = (1 << 3),
    ORANGEFS_TYPE_DIRDATA = (1 << 4),
    ORANGEFS_TYPE_INTERNAL = (1 << 5)	/* for the server's private use */
}

// This structure is used by the VFS-client interaction alone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ORANGEFS_keyval_pair {
    pub key: [c_char; ORANGEFS_MAX_XATTR_NAMELEN],
    pub /: *mut *mut __s32 key_sz; / __s32 for portable, fixed-size structures,
    pub val_sz: __s32,
    pub val: [c_char; ORANGEFS_MAX_XATTR_VALUELEN],
}

// pvfs2-sysint.h
// Describes attributes for a file, directory, or symlink.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ORANGEFS_sys_attr_s {
    pub owner: __u32,
    pub group: __u32,
    pub perms: __u32,
    pub atime: __u64,
    pub mtime: __u64,
    pub ctime: __u64,
    pub size: __s64,
// NOTE: caller must free if valid
    pub link_target: *mut c_char,
// Changed to __s32 so that size of structure does not change
    pub dfile_count: __s32,
// Changed to __s32 so that size of structure does not change
    pub distr_dir_servers_initial: __s32,
// Changed to __s32 so that size of structure does not change
    pub distr_dir_servers_max: __s32,
// Changed to __s32 so that size of structure does not change
    pub distr_dir_split_size: __s32,
    pub mirror_copies_count: __u32,
// NOTE: caller must free if valid
    pub dist_name: *mut c_char,
// NOTE: caller must free if valid
    pub dist_params: *mut c_char,
    pub dirent_count: __s64,
    pub objtype: orangefs_ds_type,
    pub flags: __u64,
    pub mask: __u32,
    pub blksize: __s64,
}

pub const ORANGEFS_LOOKUP_LINK_NO_FOLLOW: c_int = 0;
// pint-dev.h
// parameter structure used in ORANGEFS_DEV_DEBUG ioctl command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_mask_info_s {
    pub mask_type: },
    pub mask_value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_mask2_info_s {
    pub mask1_value: __u64,
    pub mask2_value: __u64,
}

// pvfs2-util.h
extern "C" {
    pub fn ORANGEFS_util_translate_mode(mode: c_int) -> __s32;
}
// pvfs2-debug.h

// pvfs2-internal.h

// pint-dev-shared.h

pub const ORANGEFS_READDIR_DEFAULT_DESC_COUNT: c_int = 5;
pub const DEV_GET_MAGIC: c_uint = 0x1;
pub const DEV_GET_MAX_UPSIZE: c_uint = 0x2;
pub const DEV_GET_MAX_DOWNSIZE: c_uint = 0x3;
pub const DEV_MAP: c_uint = 0x4;
pub const DEV_REMOUNT_ALL: c_uint = 0x5;
pub const DEV_DEBUG: c_uint = 0x6;
pub const DEV_UPSTREAM: c_uint = 0x7;
pub const DEV_CLIENT_MASK: c_uint = 0x8;
pub const DEV_CLIENT_STRING: c_uint = 0x9;
pub const DEV_MAX_NR: c_uint = 0xa;
// supported ioctls, codes are with respect to user-space
//
// version number for use in communicating between kernel space and user
// space. Zero signifies the upstream version of the kernel module.
//
pub const ORANGEFS_KERNEL_PROTO_VERSION: c_int = 0;
pub const ORANGEFS_MINIMUM_USERSPACE_VERSION: c_int = 20903;
//
// describes memory regions to map in the ORANGEFS_DEV_MAP ioctl.
// NOTE: See devorangefs-req.c for 32 bit compat structure.
// Since this structure has a variable-sized layout that is different
// on 32 and 64 bit platforms, we need to normalize to a 64 bit layout
// on such systems before servicing ioctl calls from user-space binaries
// that may be 32 bit!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ORANGEFS_dev_map_desc {
    pub ptr: *mut void __user,
    pub total_size: __s32,
    pub size: __s32,
    pub count: __s32,
}

// gossip.h
// try to avoid function call overhead by checking masks in macro

