//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext2/xattr.h
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

// Magic value in attribute blocks
pub const EXT2_XATTR_MAGIC: c_uint = 0xEA020000;
// Maximum number of references to one attribute block
pub const EXT2_XATTR_REFCOUNT_MAX: c_int = 1024;
// Name indexes
pub const EXT2_XATTR_INDEX_USER: c_int = 1;
pub const EXT2_XATTR_INDEX_POSIX_ACL_ACCESS: c_int = 2;
pub const EXT2_XATTR_INDEX_POSIX_ACL_DEFAULT: c_int = 3;
pub const EXT2_XATTR_INDEX_TRUSTED: c_int = 4;
pub const EXT2_XATTR_INDEX_LUSTRE: c_int = 5;
pub const EXT2_XATTR_INDEX_SECURITY: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_xattr_header {
    pub /: *mut *mut __le32 h_magic; / magic number for identification,
    pub /: *mut *mut __le32 h_refcount; / reference count,
    pub /: *mut *mut __le32 h_blocks; / number of disk blocks used,
    pub /: *mut *mut __le32 h_hash; / hash value of all attributes,
    pub /: *mut *mut __u32 h_reserved[4]; / zero right now,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext2_xattr_entry {
    pub /: *mut *mut __u8 e_name_len; / length of name,
    pub /: *mut *mut __u8 e_name_index; / attribute name index,
    pub /: *mut *mut __le16 e_value_offs; / offset in disk block of value,
    pub /: *mut *mut __le32 e_value_block; / disk block attribute is stored on (n/i),
    pub /: *mut *mut __le32 e_value_size; / size of attribute value,
    pub /: *mut *mut __le32 e_hash; / hash value of name and value,
    pub /: *mut *mut char e_name[]; / attribute name,
}

pub const EXT2_XATTR_PAD_BITS: c_int = 2;

extern "C" {
    pub fn ext2_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn ext2_xattr_get(: *mut inode, _arg: c_int, : *const c_char, : *mut c_void, _arg: usize) -> c_int;
}
extern "C" {
    pub fn ext2_xattr_set(: *mut inode, _arg: c_int, : *const c_char, : *const c_void, _arg: usize, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ext2_xattr_delete_inode(: *mut inode);
}
extern "C" {
    pub fn ext2_xattr_destroy_cache(cache: *mut mb_cache);
}

