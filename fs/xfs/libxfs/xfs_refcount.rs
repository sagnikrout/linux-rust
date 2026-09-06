//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_refcount.h
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
// low level btree operations need to handle the generic btree range
// query functions (which set rc_domain == -1U), so we check that the
// domain is /not/ shared.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_refcount_intent_type {
    XFS_REFCOUNT_INCREASE = 1,
    XFS_REFCOUNT_DECREASE,
    XFS_REFCOUNT_ALLOC_COW,
    XFS_REFCOUNT_FREE_COW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_refcount_intent {
    pub ri_list: list_head,
    pub ri_group: *mut xfs_group,
    pub ri_type: xfs_refcount_intent_type,
    pub ri_blockcount: xfs_extlen_t,
    pub ri_startblock: xfs_fsblock_t,
    pub ri_realtime: bool,
}

// Check that the refcount is appropriate for the record domain.
extern "C" {
    pub fn xfs_refcount_recover_cow_leftovers(xg: *mut xfs_group) -> c_int;
}
//
// While we're adjusting the refcounts records of an extent, we have
// to keep an eye on the number of extents we're dirtying -- run too
// many in a single transaction and we'll exceed the transaction's
// reservation and crash the fs.  Each record adds 12 bytes to the
// log (plus any key updates) so we'll conservatively assume 32 bytes
// per record.  We must also leave space for btree splits on both ends
// of the range and space for the CUD and a new CUI.
//
// Each EFI that we attach to the transaction is assumed to consume ~32 bytes.
// This is a low estimate for an EFI tracking a single extent (16 bytes for the
// EFI header, 16 for the extent, and 12 for the xlog op header), but the
// estimate is acceptable if there's more than one extent being freed.
// In the worst case of freeing every other block during a refcount decrease
// operation, we amortize the space used for one EFI log item across 16
// extents.
//
pub const XFS_REFCOUNT_ITEM_OVERHEAD: c_int = 32;
extern "C" {
    pub fn xfs_refcount_intent_init_cache() -> int __init;
}
extern "C" {
    pub fn xfs_refcount_intent_destroy_cache();
}
