//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/qgroup.h
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
// Copyright (C) 2014 Facebook.  All rights reserved.
//

//
// Btrfs qgroup overview
//
// Btrfs qgroup splits into 3 main part:
// 1) Reserve
// Reserve metadata/data space for incoming operations
// Affect how qgroup limit works
//
// 2) Trace
// Tell btrfs qgroup to trace dirty extents.
//
// Dirty extents including:
// - Newly allocated extents
// - Extents going to be deleted (in this trans)
// - Extents whose owner is going to be modified
//
// This is the main part affects whether qgroup numbers will stay
// consistent.
// Btrfs qgroup can trace clean extents and won't cause any problem,
// but it will consume extra CPU time, it should be avoided if possible.
//
// 3) Account
// Btrfs qgroup will updates its numbers, based on dirty extents traced
// in previous step.
//
// Normally at qgroup rescan and transaction commit time.
//
// Special performance optimization for balance.
//
// For balance, we need to swap subtree of subvolume and reloc trees.
// In theory, we need to trace all subtree blocks of both subvolume and reloc
// trees, since their owner has changed during such swap.
//
// However since balance has ensured that both subtrees are containing the
// same contents and have the same tree structures, such swap won't cause
// qgroup number change.
//
// But there is a race window between subtree swap and transaction commit,
// during that window, if we increase/decrease tree level or merge/split tree
// blocks, we still need to trace the original subtrees.
//
// So for balance, we use a delayed subtree tracing, whose workflow is:
//
// 1) Record the subtree root block get swapped.
//
// During subtree swap:
// O = Old tree blocks
// N = New tree blocks
// reloc tree                     subvolume tree X
// Root                               Root
// /    \                             /    \
// NA     OB                          OA      OB
// /  |     |  \                      /  |      |  \
// NC  ND     OE  OF                   OC  OD     OE  OF
//
// In this case, NA and OA are going to be swapped, record (NA, OA) into
// subvolume tree X.
//
// 2) After subtree swap.
// reloc tree                     subvolume tree X
// Root                               Root
// /    \                             /    \
// OA     OB                          NA      OB
// /  |     |  \                      /  |      |  \
// OC  OD     OE  OF                   NC  ND     OE  OF
//
// 3a) COW happens for OB
// If we are going to COW tree block OB, we check OB's bytenr against
// tree X's swapped_blocks structure.
// If it doesn't fit any, nothing will happen.
//
// 3b) COW happens for NA
// Check NA's bytenr against tree X's swapped_blocks, and get a hit.
// Then we do subtree scan on both subtrees OA and NA.
// Resulting 6 tree blocks to be scanned (OA, OC, OD, NA, NC, ND).
//
// Then no matter what we do to subvolume tree X, qgroup numbers will
// still be correct.
// Then NA's record gets removed from X's swapped_blocks.
//
// 4)  Transaction commit
// Any record in X's swapped_blocks gets removed, since there is no
// modification to the swapped subtrees, no need to trigger heavy qgroup
// subtree rescan for them.
//
// These flags share the flags field of the btrfs_qgroup_status_item with the
// persisted flags defined in btrfs_tree.h.
//
// To minimize the chance of collision with new persisted status flags, these
// count backwards from the MSB.
//

//
// Record a dirty extent, and info qgroup to update quota on it
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_extent_record {
//
// The bytenr of the extent is given by its index in the dirty_extents
// xarray of struct btrfs_delayed_ref_root left shifted by
// fs_info->sectorsize_bits.
//
    pub num_bytes: u64,
//
// For qgroup reserved data space freeing.
//
// @data_rsv_refroot and @data_rsv will be recorded after
// BTRFS_ADD_DELAYED_EXTENT is called.
// And will be used to free reserved qgroup space at
// transaction commit time.
//
    pub /: *mut *mut u32 data_rsv; / reserved data space needs to be freed,
    pub /: *mut *mut u64 data_rsv_refroot; / which root the reserved data belongs to,
    pub old_roots: *mut ulist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_swapped_block {
    pub node: rb_node,
    pub level: c_int,
    pub trace_leaf: bool,
// bytenr/generation of the tree block in subvolume tree after swap
    pub subvol_bytenr: u64,
    pub subvol_generation: u64,
// bytenr/generation of the tree block in reloc tree after swap
    pub reloc_bytenr: u64,
    pub reloc_generation: u64,
    pub last_snapshot: u64,
    pub first_key: btrfs_key,
}

//
// Qgroup reservation types:
//
// DATA:
// space reserved for data
//
// META_PERTRANS:
// Space reserved for metadata (per-transaction)
// Due to the fact that qgroup data is only updated at transaction commit
// time, reserved space for metadata must be kept until transaction
// commits.
// Any metadata reserved that are used in btrfs_start_transaction() should
// be of this type.
//
// META_PREALLOC:
// There are cases where metadata space is reserved before starting
// transaction, and then btrfs_join_transaction() to get a trans handle.
// Any metadata reserved for such usage should be of this type.
// And after join_transaction() part (or all) of such reservation should
// be converted into META_PERTRANS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_qgroup_rsv_type {
    BTRFS_QGROUP_RSV_DATA,
    BTRFS_QGROUP_RSV_META_PERTRANS,
    BTRFS_QGROUP_RSV_META_PREALLOC,
    BTRFS_QGROUP_RSV_LAST,
}

//
// Represents how many bytes we have reserved for this qgroup.
//
// Each type should have different reservation behavior.
// E.g, data follows its io_tree flag modification, while
// *currently* meta is just reserve-and-clear during transaction.
//
// TODO: Add new type for reservation which can survive transaction commit.
// Current metadata reservation behavior is not suitable for such case.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_rsv {
    pub values: [u64; BTRFS_QGROUP_RSV_LAST],
}

//
// one struct for each qgroup, organized in fs_info->qgroup_tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup {
    pub qgroupid: u64,
//
// state
//
    pub /: *mut *mut u64 rfer; / referenced,
    pub /: *mut *mut u64 rfer_cmpr; / referenced compressed,
    pub /: *mut *mut u64 excl; / exclusive,
    pub /: *mut *mut u64 excl_cmpr; / exclusive compressed,
//
// limits
//
    pub /: *mut *mut u64 lim_flags; / which limits are set,
    pub max_rfer: u64,
    pub max_excl: u64,
    pub rsv_rfer: u64,
    pub rsv_excl: u64,
//
// reservation tracking
//
    pub rsv: btrfs_qgroup_rsv,
//
// lists
//
    pub /: *mut *mut list_head groups; / groups this group is member of,
    pub /: *mut *mut list_head members; / groups that are members of this group,
    pub /: *mut *mut list_head dirty; / dirty groups,
//
// For qgroup iteration usage.
//
// The iteration list should always be empty until qgroup_iterator_add()
// is called.  And should be reset to empty after the iteration is
// finished.
//
    pub iterator: list_head,
//
// For nested iterator usage.
//
// Here we support at most one level of nested iterator calls like:
//
// LIST_HEAD(all_qgroups);
// {
// LIST_HEAD(local_qgroups);
// qgroup_iterator_add(local_qgroups, qg);
// qgroup_iterator_nested_add(all_qgroups, qg);
// do_some_work(local_qgroups);
// qgroup_iterator_clean(local_qgroups);
// }
// do_some_work(all_qgroups);
// qgroup_iterator_nested_clean(all_qgroups);
//
    pub nested_iterator: list_head,
    pub /: *mut *mut rb_node node; / tree of qgroups,
//
// temp variables for accounting operations
// Refer to qgroup_shared_accounting() for details.
//
    pub old_refcnt: u64,
    pub new_refcnt: u64,
//
// Sysfs kobjectid
//
    pub kobj: kobject,
}

// Glue structure to represent the relations between qgroups.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_qgroup_list {
    pub next_group: list_head,
    pub next_member: list_head,
    pub group: *mut btrfs_qgroup,
    pub member: *mut btrfs_qgroup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_squota_delta {
// The fstree root this delta counts against.
    pub root: u64,
// The number of bytes in the extent being counted.
    pub num_bytes: u64,
// The generation the extent was created in.
    pub generation: u64,
// Whether we are using or freeing the extent.
    pub is_inc: bool,
// Whether the extent is data or metadata.
    pub is_data: bool,
}

//
// For qgroup event trace points only
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_qgroup_mode {
    BTRFS_QGROUP_MODE_DISABLED,
    BTRFS_QGROUP_MODE_FULL,
    BTRFS_QGROUP_MODE_SIMPLE
}

extern "C" {
    pub fn btrfs_qgroup_mode(fs_info: *const btrfs_fs_info) -> btrfs_qgroup_mode;
}
extern "C" {
    pub fn btrfs_qgroup_enabled(fs_info: *const btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_qgroup_full_accounting(fs_info: *const btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_quota_disable(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_qgroup_rescan(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_qgroup_rescan_resume(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_create_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_remove_qgroup(trans: *mut btrfs_trans_handle, qgroupid: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_qgroup_cleanup_dropped_subvolume(fs_info: *mut btrfs_fs_info, subvolid: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_read_qgroup_config(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_free_qgroup_config(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_qgroup_account_extents(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_run_qgroups(trans: *mut btrfs_trans_handle) -> c_int;
}

// New io_tree based accurate qgroup reserve API
extern "C" {
    pub fn btrfs_qgroup_release_data(inode: *mut btrfs_inode, start: u64, len: u64, released: *mut u64) -> c_int;
}
// Pre-allocated meta reservation can be freed at need
extern "C" {
    pub fn btrfs_qgroup_free_meta_prealloc(root: *mut btrfs_root, num_bytes: c_int);
}
extern "C" {
    pub fn btrfs_qgroup_free_meta_all_pertrans(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_qgroup_convert_reserved_meta(root: *mut btrfs_root, num_bytes: c_int);
}
extern "C" {
    pub fn btrfs_qgroup_check_reserved_leak(inode: *mut btrfs_inode);
}
// btrfs_qgroup_swapped_blocks related functions
extern "C" {
    pub fn btrfs_qgroup_clean_swapped_blocks(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_qgroup_destroy_extent_records(trans: *mut btrfs_transaction);
}
extern "C" {
    pub fn btrfs_check_quota_leak(fs_info: *const btrfs_fs_info) -> bool;
}
