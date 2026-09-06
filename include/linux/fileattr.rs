//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fileattr.h
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
// Flags shared betwen flags/xflags

// Read-only inode flags

// Flags to indicate valid value of fsx_ fields

// Flags for directories

// Misc settable flags

//
// Merged interface for miscellaneous file attributes.  'flags' originates from
// ext* and 'fsx_flags' from xfs.  There's some overlap between the two, which
// is handled by the VFS helpers, so filesystems are free to implement just one
// or both of these sub-interfaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_kattr {
    pub /: *mut *mut u32 flags; / flags (FS_IOC_GETFLAGS/FS_IOC_SETFLAGS),
// struct fsxattr:
    pub /: *mut *mut u32 fsx_xflags; / xflags field value (get/set),
    pub (get/set)*/: *mut *mut u32 fsx_extsize; / extsize field value,
    pub /: *mut *mut u32 fsx_nextents; / nextents field value (get),
    pub /: *mut *mut u32 fsx_projid; / project identifier (get/set),
    pub (get/set)*/: *mut *mut u32 fsx_cowextsize; / CoW extsize field value,
// selectors:
    pub flags_valid:1: bool,
    pub fsx_valid:1: bool,
}

extern "C" {
    pub fn copy_fsxattr_to_user(fa: *const file_kattr, ufa: *mut fsxattr __user) -> c_int;
}
extern "C" {
    pub fn fileattr_fill_xflags(fa: *mut file_kattr, xflags: u32);
}
extern "C" {
    pub fn fileattr_fill_flags(fa: *mut file_kattr, flags: u32);
}
//
// fileattr_has_fsx - check for extended flags/attributes
// @fa:		fileattr pointer
//
// Return: true if any attributes are present that are not represented in
// ->flags.
//
extern "C" {
    pub fn vfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ioctl_getflags(file: *mut file, argp: *mut unsigned int __user) -> c_int;
}
extern "C" {
    pub fn ioctl_setflags(file: *mut file, argp: *mut unsigned int __user) -> c_int;
}
extern "C" {
    pub fn ioctl_fsgetxattr(file: *mut file, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn ioctl_fssetxattr(file: *mut file, argp: *mut void __user) -> c_int;
}
