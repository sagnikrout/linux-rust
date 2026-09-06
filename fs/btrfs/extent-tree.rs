//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/extent-tree.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_extent_allocation_policy {
    BTRFS_EXTENT_ALLOC_CLUSTERED,
    BTRFS_EXTENT_ALLOC_ZONED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct find_free_extent_ctl {
// Basic allocation info
    pub ram_bytes: u64,
    pub num_bytes: u64,
    pub min_alloc_size: u64,
    pub empty_size: u64,
    pub flags: u64,
// Where to start the search inside the bg
    pub search_start: u64,
// For clustered allocation
    pub empty_cluster: u64,
    pub last_ptr: *mut btrfs_free_cluster,
    pub use_cluster: bool,
    pub delalloc: bool,
    pub have_caching_bg: bool,
    pub orig_have_caching_bg: bool,
// Allocation is called for tree-log
    pub for_treelog: bool,
// Allocation is called for data relocation
    pub for_data_reloc: bool,
//
// Set to true if we're retrying the allocation on this block group
// after waiting for caching progress, this is so that we retry only
// once before moving on to another block group.
//
    pub retry_uncached: bool,
// Whether or not the allocator is currently following a hint.
    pub hinted: bool,
// RAID index, converted from flags
    pub index: c_int,
//
// Current loop number, check find_free_extent_update_loop() for details
//
    pub loop: c_int,
// If current block group is cached
    pub cached: c_int,
// Max contiguous hole found
    pub max_extent_size: u64,
// Total free space from free space cache, not always contiguous
    pub total_free_space: u64,
// Found result
    pub found_offset: u64,
// Hint where to start looking for an empty space
    pub hint_byte: u64,
// Allocation policy
    pub policy: btrfs_extent_allocation_policy,
// Size class of block groups to prefer in early loops
    pub size_class: btrfs_block_group_size_class,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_inline_ref_type {
    BTRFS_REF_TYPE_INVALID,
    BTRFS_REF_TYPE_BLOCK,
    BTRFS_REF_TYPE_DATA,
    BTRFS_REF_TYPE_ANY,
}

extern "C" {
    pub fn hash_extent_data_ref(root_objectid: u64, owner: u64, offset: u64) -> u64;
}
extern "C" {
    pub fn btrfs_run_delayed_refs(trans: *mut btrfs_trans_handle, min_bytes: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_lookup_data_extent(fs_info: *mut btrfs_fs_info, start: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_pin_extent(trans: *mut btrfs_trans_handle, bytenr: u64, num: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_exclude_logged_extents(eb: *mut extent_buffer) -> c_int;
}
extern "C" {
    pub fn btrfs_free_extent(trans: *mut btrfs_trans_handle, ref: *mut btrfs_ref) -> c_int;
}
extern "C" {
    pub fn btrfs_finish_extent_commit(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_inc_extent_ref(trans: *mut btrfs_trans_handle, generic_ref: *mut btrfs_ref) -> c_int;
}
extern "C" {
    pub fn btrfs_drop_snapshot(root: *mut btrfs_root, update_ref: bool, for_reloc: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_error_unpin_extent_range(fs_info: *mut btrfs_fs_info, start: u64, end: u64);
}
extern "C" {
    pub fn btrfs_trim_fs(fs_info: *mut btrfs_fs_info, range: *mut fstrim_range) -> c_int;
}
extern "C" {
    pub fn btrfs_handle_fully_remapped_bgs(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_complete_bg_remapping(bg: *mut btrfs_block_group) -> c_int;
}
