//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_exchmaps.h
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
// In-core deferred operation info about a file mapping exchange request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_exchmaps_intent {
// List of other incore deferred work.
    pub xmi_list: list_head,
// Inodes participating in the operation.
    pub xmi_ip1: *mut xfs_inode,
    pub xmi_ip2: *mut xfs_inode,
// File offset range information.
    pub xmi_startoff1: xfs_fileoff_t,
    pub xmi_startoff2: xfs_fileoff_t,
    pub xmi_blockcount: xfs_filblks_t,
// Set these file sizes after the operation, unless negative.
    pub xmi_isize1: xfs_fsize_t,
    pub xmi_isize2: xfs_fsize_t,
    pub /: *mut *mut *mut uint64_t xmi_flags; / XFS_EXCHMAPS_ flags,
}

// Try to convert inode2 from block to short format at the end, if possible.

// flags that can be passed to xfs_exchmaps_{estimate,mappings}

// Parameters for a mapping exchange request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_exchmaps_req {
// Inodes participating in the operation.
    pub ip1: *mut xfs_inode,
    pub ip2: *mut xfs_inode,
// File offset range information.
    pub startoff1: xfs_fileoff_t,
    pub startoff2: xfs_fileoff_t,
    pub blockcount: xfs_filblks_t,
// XFS_EXCHMAPS_* operation flags
    pub flags: u64,
//
// Fields below this line are filled out by xfs_exchmaps_estimate;
// callers should initialize this part of the struct to zero.
//
// Data device blocks to be moved out of ip1, and free space needed to
// handle the bmbt changes.
//
    pub ip1_bcount: xfs_filblks_t,
//
// Data device blocks to be moved out of ip2, and free space needed to
// handle the bmbt changes.
//
    pub ip2_bcount: xfs_filblks_t,
// rt blocks to be moved out of ip1.
    pub ip1_rtbcount: xfs_filblks_t,
// rt blocks to be moved out of ip2.
    pub ip2_rtbcount: xfs_filblks_t,
// Free space needed to handle the bmbt changes
    pub resblks: c_ulonglong,
// Number of exchanges needed to complete the operation
    pub nr_exchanges: c_ulonglong,
}

extern "C" {
    pub fn xfs_exchmaps_estimate_overhead(req: *mut xfs_exchmaps_req) -> c_int;
}
extern "C" {
    pub fn xfs_exchmaps_estimate(req: *mut xfs_exchmaps_req) -> c_int;
}
extern "C" {
    pub fn xfs_exchmaps_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_exchmaps_intent_destroy_cache();
}
