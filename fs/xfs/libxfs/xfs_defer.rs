//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_defer.h
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
// Save a log intent item and a list of extents, so that we can replay
// whatever action had to happen to the extent list and file the log done
// item.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_pending {
    pub /: *mut *mut list_head dfp_list; / pending items,
    pub /: *mut *mut list_head dfp_work; / work items,
    pub /: *mut *mut *mut xfs_log_item dfp_intent; / log intent item,
    pub /: *mut *mut *mut xfs_log_item dfp_done; / log done item,
    pub dfp_ops: *const xfs_defer_op_type,
    pub /: *mut *mut unsigned int dfp_count; / # extent items,
    pub dfp_flags: c_uint,
}

//
// Create a log intent item for this deferred item, but don't actually finish
// the work.  Caller must clear this before the final transaction commit.
//

extern "C" {
    pub fn xfs_defer_item_pause(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending);
}
extern "C" {
    pub fn xfs_defer_item_unpause(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending);
}
extern "C" {
    pub fn xfs_defer_finish_noroll(tp: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfs_defer_finish(tp: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfs_defer_finish_one(tp: *mut xfs_trans, dfp: *mut xfs_defer_pending) -> c_int;
}
extern "C" {
    pub fn xfs_defer_cancel(: *mut xfs_trans);
}
extern "C" {
    pub fn xfs_defer_move(dtp: *mut xfs_trans, stp: *mut xfs_trans);
}
// Description of a deferred type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_op_type {
    pub name: *const c_char,
    pub max_items: c_uint,
    pub sort): *mut *mut list_head items, unsigned int count, bool,
    pub intent): *mut *mut void (abort_intent)(struct xfs_log_item,
    pub count): *mut *mut xfs_log_item intent, unsigned int,
    pub state): *mut *mut list_head item, xfs_btree_cur,
    pub error): *mut *mut xfs_btree_cur state, int,
    pub item): *mut *mut void (cancel_item)(struct list_head,
    pub capture_list): *mut list_head,
    pub done_item): *mut xfs_log_item,
}

//
// Deferred operation item relogging limits.
//
// Rename w/ parent pointers can require up to 5 inodes with deferred ops to
// be joined to the transaction: src_dp, target_dp, src_ip, target_ip, and wip.
// These inodes are locked in sorted order by their inode numbers
//
pub const XFS_DEFER_OPS_NR_INODES: c_int = 5;

// Resources that must be held across a transaction roll.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_resources {
// held buffers
    pub dr_bp: [*mut xfs_buf; XFS_DEFER_OPS_NR_BUFS],
// inodes with no unlock flags
    pub dr_ip: [*mut xfs_inode; XFS_DEFER_OPS_NR_INODES],
// number of held buffers
    pub dr_bufs: c_ushort,
// bitmap of ordered buffers
    pub dr_ordered: c_ushort,
// number of held inodes
    pub dr_inos: c_ushort,
}

//
// This structure enables a dfops user to detach the chain of deferred
// operations from a transaction so that they can be continued later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_defer_capture {
// List of other capture structures.
    pub dfc_list: list_head,
// Deferred ops state saved from the transaction.
    pub dfc_dfops: list_head,
    pub dfc_tpflags: c_uint,
// Block reservations for the data and rt devices.
    pub dfc_blkres: c_uint,
    pub dfc_rtxres: c_uint,
// Log reservation saved from the transaction.
    pub dfc_logres: c_uint,
    pub dfc_held: xfs_defer_resources,
}

//
// Functions to capture a chain of deferred operations and continue them later.
// This doesn't normally happen except log recovery.
//
extern "C" {
    pub fn xfs_defer_resources_rele(dres: *mut xfs_defer_resources);
}
extern "C" {
    pub fn xfs_defer_init_item_caches() -> int __init;
}
extern "C" {
    pub fn xfs_defer_destroy_item_caches();
}
extern "C" {
    pub fn xfs_defer_add_barrier(tp: *mut xfs_trans);
}
