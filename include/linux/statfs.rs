//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/statfs.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kstatfs {
    pub f_type: c_long,
    pub f_bsize: c_long,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: c_long,
    pub f_frsize: c_long,
    pub f_flags: c_long,
    pub f_spare: [c_long; 4],
}

//
// Definitions for the flag in f_flag.
//
// Generally these flags are equivalent to the MS_ flags used in the mount
// ABI.  The exception is ST_VALID which has the same value as MS_REMOUNT
// which doesn't make any sense for statfs.
//
pub const ST_RDONLY: c_uint = 0x0001	/* mount read-only */;
pub const ST_NOSUID: c_uint = 0x0002	/* ignore suid and sgid bits */;
pub const ST_NODEV: c_uint = 0x0004	/* disallow access to device special files */;
pub const ST_NOEXEC: c_uint = 0x0008	/* disallow program execution */;
pub const ST_SYNCHRONOUS: c_uint = 0x0010	/* writes are synced at once */;
pub const ST_VALID: c_uint = 0x0020	/* f_flags support is implemented */;
pub const ST_MANDLOCK: c_uint = 0x0040	/* allow mandatory locks on an FS */;
// 0x0080 used for ST_WRITE in glibc
// 0x0100 used for ST_APPEND in glibc
// 0x0200 used for ST_IMMUTABLE in glibc
pub const ST_NOATIME: c_uint = 0x0400	/* do not update access times */;
pub const ST_NODIRATIME: c_uint = 0x0800	/* do not update directory access times */;
pub const ST_RELATIME: c_uint = 0x1000	/* update atime relative to mtime/ctime */;
pub const ST_NOSYMFOLLOW: c_uint = 0x2000	/* do not follow symlinks */;
extern "C" {
    pub fn vfs_get_fsid(dentry: *mut dentry, fsid: *mut __kernel_fsid_t) -> c_int;
}
// Fold 16 bytes uuid to 64 bit fsid
