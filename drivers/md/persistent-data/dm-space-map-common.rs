//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-space-map-common.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// Low level disk format
//
// Bitmap btree
// ------------
//
// Each value stored in the btree is an index_entry.  This points to a
// block that is used as a bitmap.  Within the bitmap hold 2 bits per
// entry, which represent UNUSED = 0, REF_COUNT = 1, REF_COUNT = 2 and
// REF_COUNT = many.
//
// Refcount btree
// --------------
//
// Any entry that has a ref count higher than 2 gets entered in the ref
// count tree.  The leaf values for this tree is the 32-bit ref count.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_index_entry {
    pub blocknr: __le64,
    pub nr_free: __le32,
    pub none_free_before: __le32,
    pub __aligned(8): } __packed,
pub const MAX_METADATA_BITMAPS: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_metadata_index {
    pub csum: __le32,
    pub padding: __le32,
    pub blocknr: __le64,
    pub index: [disk_index_entry; MAX_METADATA_BITMAPS],
    pub __aligned(8): } __packed,
    pub ll_disk: struct,
    pub result): *mut *mut *mut typedef int (load_ie_fn)(struct ll_disk ll, dm_block_t index, struct disk_index_entry,
    pub ie): *mut *mut *mut typedef int (save_ie_fn)(struct ll_disk ll, dm_block_t index, struct disk_index_entry,
    pub ll): *mut *mut typedef int (init_index_fn)(struct ll_disk,
    pub ll): *mut *mut typedef int (open_index_fn)(struct ll_disk,
    pub ll): *mut *mut typedef dm_block_t (max_index_entries_fn)(struct ll_disk,
    pub ll): *mut *mut typedef int (commit_fn)(struct ll_disk,
//
// A lot of time can be wasted reading and writing the same
// index entry.  So we cache a few entries.
//
pub const IE_CACHE_SIZE: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ie_cache {
    pub valid: bool,
    pub dirty: bool,
    pub index: dm_block_t,
    pub ie: disk_index_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ll_disk {
    pub tm: *mut dm_transaction_manager,
    pub bitmap_info: dm_btree_info,
    pub ref_count_info: dm_btree_info,
    pub block_size: u32,
    pub entries_per_block: u32,
    pub nr_blocks: dm_block_t,
    pub nr_allocated: dm_block_t,
//
// bitmap_root may be a btree root or a simple index.
//
    pub bitmap_root: dm_block_t,
    pub ref_count_root: dm_block_t,
    pub mi_le: disk_metadata_index,
    pub load_ie: load_ie_fn,
    pub save_ie: save_ie_fn,
    pub init_index: init_index_fn,
    pub open_index: open_index_fn,
    pub max_entries: max_index_entries_fn,
    pub commit: commit_fn,
    pub bitmap_index_changed:1: bool,
    pub ie_cache: [ie_cache; IE_CACHE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_sm_root {
    pub nr_blocks: __le64,
    pub nr_allocated: __le64,
    pub bitmap_root: __le64,
    pub ref_count_root: __le64,
    pub __aligned(8): } __packed,
pub const ENTRIES_PER_BYTE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_bitmap_header {
    pub csum: __le32,
    pub not_used: __le32,
    pub blocknr: __le64,
    pub __aligned(8): } __packed,
// ----------------------------------------------------------------
    pub extra_blocks): *mut *mut int sm_ll_extend(struct ll_disk ll, dm_block_t,
    pub result): *mut *mut int sm_ll_lookup_bitmap(struct ll_disk ll, dm_block_t b, uint32_t,
    pub result): *mut *mut int sm_ll_lookup(struct ll_disk ll, dm_block_t b, uint32_t,
    pub result): *mut dm_block_t end, dm_block_t,
    pub result): *mut dm_block_t begin, dm_block_t end, dm_block_t,
//
// The next three functions return (via nr_allocations) the net number of
// allocations that were made.  This number may be negative if there were
// more frees than allocs.
//
    pub nr_allocations): *mut *mut int sm_ll_insert(struct ll_disk ll, dm_block_t b, uint32_t ref_count, int32_t,
    pub nr_allocations): *mut *mut int sm_ll_inc(struct ll_disk ll, dm_block_t b, dm_block_t e, int32_t,
    pub nr_allocations): *mut *mut int sm_ll_dec(struct ll_disk ll, dm_block_t b, dm_block_t e, int32_t,
    pub ll): *mut int sm_ll_commit(struct ll_disk,
    pub tm): *mut *mut int sm_ll_new_metadata(struct ll_disk ll, struct dm_transaction_manager,
    pub len): *mut *mut void root_le, size_t,
    pub tm): *mut *mut int sm_ll_new_disk(struct ll_disk ll, struct dm_transaction_manager,
    pub len): *mut *mut void root_le, size_t,
// ----------------------------------------------------------------
