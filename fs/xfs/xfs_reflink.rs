//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_reflink.h
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
pub const __XFS_REFLINK_H: c_int = 1;
//
// Check whether it is safe to free COW fork blocks from an inode. It is unsafe
// to do so when an inode has dirty cache or I/O in-flight, even if no shared
// extents exist in the data fork, because outstanding I/O may target blocks
// that were speculatively allocated to the COW fork.
//
extern "C" {
    pub fn xfs_reflink_recover_cow(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_reflink_supports_rextsize(mp: *mut xfs_mount, rextsize: c_uint) -> bool;
}
extern "C" {
    pub fn xfs_reflink_max_atomic_cow(mp: *mut xfs_mount) -> xfs_extlen_t;
}
