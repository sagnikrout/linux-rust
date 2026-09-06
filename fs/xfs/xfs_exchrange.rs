//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_exchrange.h
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
// Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Update the mtime/cmtime of file1 and file2

// Freshness check required

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_exchrange {
    pub file1: *mut file,
    pub file2: *mut file,
    pub file1_offset: loff_t,
    pub file2_offset: loff_t,
    pub length: u64,
    pub /: *mut *mut u64 flags; / XFS_EXCHANGE_RANGE flags,
// file2 metadata for freshness checks
    pub file2_ino: u64,
    pub file2_mtime: timespec64,
    pub file2_ctime: timespec64,
    pub file2_gen: u32,
}

extern "C" {
    pub fn xfs_exchrange_iunlock(ip1: *mut xfs_inode, ip2: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_exchrange_estimate(req: *mut xfs_exchmaps_req) -> c_int;
}
