//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_inode.h
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
// Copyright (C) International Business Machines Corp., 2000-2001
//
extern "C" {
    pub fn jfs_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn jfs_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
extern "C" {
    pub fn jfs_commit_inode(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jfs_write_inode(: *mut inode, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn jfs_evict_inode(: *mut inode);
}
extern "C" {
    pub fn jfs_dirty_inode(: *mut inode, _arg: c_int);
}
extern "C" {
    pub fn jfs_truncate(: *mut inode);
}
extern "C" {
    pub fn jfs_truncate_nolock(: *mut inode, _arg: loff_t);
}
extern "C" {
    pub fn jfs_free_zero_link(: *mut inode);
}
extern "C" {
    pub fn jfs_set_inode_flags(: *mut inode);
}
extern "C" {
    pub fn jfs_get_block(: *mut inode, _arg: sector_t, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn jfs_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
