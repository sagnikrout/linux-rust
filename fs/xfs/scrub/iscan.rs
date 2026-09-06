//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/iscan.h
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
// Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_iscan {
    pub sc: *mut xfs_scrub,
// Lock to protect the scan cursor.
    pub lock: mutex,
//
// This is the first inode in the inumber address space that we
// examined.  When the scan wraps around back to here, the scan is
// finished.
//
    pub scan_start_ino: xfs_ino_t,
// This is the inode that will be examined next.
    pub cursor_ino: xfs_ino_t,
// If nonzero and non-NULL, skip this inode when scanning.
    pub skip_ino: xfs_ino_t,
//
// This is the last inode that we've successfully scanned, either
// because the caller scanned it, or we moved the cursor past an empty
// part of the inode address space.  Scan callers should only use the
// xchk_iscan_visit function to modify this.
//
    pub __visited_ino: xfs_ino_t,
// Operational state of the livescan.
    pub __opstate: c_ulong,
// Give up on iterating @cursor_ino if we can't iget it by this time.
    pub __iget_deadline: c_ulong,
// Amount of time (in ms) that we will try to iget an inode.
    pub iget_timeout: c_uint,
// Wait this many ms to retry an iget.
    pub iget_retry_delay: c_uint,
//
// The scan grabs batches of inodes and stashes them here before
// handing them out with _iter.  Unallocated inodes are set in the
// mask so that all updates to that inode are selected for live
// update propagation.
//
    pub __batch_ino: xfs_ino_t,
    pub __skipped_inomask: xfs_inofree_t,
    pub __inodes: [*mut xfs_inode; XFS_INODES_PER_CHUNK],
}

// Set if the scan has been aborted due to some event in the fs.

// Use trylock to acquire the AGI

extern "C" {
    pub fn test_bit(_arg: XCHK_ISCAN_OPSTATE_ABORTED, _arg: &iscan->__opstate) -> return;
}
extern "C" {
    pub fn test_bit(_arg: XCHK_ISCAN_OPSTATE_TRYLOCK_AGI, _arg: &iscan->__opstate) -> return;
}
extern "C" {
    pub fn xchk_iscan_finish_early(iscan: *mut xchk_iscan);
}
extern "C" {
    pub fn xchk_iscan_teardown(iscan: *mut xchk_iscan);
}
extern "C" {
    pub fn xchk_iscan_iter(iscan: *mut xchk_iscan, ipp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xchk_iscan_iter_finish(iscan: *mut xchk_iscan);
}
extern "C" {
    pub fn xchk_iscan_mark_visited(iscan: *mut xchk_iscan, ip: *mut xfs_inode);
}
extern "C" {
    pub fn xchk_iscan_want_live_update(iscan: *mut xchk_iscan, ino: xfs_ino_t) -> bool;
}
