//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_log_recover.h
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
// Copyright (c) 2000,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Each log item type (XFS_LI_*) gets its own xlog_recover_item_ops to
// define how recovery should work for that type of log item.
//
// Sorting hat for log items as they're read in.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xlog_recover_reorder {
    XLOG_REORDER_BUFFER_LIST,
    XLOG_REORDER_ITEM_LIST,
    XLOG_REORDER_INODE_BUFFER_LIST,
    XLOG_REORDER_CANCEL_LIST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_recover_item_ops {
    pub /: *mut *mut *mut uint16_t item_type; / XFS_LI_ type code.,
//
// Help sort recovered log items into the order required to replay them
// correctly.  Log item types that always use XLOG_REORDER_ITEM_LIST do
// not have to supply a function here.  See the comment preceding
// xlog_recover_reorder_trans for more details about what the return
// values mean.
//
    pub item): *mut *mut xlog_recover_reorder (reorder)(struct xlog_recover_item,
// Start readahead for pass2, if provided.
    pub item): *mut *mut *mut void (ra_pass2)(struct xlog log, struct xlog_recover_item,
// Do whatever work we need to do for pass1, if provided.
    pub item): *mut *mut *mut int (commit_pass1)(struct xlog log, struct xlog_recover_item,
//
// This function should do whatever work is needed for pass2 of log
// recovery, if provided.
//
// If the recovered item is an intent item, this function should parse
// the recovered item to construct an in-core log intent item and
// insert it into the AIL.  The in-core log intent item should have 1
// refcount so that the item is freed either (a) when we commit the
// recovered log item for the intent-done item; (b) replay the work and
// log a new intent-done item; or (c) recovery fails and we have to
// abort.
//
// If the recovered item is an intent-done item, this function should
// parse the recovered item to find the id of the corresponding intent
// log item.  Next, it should find the in-core log intent item in the
// AIL and release it.
//
    pub lsn): *mut *mut xlog_recover_item item, xfs_lsn_t,
}

//
// Macros, structures, prototypes for internal log manager use.
//
pub const XLOG_RHASH_BITS: c_int = 4;
pub const XLOG_RHASH_SIZE: c_int = 16;
pub const XLOG_RHASH_SHIFT: c_int = 2;

//
// item headers are in ri_buf[0].  Additional buffers follow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_recover_item {
    pub ri_list: list_head,
    pub /: *mut *mut int ri_cnt; / count of regions found,
    pub /: *mut *mut int ri_total; / total regions,
    pub /: *mut *mut *mut kvec ri_buf; / ptr to regions buffer,
    pub ri_ops: *const xlog_recover_item_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_recover {
    pub r_list: hlist_node,
    pub /: *mut *mut xlog_tid_t r_log_tid; / log's transaction id,
    pub /: *mut *mut xfs_trans_header r_theader; / trans header for partial,
    pub /: *mut *mut int r_state; / not needed,
    pub /: *mut *mut xfs_lsn_t r_lsn; / xact lsn,
    pub /: *mut *mut list_head r_itemq; / q for items,
}

pub const XLOG_RECOVER_CRCPASS: c_int = 0;
pub const XLOG_RECOVER_PASS1: c_int = 1;
pub const XLOG_RECOVER_PASS2: c_int = 2;
extern "C" {
    pub fn xlog_is_buffer_cancelled(log: *mut xlog, blkno: xfs_daddr_t, len: c_uint) -> bool;
}
extern "C" {
    pub fn xlog_alloc_buf_cancel_table(log: *mut xlog) -> c_int;
}
extern "C" {
    pub fn xlog_free_buf_cancel_table(log: *mut xlog);
}

extern "C" {
    pub fn xlog_check_buf_cancel_table(log: *mut xlog);
}

//
// Transform a regular reservation into one suitable for recovery of a log
// intent item.
//
// Intent recovery only runs a single step of the transaction chain and defers
// the rest to a separate transaction.  Therefore, we reduce logcount to 1 here
// to avoid livelocks if the log grant space is nearly exhausted due to the
// recovered intent pinning the tail.  Keep the same logflags to avoid tripping
// asserts elsewhere.  Struct copies abound below.
//
