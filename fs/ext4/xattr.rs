//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/xattr.h
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
pub const EXT4_XATTR_MAGIC: c_uint = 0xEA020000;
// Maximum number of references to one attribute block
pub const EXT4_XATTR_REFCOUNT_MAX: c_int = 1024;
// Name indexes
pub const EXT4_XATTR_INDEX_USER: c_int = 1;
pub const EXT4_XATTR_INDEX_POSIX_ACL_ACCESS: c_int = 2;
pub const EXT4_XATTR_INDEX_POSIX_ACL_DEFAULT: c_int = 3;
pub const EXT4_XATTR_INDEX_TRUSTED: c_int = 4;
pub const EXT4_XATTR_INDEX_LUSTRE: c_int = 5;
pub const EXT4_XATTR_INDEX_SECURITY: c_int = 6;
pub const EXT4_XATTR_INDEX_SYSTEM: c_int = 7;
pub const EXT4_XATTR_INDEX_RICHACL: c_int = 8;
pub const EXT4_XATTR_INDEX_ENCRYPTION: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_header {
    pub /: *mut *mut __le32 h_magic; / magic number for identification,
    pub /: *mut *mut __le32 h_refcount; / reference count,
    pub /: *mut *mut __le32 h_blocks; / number of disk blocks used,
    pub /: *mut *mut __le32 h_hash; / hash value of all attributes,
    pub /: *mut *mut __le32 h_checksum; / crc32c(uuid+blknum+xattrblock),
    pub /: *mut *mut __u32 h_reserved[3]; / zero right now,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_ibody_header {
    pub /: *mut *mut __le32 h_magic; / magic number for identification,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_entry {
    pub /: *mut *mut __u8 e_name_len; / length of name,
    pub /: *mut *mut __u8 e_name_index; / attribute name index,
    pub /: *mut *mut __le16 e_value_offs; / offset in disk block of value,
    pub /: *mut *mut __le32 e_value_inum; / inode in which the value is stored,
    pub /: *mut *mut __le32 e_value_size; / size of attribute value,
    pub /: *mut *mut __le32 e_hash; / hash value of name and value,
    pub /: *mut *mut char e_name[]; / attribute name,
}

pub const EXT4_XATTR_PAD_BITS: c_int = 2;

//
// XATTR_SIZE_MAX is currently 64k, but for the purposes of checking
// for file system consistency errors, we use a somewhat bigger value.
// This allows XATTR_SIZE_MAX to grow in the future, but by using this
// instead of INT_MAX for certain consistency checks, we don't need to
// worry about arithmetic overflows.  (Actually XATTR_SIZE_MAX is
// defined in include/uapi/linux/limits.h, so changing it is going
// not going to be trivial....)
//

//
// The minimum size of EA value when you start storing it in an external inode
// size of block - size of header - size of 1 entry - 4 null bytes
//

//
// If we want to add an xattr to the inode, we should make sure that
// i_extra_isize is not 0 and that the inode size is not less than
// EXT4_GOOD_OLD_INODE_SIZE + extra_isize + pad.
// EXT4_GOOD_OLD_INODE_SIZE   extra_isize header   entry   pad  data
// |--------------------------|------------|------|---------|---|-------|
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_info {
    pub name: *const c_char,
    pub value: *const c_void,
    pub value_len: usize,
    pub name_index: c_int,
    pub in_inode: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_search {
    pub first: *mut ext4_xattr_entry,
    pub base: *mut c_void,
    pub end: *mut c_void,
    pub here: *mut ext4_xattr_entry,
    pub not_found: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext4_xattr_ibody_find {
    pub s: ext4_xattr_search,
    pub iloc: ext4_iloc,
}

//
// The EXT4_STATE_NO_EXPAND is overloaded and used for two purposes.
// The first is to signal that there the inline xattrs and data are
// taking up so much space that we might as well not keep trying to
// expand it.  The second is that xattr_sem is taken for writing, so
// we shouldn't try to recurse into the inode expansion.  For this
// second case, we need to make sure that we take save and restore the
// NO_EXPAND state flag appropriately.
//
// save = ext4_test_inode_state(inode, EXT4_STATE_NO_EXPAND);
extern "C" {
    pub fn ext4_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn ext4_xattr_get(: *mut inode, _arg: c_int, : *const c_char, : *mut c_void, _arg: usize) -> c_int;
}
extern "C" {
    pub fn ext4_xattr_set(: *mut inode, _arg: c_int, : *const c_char, : *const c_void, _arg: usize, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_xattr_set_handle(: *mut handle_t, : *mut inode, _arg: c_int, : *const c_char, : *const c_void, _arg: usize, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ext4_init_ea_inode_work(sbi: *mut ext4_sb_info);
}
extern "C" {
    pub fn ext4_put_ea_inode(inode: *mut inode);
}
extern "C" {
    pub fn ext4_evict_ea_inode(inode: *mut inode);
}
extern "C" {
    pub fn ext4_xattr_destroy_cache(: *mut mb_cache);
}

extern "C" {
    pub fn ext4_xattr_inode_set_class(ea_inode: *mut inode);
}

extern "C" {
    pub fn ext4_get_inode_usage(inode: *mut inode, usage: *mut qsize_t) -> c_int;
}
