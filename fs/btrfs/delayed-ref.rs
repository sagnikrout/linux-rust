//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/delayed-ref.h
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

// these are the possible values of struct btrfs_delayed_ref_node->action
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_delayed_ref_action {
// Add one backref to the tree
    BTRFS_ADD_DELAYED_REF = 1,
// Delete one backref from the tree
    BTRFS_DROP_DELAYED_REF,
// Record a full extent allocation
    BTRFS_ADD_DELAYED_EXTENT,
// Not changing ref count on head ref
    BTRFS_UPDATE_DELAYED_HEAD,
    } __packed;

    struct btrfs_data_ref {
// For EXTENT_DATA_REF

// Inode which refers to this data extent
    u64 objectid;

//
// file_offset - extent_offset
//
// file_offset is the key.offset of the EXTENT_DATA key.
// extent_offset is btrfs_file_extent_offset() of the EXTENT_DATA data.
//
    u64 offset;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_tree_ref {
//
// Level of this tree block.
//
// Shared for skinny (TREE_BLOCK_REF) and normal tree ref.
//
    pub level: c_int,
// For non-skinny metadata, no special member needed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_ref_node {
    pub ref_node: rb_node,
//
// If action is BTRFS_ADD_DELAYED_REF, also link this node to
// ref_head->ref_add_list, then we do not need to iterate the
// refs rbtree in the corresponding delayed ref head
// (struct btrfs_delayed_ref_head::ref_tree).
//
    pub add_list: list_head,
// the starting bytenr of the extent
    pub bytenr: u64,
// the size of the extent
    pub num_bytes: u64,
// seq number to keep track of insertion order
    pub seq: u64,
// The ref_root for this ref
    pub ref_root: u64,
//
// The parent for this ref, if this isn't set the ref_root is the
// reference owner.
//
    pub parent: u64,
// ref count on this data structure
    pub refs: refcount_t,
//
// how many refs is this entry adding or deleting.  For
// head refs, this may be a negative number because it is keeping
// track of the total mods done to the reference count.
// For individual refs, this will always be a positive number
//
// It may be more than one, since it is possible for a single
// parent to have more than one ref on an extent
//
    pub ref_mod: c_int,
    pub action:8: c_uint,
    pub type:8: c_uint,
    pub tree_ref: btrfs_tree_ref,
    pub data_ref: btrfs_data_ref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_extent_op {
    pub key: btrfs_disk_key,
    pub update_key: bool,
    pub update_flags: bool,
    pub flags_to_set: u64,
}

//
// the head refs are used to hold a lock on a given extent, which allows us
// to make sure that only one process is running the delayed refs
// at a time for a single extent.  They also store the sum of all the
// reference count modifications we've queued up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_ref_head {
    pub bytenr: u64,
    pub num_bytes: u64,
//
// the mutex is held while running the refs, and it is also
// held when checking the sum of reference modifications.
//
    pub mutex: mutex,
    pub refs: refcount_t,
// Protects 'ref_tree' and 'ref_add_list'.
    pub lock: spinlock_t,
    pub ref_tree: rb_root_cached,
// accumulate add BTRFS_ADD_DELAYED_REF nodes to this ref_add_list.
    pub ref_add_list: list_head,
    pub extent_op: *mut btrfs_delayed_extent_op,
//
// This is used to track the final ref_mod from all the refs associated
// with this head ref, this is not adjusted as delayed refs are run,
// this is meant to track if we need to do the csum accounting or not.
//
    pub total_ref_mod: c_int,
//
// This is the current outstanding mod references for this bytenr.  This
// is used with lookup_extent_info to get an accurate reference count
// for a bytenr, so it is adjusted as delayed refs are run so that any
// on disk reference count + ref_mod is accurate.
//
    pub ref_mod: c_int,
//
// The root that triggered the allocation when must_insert_reserved is
// set to true.
//
    pub owning_root: u64,
//
// Track reserved bytes when setting must_insert_reserved.  On success
// or cleanup, we will need to free the reservation.
//
    pub reserved_bytes: u64,
// Tree block level, for metadata only.
    pub level: u8,
//
// when a new extent is allocated, it is just reserved in memory
// The actual extent isn't inserted into the extent allocation tree
// until the delayed ref is processed.  must_insert_reserved is
// used to flag a delayed ref so the accounting can be updated
// when a full insert is done.
//
// It is possible the extent will be freed before it is ever
// inserted into the extent allocation tree.  In this case
// we need to update the in ram accounting to properly reflect
// the free has happened.
//
    pub must_insert_reserved: bool,
    pub is_data: bool,
    pub is_system: bool,
    pub processing: bool,
//
// Indicate if it's currently in the data structure that tracks head
// refs (struct btrfs_delayed_ref_root::head_refs).
//
    pub tracked: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_delayed_ref_flags {
// Indicate that we are flushing delayed refs for the commit
    BTRFS_DELAYED_REFS_FLUSHING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_ref_root {
//
// Track head references.
// The keys correspond to the logical address of the extent ("bytenr")
// right shifted by fs_info->sectorsize_bits. This is both to get a more
// dense index space (optimizes xarray structure) and because indexes in
// xarrays are of "unsigned long" type, meaning they are 32 bits wide on
// 32 bits platforms, limiting the extent range to 4G which is too low
// and makes it unusable (truncated index values) on 32 bits platforms.
// Protected by the spinlock 'lock' defined below.
//
    pub head_refs: xarray,
//
// Track dirty extent records.
// The keys correspond to the logical address of the extent ("bytenr")
// right shifted by fs_info->sectorsize_bits, for same reasons as above.
//
    pub dirty_extents: xarray,
//
// Protects the xarray head_refs, its entries and the following fields:
// num_heads, num_heads_ready, pending_csums and run_delayed_start.
//
    pub lock: spinlock_t,
// Total number of head refs, protected by the spinlock 'lock'.
    pub num_heads: c_ulong,
//
// Total number of head refs ready for processing, protected by the
// spinlock 'lock'.
//
    pub num_heads_ready: c_ulong,
//
// Track space reserved for deleting csums of data extents.
// Protected by the spinlock 'lock'.
//
    pub pending_csums: u64,
    pub flags: c_ulong,
//
// Track from which bytenr to start searching ref heads.
// Protected by the spinlock 'lock'.
//
    pub run_delayed_start: u64,
//
// To make qgroup to skip given root.
// This is for snapshot, as btrfs_qgroup_inherit() will manually
// modify counters for snapshot and its source, so we should skip
// the snapshot in new_root/old_roots or it will get calculated twice
//
    pub qgroup_to_skip: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_ref_type {
    BTRFS_REF_NOT_SET,
    BTRFS_REF_DATA,
    BTRFS_REF_METADATA,
    } __packed;

    struct btrfs_ref {
    enum btrfs_ref_type type;
    enum btrfs_delayed_ref_action action;

//
// Whether this extent should go through qgroup record.
//
// Normally false, but for certain cases like delayed subtree scan,
// setting this flag can hugely reduce qgroup overhead.
//
    bool skip_qgroup;

    u64 bytenr;
    u64 num_bytes;
    u64 owning_root;

//
// The root that owns the reference for this reference, this will be set
// or ->parent will be set, depending on what type of reference this is.
//
    u64 ref_root;

// Bytenr of the parent tree block
    u64 parent;
    union {
    struct btrfs_data_ref data_ref;
    struct btrfs_tree_ref tree_ref;
}

// Through which root is this modification.

extern "C" {
    pub fn btrfs_delayed_ref_init() -> int __init;
}
extern "C" {
    pub fn btrfs_delayed_ref_exit() -> void __cold;
}
//
// We have to check the mount option here because we could be enabling
// the free space tree for the first time and don't have the compat_ro
// option set yet.
//
// We need extra reservations if we have the free space tree because
// we'll have to modify that tree as well.
//
// Deleting csum items does not result in new nodes/leaves and does not
// require changing the free space tree, only the csum tree, so this is
// all we need.
//
extern "C" {
    pub fn btrfs_calc_metadata_size(_arg: fs_info, _arg: num_csum_items) -> return;
}
extern "C" {
    pub fn kmem_cache_alloc(_arg: btrfs_delayed_extent_op_cachep, _arg: GFP_NOFS) -> return;
}
extern "C" {
    pub fn btrfs_put_delayed_ref(ref: *mut btrfs_delayed_ref_node);
}
extern "C" {
    pub fn btrfs_check_delayed_seq(fs_info: *mut btrfs_fs_info, seq: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_delayed_refs_rsv_release(fs_info: *mut btrfs_fs_info, nr_refs: c_int, nr_csums: c_int);
}
extern "C" {
    pub fn btrfs_update_delayed_refs_rsv(trans: *mut btrfs_trans_handle);
}
extern "C" {
    pub fn btrfs_inc_delayed_refs_rsv_bg_inserts(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_dec_delayed_refs_rsv_bg_inserts(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_inc_delayed_refs_rsv_bg_updates(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_dec_delayed_refs_rsv_bg_updates(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_check_space_for_delayed_refs(fs_info: *mut btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_destroy_delayed_refs(trans: *mut btrfs_transaction);
}
