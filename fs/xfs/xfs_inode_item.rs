//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_inode_item.h
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
// kernel only definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inode_log_item {
    pub /: *mut *mut xfs_log_item ili_item; / common portion,
    pub /: *mut *mut *mut xfs_inode ili_inode; / inode ptr,
    pub /: *mut *mut unsigned short ili_lock_flags; / inode lock flags,
    pub /: *mut *mut unsigned int ili_dirty_flags; / dirty in current tx,
//
// The ili_lock protects the interactions between the dirty state and
// the flush state of the inode log item. This allows us to do atomic
// modifications of multiple state fields without having to hold a
// specific inode lock to serialise them.
//
// We need atomic changes between inode dirtying, inode flushing and
// inode completion, but these all hold different combinations of
// ILOCK and IFLUSHING and hence we need some other method of
// serialising updates to the flush state.
//
    pub /: *mut *mut spinlock_t ili_lock; / flush state lock,
    pub /: *mut *mut unsigned int ili_last_fields; / fields when flushed,
    pub /: *mut *mut unsigned int ili_fields; / fields to be logged,
    pub /: *mut *mut xfs_lsn_t ili_flush_lsn; / lsn at last flush,
//
// We record the sequence number for every inode modification, as
// well as those that only require fdatasync operations for data
// integrity. This allows optimisation of the O_DSYNC/fdatasync path
// without needing to track what modifications the journal is currently
// carrying for the inode. These are protected by the above ili_lock.
//
    pub /: *mut *mut xfs_csn_t ili_commit_seq; / last transaction commit,
    pub /: *mut *mut xfs_csn_t ili_datasync_seq; / for datasync optimisation,
}

extern "C" {
    pub fn xfs_inode_item_init(: *mut xfs_inode, : *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inode_item_destroy(: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_iflush_abort(: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_iflush_shutdown_abort(: *mut xfs_inode);
}
