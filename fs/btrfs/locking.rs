//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/locking.h
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
// Copyright (C) 2008 Oracle.  All rights reserved.
//

pub const BTRFS_WRITE_LOCK: c_int = 1;
pub const BTRFS_READ_LOCK: c_int = 2;
//
// We are limited in number of subclasses by MAX_LOCKDEP_SUBCLASSES, which at
// the time of this patch is 8, which is how many we use.  Keep this in mind if
// you decide you want to add another subclass.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_lock_nesting {
    BTRFS_NESTING_NORMAL,

//
// When we COW a block we are holding the lock on the original block,
// and since our lockdep maps are rootid+level, this confuses lockdep
// when we lock the newly allocated COW'd block.  Handle this by having
// a subclass for COW'ed blocks so that lockdep doesn't complain.
//
    BTRFS_NESTING_COW,

//
// Oftentimes we need to lock adjacent nodes on the same level while
// still holding the lock on the original node we searched to, such as
// for searching forward or for split/balance.
//
// Because of this we need to indicate to lockdep that this is
// acceptable by having a different subclass for each of these
// operations.
//
    BTRFS_NESTING_LEFT,
    BTRFS_NESTING_RIGHT,

//
// When splitting we will be holding a lock on the left/right node when
// we need to cow that node, thus we need a new set of subclasses for
// these two operations.
//
    BTRFS_NESTING_LEFT_COW,
    BTRFS_NESTING_RIGHT_COW,

//
// When splitting we may push nodes to the left or right, but still use
// the subsequent nodes in our path, keeping our locks on those adjacent
// blocks.  Thus when we go to allocate a new split block we've already
// used up all of our available subclasses, so this subclass exists to
// handle this case where we need to allocate a new split block.
//
    BTRFS_NESTING_SPLIT,

//
// When promoting a new block to a root we need to have a special
// subclass so we don't confuse lockdep, as it will appear that we are
// locking a higher level node before a lower level one.  Copying also
// has this problem as it appears we're locking the same block again
// when we make a snapshot of an existing root.
//
    BTRFS_NESTING_NEW_ROOT,

//
// We are limited to MAX_LOCKDEP_SUBCLASSES number of subclasses, so
// add this in here and add a static_assert to keep us from going over
// the limit.  As of this writing we're limited to 8, and we're
// definitely using 8, hence this check to keep us from messing up in
// the future.
//
    BTRFS_NESTING_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_lockdep_trans_states {
    BTRFS_LOCKDEP_TRANS_COMMIT_PREP,
    BTRFS_LOCKDEP_TRANS_UNBLOCKED,
    BTRFS_LOCKDEP_TRANS_SUPER_COMMITTED,
    BTRFS_LOCKDEP_TRANS_COMPLETED,
}

//
// Lockdep annotation for wait events.
//
// @owner:  The struct where the lockdep map is defined
// @lock:   The lockdep map corresponding to a wait event
//
// This macro is used to annotate a wait event. In this case a thread acquires
// the lockdep map as writer (exclusive lock) because it has to block until all
// the threads that hold the lock as readers signal the condition for the wait
// event and release their locks.
//

//
// Protection for the resource/condition of a wait event.
//
// @owner:  The struct where the lockdep map is defined
// @lock:   The lockdep map corresponding to a wait event
//
// Many threads can modify the condition for the wait event at the same time
// and signal the threads that block on the wait event. The threads that modify
// the condition and do the signaling acquire the lock as readers (shared
// lock).
//

//
// Used after signaling the condition for a wait event to release the lockdep
// map held by a reader thread.
//

//
// Used to account for the fact that when doing io_uring encoded I/O, we can
// return to userspace with the inode lock still held.
//

//
// Macros for the transaction states wait events, similar to the generic wait
// event macros.
//

// Initialization of the lockdep map

// Initialization of the transaction states lockdep maps.

extern "C" {
    pub fn btrfs_tree_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting);
}
extern "C" {
    pub fn btrfs_tree_unlock(eb: *mut extent_buffer);
}
extern "C" {
    pub fn btrfs_tree_read_lock_nested(eb: *mut extent_buffer, nest: btrfs_lock_nesting);
}
extern "C" {
    pub fn btrfs_tree_read_unlock(eb: *mut extent_buffer);
}
extern "C" {
    pub fn btrfs_try_tree_read_lock(eb: *mut extent_buffer) -> bool;
}

extern "C" {
    pub fn btrfs_unlock_up_safe(path: *mut btrfs_path, level: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_drew_lock {
    pub readers: core::sync::atomic::AtomicI32,
    pub writers: core::sync::atomic::AtomicI32,
    pub pending_writers: wait_queue_head_t,
    pub pending_readers: wait_queue_head_t,
}

extern "C" {
    pub fn btrfs_drew_lock_init(lock: *mut btrfs_drew_lock);
}
extern "C" {
    pub fn btrfs_drew_write_lock(lock: *mut btrfs_drew_lock);
}
extern "C" {
    pub fn btrfs_drew_try_write_lock(lock: *mut btrfs_drew_lock) -> bool;
}
extern "C" {
    pub fn btrfs_drew_write_unlock(lock: *mut btrfs_drew_lock);
}
extern "C" {
    pub fn btrfs_drew_read_lock(lock: *mut btrfs_drew_lock);
}
extern "C" {
    pub fn btrfs_drew_read_unlock(lock: *mut btrfs_drew_lock);
}

extern "C" {
    pub fn btrfs_set_buffer_lockdep_class(objectid: u64, eb: *mut extent_buffer, level: c_int);
}
extern "C" {
    pub fn btrfs_maybe_reset_lockdep_class(root: *mut btrfs_root, eb: *mut extent_buffer);
}

