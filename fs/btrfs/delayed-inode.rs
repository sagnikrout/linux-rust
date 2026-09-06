//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/delayed-inode.h
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
// Copyright (C) 2011 Fujitsu.  All rights reserved.
// Written by Miao Xie <miaox@cn.fujitsu.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_delayed_item_type {
    BTRFS_DELAYED_INSERTION_ITEM,
    BTRFS_DELAYED_DELETION_ITEM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ref_tracker_dir {

    pub dir: ref_tracker_dir,

    pub tracker: {},

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_ref_tracker {

    pub tracker: *mut ref_tracker,

    pub tracker: {},

}

pub const BTRFS_DELAYED_NODE_IN_LIST: c_int = 0;
pub const BTRFS_DELAYED_NODE_INODE_DIRTY: c_int = 1;
pub const BTRFS_DELAYED_NODE_DEL_IREF: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_node {
    pub inode_id: u64,
    pub bytes_reserved: u64,
    pub root: *mut btrfs_root,
// Used to add the node into the delayed root's node list.
    pub n_list: list_head,
//
// Used to add the node into the prepare list, the nodes in this list
// is waiting to be dealt with by the async worker.
//
    pub p_list: list_head,
    pub ins_root: rb_root_cached,
    pub del_root: rb_root_cached,
    pub mutex: mutex,
    pub inode_item: btrfs_inode_item,
    pub refs: refcount_t,
    pub count: c_int,
    pub index_cnt: u64,
    pub flags: c_ulong,
//
// The size of the next batch of dir index items to insert (if this
// node is from a directory inode). Protected by @mutex.
//
    pub curr_index_batch_size: u32,
//
// Number of leaves reserved for inserting dir index items (if this
// node belongs to a directory inode). This may be larger then the
// actual number of leaves we end up using. Protected by @mutex.
//
    pub index_item_leaves: u32,
// Track all references to this delayed node.
    pub ref_dir: btrfs_ref_tracker_dir,
// Track delayed node reference stored in node list.
    pub node_list_tracker: btrfs_ref_tracker,
// Track delayed node reference stored in inode cache.
    pub inode_cache_tracker: btrfs_ref_tracker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_item {
    pub rb_node: rb_node,
// Offset value of the corresponding dir index key.
    pub index: u64,
    pub /: *mut *mut list_head tree_list; / used for batch insert/delete items,
    pub /: *mut *mut list_head readdir_list; / used for readdir items,
//
// Used when logging a directory.
// Insertions and deletions to this list are protected by the parent
// delayed node's mutex.
//
    pub log_list: list_head,
    pub bytes_reserved: u64,
    pub delayed_node: *mut btrfs_delayed_node,
    pub refs: refcount_t,
    pub type:8: btrfs_delayed_item_type,
//
// Track if this delayed item was already logged.
// Protected by the mutex of the parent delayed inode.
//
    pub logged: bool,
// The maximum leaf size is 64K, so u16 is more than enough.
    pub data_len: u16,
    pub __counted_by(data_len): char data[],
}

extern "C" {
    pub fn btrfs_init_delayed_root(delayed_root: *mut btrfs_delayed_root);
}
extern "C" {
    pub fn btrfs_inode_delayed_dir_index_count(inode: *mut btrfs_inode) -> c_int;
}
extern "C" {
    pub fn btrfs_run_delayed_items(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_run_delayed_items_nr(trans: *mut btrfs_trans_handle, nr: c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_balance_delayed_items(fs_info: *mut btrfs_fs_info);
}
// Used for evicting the inode.
extern "C" {
    pub fn btrfs_remove_delayed_node(inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_kill_delayed_inode_items(inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_commit_inode_delayed_inode(inode: *mut btrfs_inode) -> c_int;
}
extern "C" {
    pub fn btrfs_fill_inode(inode: *mut btrfs_inode, rdev: *mut u32) -> c_int;
}
extern "C" {
    pub fn btrfs_delayed_delete_inode_ref(inode: *mut btrfs_inode) -> c_int;
}
// Used for drop dead root
extern "C" {
    pub fn btrfs_kill_all_delayed_nodes(root: *mut btrfs_root);
}
// Used for clean the transaction
extern "C" {
    pub fn btrfs_destroy_delayed_inodes(fs_info: *mut btrfs_fs_info);
}
// Used for readdir()
extern "C" {
    pub fn btrfs_should_delete_dir_index(del_list: *const list_head, index: u64) -> bool;
}
// Used during directory logging.
// for init
extern "C" {
    pub fn btrfs_delayed_inode_init() -> int __init;
}
extern "C" {
    pub fn btrfs_delayed_inode_exit() -> void __cold;
}
// for debugging
extern "C" {
    pub fn btrfs_assert_delayed_root_empty(fs_info: *mut btrfs_fs_info);
}
pub const BTRFS_DELAYED_NODE_REF_TRACKER_QUARANTINE_COUNT: c_int = 16;
pub const BTRFS_DELAYED_NODE_REF_TRACKER_DISPLAY_LIMIT: c_int = 16;

//
// Only print if there are leaked references. The caller is
// holding one reference, so if refs == 1 there is no leak.
//
extern "C" {
    pub fn ref_tracker_alloc(_arg: &node->ref_dir.dir, _arg: &tracker->tracker, _arg: gfp) -> return;
}
extern "C" {
    pub fn ref_tracker_free(_arg: &node->ref_dir.dir, _arg: &tracker->tracker) -> return;
}

