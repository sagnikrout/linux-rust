//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/tree-log.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_log_mode {
// Log everything about an inode.
    LOG_INODE_ALL,
// Log just enough to recreate the inode during log replay.
    LOG_INODE_EXISTS,
}

// return value for btrfs_log_dentry_safe that means we don't need to log it at all
pub const BTRFS_NO_LOG_SYNC: c_int = 256;
//
// We can't use the tree log for whatever reason, force a transaction commit.
// We use a negative value because there are functions through the logging code
// that need to return an error (< 0 value), false (0) or true (1). Any negative
// value will do, as it will cause the log to be marked for a full sync.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_log_ctx {
    pub log_ret: c_int,
    pub log_transid: c_int,
    pub log_new_dentries: bool,
    pub logging_new_name: bool,
    pub logging_new_delayed_dentries: bool,
// Indicate if the inode being logged was logged before.
    pub logged_before: bool,
    pub inode: *mut btrfs_inode,
    pub list: list_head,
// Only used for fast fsyncs.
    pub ordered_extents: list_head,
    pub conflict_inodes: list_head,
    pub num_conflict_inodes: c_int,
    pub logging_conflict_inodes: bool,
//
// Used for fsyncs that need to copy items from the subvolume tree to
// the log tree (full sync flag set or copy everything flag set) to
// avoid allocating a temporary extent buffer while holding a lock on
// an extent buffer of the subvolume tree and under the log transaction.
// Also helps to avoid allocating and freeing a temporary extent buffer
// in case we need to process multiple leaves from the subvolume tree.
//
    pub scratch_eb: *mut extent_buffer,
}

extern "C" {
    pub fn btrfs_init_log_ctx(ctx: *mut btrfs_log_ctx, inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_init_log_ctx_scratch_eb(ctx: *mut btrfs_log_ctx);
}
extern "C" {
    pub fn btrfs_release_log_ctx_extents(ctx: *mut btrfs_log_ctx);
}
extern "C" {
    pub fn btrfs_free_log(trans: *mut btrfs_trans_handle, root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_free_log_root_tree(trans: *mut btrfs_trans_handle, fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_recover_log_trees(tree_root: *mut btrfs_root) -> c_int;
}
extern "C" {
    pub fn btrfs_end_log_trans(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_pin_log_trans(root: *mut btrfs_root);
}
