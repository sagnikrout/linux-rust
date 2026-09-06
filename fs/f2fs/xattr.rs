//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/xattr.h
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
//
// fs/f2fs/xattr.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Portions of this code from linux/fs/ext2/xattr.h
//
// On-disk format of extended attributes for the ext2 filesystem.
//
// (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
//

// Magic value in attribute blocks
pub const F2FS_XATTR_MAGIC: c_uint = 0xF2F52011;
// Maximum number of references to one attribute block
pub const F2FS_XATTR_REFCOUNT_MAX: c_int = 1024;
// Name indexes

pub const F2FS_XATTR_INDEX_USER: c_int = 1;
pub const F2FS_XATTR_INDEX_POSIX_ACL_ACCESS: c_int = 2;
pub const F2FS_XATTR_INDEX_POSIX_ACL_DEFAULT: c_int = 3;
pub const F2FS_XATTR_INDEX_TRUSTED: c_int = 4;
pub const F2FS_XATTR_INDEX_LUSTRE: c_int = 5;
pub const F2FS_XATTR_INDEX_SECURITY: c_int = 6;
pub const F2FS_XATTR_INDEX_ADVISE: c_int = 7;
// Should be same as EXT4_XATTR_INDEX_ENCRYPTION
pub const F2FS_XATTR_INDEX_ENCRYPTION: c_int = 9;
pub const F2FS_XATTR_INDEX_VERITY: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_xattr_header {
    pub /: *mut *mut __le32 h_magic; / magic number for identification,
    pub /: *mut *mut __le32 h_refcount; / reference count,
    pub /: *mut *mut __u32 h_reserved[4]; / zero right now,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_xattr_entry {
    pub e_name_index: __u8,
    pub e_name_len: __u8,
    pub /: *mut *mut __le16 e_value_size; / size of attribute value,
    pub /: *mut *mut char e_name[]; / attribute name,
}

//
// On-disk structure of f2fs_xattr
// We use inline xattrs space + 1 block for xattr.
//
// +--------------------+
// | f2fs_xattr_header  |
// |                    |
// +--------------------+
// | f2fs_xattr_entry   |
// | .e_name_index = 1  |
// | .e_name_len = 3    |
// | .e_value_size = 14 |
// | .e_name = "foo"    |
// | "value_of_xattr"   |<- value_offs = e_name + e_name_len
// +--------------------+
// | f2fs_xattr_entry   |
// | .e_name_index = 4  |
// | .e_name = "bar"    |
// +--------------------+
// |                    |
// |        Free        |
// |                    |
// +--------------------+<- MIN_OFFSET
// |   node_footer      |
// | (nid, ino, offset) |
// +--------------------+
//

extern "C" {
    pub fn f2fs_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn f2fs_init_xattr_cache() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_xattr_cache();
}

