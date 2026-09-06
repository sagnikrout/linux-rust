//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_xattr.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) International Business Machines Corp., 2000-2002
//

// Macro flag: #define H_JFS_XATTR

//
// jfs_ea_list describe the on-disk format of the extended attributes.
// I know the null-terminator is redundant since namelen is stored, but
// I am maintaining compatibility with OS/2 where possible.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_ea {
    pub /: *mut *mut u8 flag; / Unused?,
    pub /: *mut *mut u8 namelen; / Length of name,
    pub /: *mut *mut __le16 valuelen; / Length of value,
    pub /: *mut *mut char name[]; / Attribute name (includes null-terminator),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_ea_list {
    pub /: *mut *mut __le32 size; / overall size,
    pub /: *mut *mut jfs_ea ea[]; / Variable length list,
}

// Macros for defining maximum number of bytes supported for EAs
pub const MAXEASIZE: c_int = 65535;

//
// some macros for dealing with variable length EA lists.
//

extern "C" {
    pub fn __jfs_getxattr(: *mut inode, : *const c_char, : *mut c_void, _arg: usize) -> isize;
}
extern "C" {
    pub fn jfs_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}

