//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/transaction.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//

//
// Signal that a direct IO write is in progress, to avoid deadlock for sync
// direct IO writes when fsync is called during the direct IO write path.
//

// Radix-tree tag for roots that are part of the transaction.
pub const BTRFS_ROOT_TRANS_TAG: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_trans_state {
    TRANS_STATE_RUNNING,
    TRANS_STATE_COMMIT_PREP,
    TRANS_STATE_COMMIT_START,
    TRANS_STATE_COMMIT_DOING,
    TRANS_STATE_UNBLOCKED,
    TRANS_STATE_SUPER_COMMITTED,
    TRANS_STATE_COMPLETED,
    TRANS_STATE_MAX,
}

pub const BTRFS_TRANS_HAVE_FREE_BGS: c_int = 0;
pub const BTRFS_TRANS_DIRTY_BG_RUN: c_int = 1;
pub const BTRFS_TRANS_CACHE_ENOSPC: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_transaction {
    pub transid: u64,
//
// total external writers(USERSPACE/START/ATTACH) in this
// transaction, it must be zero before the transaction is
// being committed
//
    pub num_extwriters: core::sync::atomic::AtomicI32,
//
// total writers in this transaction, it must be zero before the
// transaction can end
//
    pub num_writers: core::sync::atomic::AtomicI32,
    pub use_count: refcount_t,
    pub flags: c_ulong,
// Be protected by fs_info->trans_lock when we want to change it.
    pub state: btrfs_trans_state,
    pub aborted: c_int,
    pub list: list_head,
    pub dirty_pages: extent_io_tree,
    pub start_time: time64_t,
    pub writer_wait: wait_queue_head_t,
    pub commit_wait: wait_queue_head_t,
    pub pending_snapshots: list_head,
    pub dev_update_list: list_head,
    pub switch_commits: list_head,
    pub dirty_bgs: list_head,
//
// There is no explicit lock which protects io_bgs, rather its
// consistency is implied by the fact that all the sites which modify
// it do so under some form of transaction critical section, namely:
//
// - btrfs_start_dirty_block_groups - This function can only ever be
// run by one of the transaction committers. Refer to
// BTRFS_TRANS_DIRTY_BG_RUN usage in btrfs_commit_transaction
//
// - btrfs_write_dirty_blockgroups - this is called by
// commit_cowonly_roots from transaction critical section
// (TRANS_STATE_COMMIT_DOING)
//
// - btrfs_cleanup_dirty_bgs - called on transaction abort
//
    pub io_bgs: list_head,
    pub dropped_roots: list_head,
    pub pinned_extents: extent_io_tree,
//
// we need to make sure block group deletion doesn't race with
// free space cache writeout.  This mutex keeps them from stomping
// on each other
//
    pub cache_write_mutex: mutex,
    pub dirty_bgs_lock: spinlock_t,
// Protected by spin lock fs_info->unused_bgs_lock.
    pub deleted_bgs: list_head,
    pub dropped_roots_lock: spinlock_t,
    pub delayed_refs: btrfs_delayed_ref_root,
    pub fs_info: *mut btrfs_fs_info,
//
// Number of ordered extents the transaction must wait for before
// committing. These are ordered extents started by a fast fsync.
//
    pub pending_ordered: core::sync::atomic::AtomicI32,
    pub pending_wait: wait_queue_head_t,
}

//
// Number of extent buffers a transaction handle tracks for writeback
// inhibition. The CLOCK reference bits pack into a u32 so this must not exceed
// 32, and keeping it a power of two lets the compiler reduce the CLOCK hand
// modulo to a mask.
//
pub const BTRFS_INHIBITED_EBS_SLOTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_trans_handle {
    pub transid: u64,
    pub bytes_reserved: u64,
    pub delayed_refs_bytes_reserved: u64,
    pub chunk_bytes_reserved: u64,
    pub delayed_ref_updates: c_ulong,
    pub delayed_ref_csum_deletions: c_ulong,
    pub transaction: *mut btrfs_transaction,
    pub block_rsv: *mut btrfs_block_rsv,
    pub orig_rsv: *mut btrfs_block_rsv,
// Set by a task that wants to create a snapshot.
    pub pending_snapshot: *mut btrfs_pending_snapshot,
    pub use_count: refcount_t,
    pub type: c_uint,
//
// Error code of transaction abort, set outside of locks and must use
// the READ_ONCE/WRITE_ONCE access
//
    pub aborted: c_short,
    pub adding_csums: bool,
    pub allocating_chunk: bool,
    pub removing_chunk: bool,
    pub reloc_reserved: bool,
    pub in_fsync: bool,
    pub fs_info: *mut btrfs_fs_info,
    pub new_bgs: list_head,
    pub delayed_rsv: btrfs_block_rsv,
// Extent buffers this handle has inhibited writeback on.
    pub inhibited_ebs: [*mut extent_buffer; BTRFS_INHIBITED_EBS_SLOTS],
// CLOCK reference bit per slot.
    pub inhibited_ebs_referenced: u32,
    pub nr_inhibited_ebs: u32,
// CLOCK hand.
    pub inhibited_ebs_hand: u32,
}

//
// The abort status can be changed between calls and is not protected by locks.
// This accepts btrfs_transaction and btrfs_trans_handle as types. Once it's
// set to a non-zero value it does not change, so the macro should be in checks
// but is not necessary for further reads of the value.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_pending_snapshot {
    pub dentry: *mut dentry,
    pub dir: *mut btrfs_inode,
    pub root: *mut btrfs_root,
    pub root_item: *mut btrfs_root_item,
    pub snap: *mut btrfs_root,
    pub inherit: *mut btrfs_qgroup_inherit,
    pub path: *mut btrfs_path,
// block reservation for the operation
    pub block_rsv: btrfs_block_rsv,
// extra metadata reservation for relocation
    pub error: c_int,
// Preallocated anonymous block device number
    pub anon_dev: dev_t,
    pub readonly: bool,
    pub list: list_head,
}

//
// Make qgroup codes to skip given qgroupid, means the old/new_roots for
// qgroup won't contain the qgroupid in it.
//
// We want the transaction abort to print stack trace only for errors where the
// cause could be a bug, eg. due to ENOSPC, and not for common errors that are
// caused by external factors.
//
// Compile-time and run-time verification of error passed to transaction abort.
// Direct constants will be caught at compile time, errors read from variables
// can be caught only at run-time and will warn under debugging config.
//
// How verification works:
// - accepted builtin constants are all -EIO and such
// - for compile-time check, invalid condition produces a negative-sized array
// type, valid zero-sized
// - when a variable is passed as error the first check is a no-op
// - with enabled debugging, the second array type size is constructed from the
// real variable value, valid condition produces array of size 1
// - sizeof(type) does not generate any code
//

//
// Call btrfs_abort_transaction() as early as possible when an error condition
// is detected, that way the exact stack trace is reported for some errors.
//
// Error number must be negative as it encodes wheather it's the first abort.
//

// Report first abort since mount */			\
extern "C" {
    pub fn btrfs_end_transaction(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_wait_for_commit(fs_info: *mut btrfs_fs_info, transid: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_add_dead_root(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_maybe_wake_unfinished_drop(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_clean_one_deleted_snapshot(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_commit_transaction(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_commit_transaction_async(trans: *mut btrfs_trans_handle);
}
extern "C" {
    pub fn btrfs_commit_current_transaction(root: *mut btrfs_root) -> c_int;
}
extern "C" {
    pub fn btrfs_end_transaction_throttle(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_should_end_transaction(trans: *mut btrfs_trans_handle) -> bool;
}
extern "C" {
    pub fn btrfs_throttle(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_wait_tree_log_extents(root: *mut btrfs_root, mark: c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_transaction_blocked(info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_put_transaction(transaction: *mut btrfs_transaction);
}
extern "C" {
    pub fn btrfs_trans_release_chunk_metadata(trans: *mut btrfs_trans_handle);
}
extern "C" {
    pub fn btrfs_transaction_init() -> int __init;
}
extern "C" {
    pub fn btrfs_transaction_exit() -> void __cold;
}
