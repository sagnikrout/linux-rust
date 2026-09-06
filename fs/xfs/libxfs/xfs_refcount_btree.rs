//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_refcount_btree.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
// Reference Count Btree on-disk structures
//
// Btree block header size
//

//
// Record, key, and pointer address macros for btree blocks.
//
// (note that some of these may appear unused, but they are used in userspace)
//

extern "C" {
    pub fn xfs_refcountbt_compute_maxlevels(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_refcountbt_maxlevels_ondisk() -> c_uint;
}
extern "C" {
    pub fn xfs_refcountbt_init_cur_cache() -> int __init;
}
extern "C" {
    pub fn xfs_refcountbt_destroy_cur_cache();
}
