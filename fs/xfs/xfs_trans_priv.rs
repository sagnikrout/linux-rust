//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_trans_priv.h
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
// Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
extern "C" {
    pub fn xfs_trans_init(: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_trans_add_item(: *mut xfs_trans, : *mut xfs_log_item);
}
extern "C" {
    pub fn xfs_trans_del_item(: *mut xfs_log_item);
}
extern "C" {
    pub fn xfs_trans_unreserve_and_mod_sb(tp: *mut xfs_trans);
}
//
// AIL traversal cursor.
//
// Rather than using a generation number for detecting changes in the ail, use
// a cursor that is protected by the ail lock. The aild cursor exists in the
// struct xfs_ail, but other traversals can declare it on the stack and link it
// to the ail list.
//
// When an object is deleted from or moved int the AIL, the cursor list is
// searched to see if the object is a designated cursor item. If it is, it is
// deleted from the cursor so that the next time the cursor is used traversal
// will return to the start.
//
// This means a traversal colliding with a removal will cause a restart of the
// list scan, rather than any insertion or deletion anywhere in the list. The
// low bit of the item pointer is set if the cursor has been invalidated so
// that we can tell the difference between invalidation and reaching the end
// of the list to trigger traversal restarts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ail_cursor {
    pub list: list_head,
    pub item: *mut xfs_log_item,
}

//
// Private AIL structures.
//
// Eventually we need to drive the locking in here as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_ail {
    pub ail_log: *mut xlog,
    pub ail_task: *mut task_struct,
    pub ail_head: list_head,
    pub ail_cursors: list_head,
    pub ail_lock: spinlock_t,
    pub ail_last_pushed_lsn: xfs_lsn_t,
    pub ail_head_lsn: xfs_lsn_t,
    pub ail_log_flush: c_int,
    pub ail_opstate: c_ulong,
    pub ail_buf_list: list_head,
    pub ail_empty: wait_queue_head_t,
    pub ail_target: xfs_lsn_t,
}

// Push all items out of the AIL immediately.

//
// From xfs_trans_ail.c
//
// Return a pointer to the first item in the AIL.  If the AIL is empty, then
// return NULL.
//
extern "C" {
    pub fn xfs_ail_delete_one(ailp: *mut xfs_ail, lip: *mut xfs_log_item) -> xfs_lsn_t;
}
extern "C" {
    pub fn xfs_trans_ail_delete(lip: *mut xfs_log_item, shutdown_type: c_int);
}
extern "C" {
    pub fn READ_ONCE(_arg: ailp->ail_target) -> return;
}
extern "C" {
    pub fn xfs_ail_push_all_sync(ailp: *mut xfs_ail);
}
extern "C" {
    pub fn xfs_ail_min_lsn(ailp: *mut xfs_ail) -> xfs_lsn_t;
}
extern "C" {
    pub fn xfs_trans_ail_cursor_done(cur: *mut xfs_ail_cursor);
}
extern "C" {
    pub fn __xfs_ail_assign_tail_lsn(ailp: *mut xfs_ail);
}

// dst = *src;

// dst = *src;

