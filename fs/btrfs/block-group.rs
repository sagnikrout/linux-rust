//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/block-group.h
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
pub enum btrfs_disk_cache_state {
    BTRFS_DC_WRITTEN,
    BTRFS_DC_ERROR,
    BTRFS_DC_CLEAR,
    BTRFS_DC_SETUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_block_group_size_class {
// Unset
    BTRFS_BG_SZ_NONE,
// 0 < size <= 128K
    BTRFS_BG_SZ_SMALL,
// 128K < size <= 8M
    BTRFS_BG_SZ_MEDIUM,
// 8M < size < BG_LENGTH
    BTRFS_BG_SZ_LARGE,
}

//
// This describes the state of the block_group for async discard.  This is due
// to the two pass nature of it where extent discarding is prioritized over
// bitmap discarding.  BTRFS_DISCARD_RESET_CURSOR is set when we are resetting
// between lists to prevent contention for discard state variables
// (eg. discard_cursor).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_discard_state {
    BTRFS_DISCARD_EXTENTS,
    BTRFS_DISCARD_BITMAPS,
    BTRFS_DISCARD_RESET_CURSOR,
    BTRFS_DISCARD_FULLY_REMAPPED,
}

//
// Control flags for do_chunk_alloc's force field CHUNK_ALLOC_NO_FORCE means to
// only allocate a chunk if we really need one.
//
// CHUNK_ALLOC_LIMITED means to only try and allocate one if we have very few
// chunks already allocated.  This is used as part of the clustering code to
// help make sure we have a good pool of storage to cluster in, without filling
// the FS with empty chunks
//
// CHUNK_ALLOC_FORCE means it must try to allocate one
//
// CHUNK_ALLOC_FORCE_FOR_EXTENT like CHUNK_ALLOC_FORCE but called from
// find_free_extent() that also activates the zone
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_chunk_alloc_enum {
    CHUNK_ALLOC_NO_FORCE,
    CHUNK_ALLOC_LIMITED,
    CHUNK_ALLOC_FORCE,
    CHUNK_ALLOC_FORCE_FOR_EXTENT,
}

// Block group flags set at runtime
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_block_group_flags {
    BLOCK_GROUP_FLAG_IREF,
    BLOCK_GROUP_FLAG_REMOVED,
    BLOCK_GROUP_FLAG_TO_COPY,
    BLOCK_GROUP_FLAG_RELOCATING_REPAIR,
    BLOCK_GROUP_FLAG_CHUNK_ITEM_INSERTED,
    BLOCK_GROUP_FLAG_ZONE_IS_ACTIVE,
    BLOCK_GROUP_FLAG_ZONED_DATA_RELOC,
// Does the block group need to be added to the free space tree?
    BLOCK_GROUP_FLAG_NEEDS_FREE_SPACE,
// Set after we add a new block group to the free space tree.
    BLOCK_GROUP_FLAG_FREE_SPACE_ADDED,
// Indicate that the block group is placed on a sequential zone
    BLOCK_GROUP_FLAG_SEQUENTIAL_ZONE,
//
// Indicate that block group is in the list of new block groups of a
// transaction.
//
    BLOCK_GROUP_FLAG_NEW,
    BLOCK_GROUP_FLAG_FULLY_REMAPPED,
    BLOCK_GROUP_FLAG_STRIPE_REMOVAL_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_caching_type {
    BTRFS_CACHE_NO,
    BTRFS_CACHE_STARTED,
    BTRFS_CACHE_FINISHED,
    BTRFS_CACHE_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_caching_control {
    pub list: list_head,
    pub mutex: mutex,
    pub wait: wait_queue_head_t,
    pub work: btrfs_work,
    pub block_group: *mut btrfs_block_group,
// Track progress of caching during allocation.
    pub progress: core::sync::atomic::AtomicI32,
    pub count: refcount_t,
}

// Once caching_thread() finds this much free space, it will wake up waiters.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_block_group {
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut btrfs_inode,
    pub lock: spinlock_t,
    pub ro: c_uint,
    pub start: u64,
    pub length: u64,
    pub pinned: u64,
    pub reserved: u64,
    pub used: u64,
    pub delalloc_bytes: u64,
    pub bytes_super: u64,
    pub flags: u64,
    pub cache_generation: u64,
    pub global_root_id: u64,
    pub remap_bytes: u64,
    pub identity_remap_count: u32,
// The last commited identity_remap_count value of this block group.
    pub last_identity_remap_count: u32,
//
// The last committed used bytes of this block group, if the above @used
// is still the same as @last_used, we don't need to update block
// group item of this block group.
//
    pub last_used: u64,
// The last committed remap_bytes value of this block group.
    pub last_remap_bytes: u64,
// The last committed flags value for this block group.
    pub last_flags: u64,
//
// If the free space extent count exceeds this number, convert the block
// group to bitmaps.
//
    pub bitmap_high_thresh: u32,
//
// If the free space extent count drops below this number, convert the
// block group back to extents.
//
    pub bitmap_low_thresh: u32,
//
// It is just used for the delayed data space allocation because
// only the data space allocation and the relative metadata update
// can be done cross the transaction.
//
    pub data_rwsem: rw_semaphore,
// For raid56, this is a full stripe, without parity
    pub full_stripe_len: c_ulong,
    pub runtime_flags: c_ulong,
    pub disk_cache_state: btrfs_disk_cache_state,
// Cache tracking stuff
    pub cached: btrfs_caching_type,
    pub caching_ctl: *mut btrfs_caching_control,
    pub space_info: *mut btrfs_space_info,
// Free space cache stuff
    pub free_space_ctl: *mut btrfs_free_space_ctl,
// Block group cache stuff
    pub cache_node: rb_node,
// For block groups in the same raid type
    pub list: list_head,
    pub refs: refcount_t,
//
// When non-zero it means the block group's logical address and its
// device extents can not be reused for future block group allocations
// until the counter goes down to 0. This is to prevent them from being
// reused while some task is still using the block group after it was
// deleted - we want to make sure they can only be reused for new block
// groups after that task is done with the deleted block group.
//
    pub frozen: core::sync::atomic::AtomicI32,
//
// List of struct btrfs_free_clusters for this block group.
// Today it will only have one thing on it, but that may change
//
    pub cluster_list: list_head,
//
// Used for several lists:
//
// 1) struct btrfs_fs_info::unused_bgs
// 2) struct btrfs_fs_info::reclaim_bgs
// 3) struct btrfs_transaction::deleted_bgs
// 4) struct btrfs_trans_handle::new_bgs
//
    pub bg_list: list_head,
// For read-only block groups
    pub ro_list: list_head,
// For discard operations
    pub discard_list: list_head,
    pub discard_index: c_int,
    pub discard_state: btrfs_discard_state,
    pub discard_eligible_time: u64,
    pub discard_cursor: u64,
// For dirty block groups
    pub dirty_list: list_head,
    pub io_list: list_head,
    pub io_ctl: btrfs_io_ctl,
//
// Incremented when doing extent allocations and holding a read lock
// on the space_info's groups_sem semaphore.
// Decremented when an ordered extent that represents an IO against this
// block group's range is created (after it's added to its inode's
// root's list of ordered extents) or immediately after the allocation
// if it's a metadata extent or fallocate extent (for these cases we
// don't create ordered extents).
//
    pub reservations: core::sync::atomic::AtomicI32,
//
// Incremented while holding the spinlock *lock* by a task checking if
// it can perform a nocow write (incremented if the value for the *ro
// field is 0). Decremented by such tasks once they create an ordered
// extent or before that if some error happens before reaching that step.
// This is to prevent races between block group relocation and nocow
// writes through direct IO.
//
    pub nocow_writers: core::sync::atomic::AtomicI32,
// Lock for free space tree operations.
    pub free_space_lock: mutex,
// Protected by @free_space_lock.
    pub using_free_space_bitmaps: bool,
// Protected by @free_space_lock.
    pub using_free_space_bitmaps_cached: bool,
    pub size_class:8: btrfs_block_group_size_class,
// If set, this blockgroup is not used for allocation between two reclaim sweeps.
    pub reclaim_mark: bool,
//
// Number of extents in this block group used for swap files.
// All accesses protected by the spinlock 'lock'.
//
    pub swap_extents: c_int,
//
// Allocation offset for the block group to implement sequential
// allocation. This is used only on a zoned filesystem.
//
    pub alloc_offset: u64,
    pub zone_unusable: u64,
    pub zone_capacity: u64,
    pub meta_write_pointer: u64,
    pub physical_map: *mut btrfs_chunk_map,
    pub active_bg_list: list_head,
    pub zone_finish_work: work_struct,
    pub last_eb: *mut extent_buffer,
}

//
// In mixed mode the fragmentation is expected to be high, lowering the
// efficiency, so only proper data block groups are considered.
//

extern "C" {
    pub fn btrfs_should_fragment_free_space(block_group: *const btrfs_block_group) -> c_int;
}

extern "C" {
    pub fn btrfs_init_block_group() -> int __init;
}
extern "C" {
    pub fn btrfs_exit_block_group() -> void __cold;
}
extern "C" {
    pub fn btrfs_get_block_group(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_put_block_group(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_wait_block_group_reservations(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_dec_nocow_writers(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_wait_nocow_writers(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_cache_block_group(cache: *mut btrfs_block_group, wait: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_remove_bg_from_sinfo(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_delete_unused_bgs(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_mark_bg_unused(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_reclaim_block_groups(fs_info: *mut btrfs_fs_info, limit: c_uint);
}
extern "C" {
    pub fn btrfs_reclaim_bgs_work(work: *mut work_struct);
}
extern "C" {
    pub fn btrfs_reclaim_bgs(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_mark_bg_to_reclaim(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_read_block_groups(info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_create_pending_block_groups(trans: *mut btrfs_trans_handle);
}
extern "C" {
    pub fn btrfs_dec_block_group_ro(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_start_dirty_block_groups(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_write_dirty_block_groups(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_setup_space_cache(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_force_chunk_alloc(trans: *mut btrfs_trans_handle, type: u64) -> c_int;
}
extern "C" {
    pub fn check_system_chunk(trans: *mut btrfs_trans_handle, type: u64);
}
extern "C" {
    pub fn btrfs_get_alloc_profile(fs_info: *mut btrfs_fs_info, orig_flags: u64) -> u64;
}
extern "C" {
    pub fn btrfs_put_block_group_cache(info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_free_block_groups(info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_get_alloc_profile(_arg: fs_info, _arg: BTRFS_BLOCK_GROUP_DATA) -> return;
}
extern "C" {
    pub fn btrfs_get_alloc_profile(_arg: fs_info, _arg: BTRFS_BLOCK_GROUP_METADATA) -> return;
}
extern "C" {
    pub fn btrfs_get_alloc_profile(_arg: fs_info, _arg: BTRFS_BLOCK_GROUP_SYSTEM) -> return;
}
extern "C" {
    pub fn btrfs_freeze_block_group(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_unfreeze_block_group(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_inc_block_group_swap_extents(bg: *mut btrfs_block_group) -> bool;
}
extern "C" {
    pub fn btrfs_dec_block_group_swap_extents(bg: *mut btrfs_block_group, amount: c_int);
}
extern "C" {
    pub fn btrfs_calc_block_group_size_class(size: u64) -> btrfs_block_group_size_class;
}
extern "C" {
    pub fn btrfs_block_group_should_use_size_class(bg: *const btrfs_block_group) -> bool;
}
extern "C" {
    pub fn btrfs_populate_fully_remapped_bgs_list(fs_info: *mut btrfs_fs_info) -> c_int;
}
