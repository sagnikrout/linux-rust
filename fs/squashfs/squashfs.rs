//! Automatically rewritten from C Header to Rust Module
//! Source: fs/squashfs/squashfs.h
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
// Squashfs - a compressed read only filesystem for Linux
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
// Phillip Lougher <phillip@squashfs.org.uk>
//
// squashfs.h
//

pub const SQUASHFS_READ_PAGES: c_int = 0;

// block.c
// cache.c
extern "C" {
    pub fn squashfs_cache_delete(: *mut squashfs_cache);
}
extern "C" {
    pub fn squashfs_cache_put(: *mut squashfs_cache_entry);
}
extern "C" {
    pub fn squashfs_copy_data(: *mut c_void, : *mut squashfs_cache_entry, _arg: c_int, _arg: c_int) -> c_int;
}
// decompressor.c
// decompressor_xxx.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_decompressor_thread_ops {
    pub comp_opts): *mut *mut *mut *mut void  (create)(struct squashfs_sb_info msblk, void,
    pub msblk): *mut *mut void (destroy)(struct squashfs_sb_info,
    pub output): *mut int offset, int length, struct squashfs_page_actor,
    pub (*max_decompressors)(void): *mut c_int,
}

// export.c
// fragment.c
extern "C" {
    pub fn squashfs_frag_lookup(: *mut super_block, int: unsigned, : *mut u64) -> c_int;
}
// file.c
// file_xxx.c
extern "C" {
    pub fn squashfs_readpage_block(: *mut folio, block: u64, bsize: c_int, expected: c_int) -> c_int;
}
// id.c
extern "C" {
    pub fn squashfs_get_id(: *mut super_block, int: unsigned, : *mut c_uint) -> c_int;
}
// inode.c
extern "C" {
    pub fn squashfs_read_inode(: *mut inode, long: c_long) -> c_int;
}
// xattr.c
extern "C" {
    pub fn squashfs_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
//
// Inodes, files,  decompressor and xattr operations
//
// dir.c
// export.c
// file.c
// inode.c
// namei.c
// symlink.c
// xattr.c
