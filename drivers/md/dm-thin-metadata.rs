//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-thin-metadata.h
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
// Copyright (C) 2010-2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

//
// The metadata device is currently limited in size.
//

//
// A metadata device larger than 16GB triggers a warning.
//

// ----------------------------------------------------------------
//
// Thin metadata superblock flags.
//

//
// Device identifier
//
pub type dm_thin_id = u64;
//
// Reopens or creates a new, empty metadata volume.
//
extern "C" {
    pub fn dm_pool_metadata_close(pmd: *mut dm_pool_metadata) -> c_int;
}
//
// Compat feature flags.  Any incompat flags beyond the ones
// specified below will prevent use of the thin metadata.
//

//
// Device creation/deletion.
//
extern "C" {
    pub fn dm_pool_create_thin(pmd: *mut dm_pool_metadata, dev: dm_thin_id) -> c_int;
}
//
// An internal snapshot.
//
// You can only snapshot a quiesced origin i.e. one that is either
// suspended or not instanced at all.
//
// Deletes a virtual device from the metadata.  It _is_ safe to call this
// when that device is open.  Operations on that device will just start
// failing.  You still need to call close() on the device.
//
// Commits _all_ metadata changes: device creation, deletion, mapping
// updates.
//
extern "C" {
    pub fn dm_pool_commit_metadata(pmd: *mut dm_pool_metadata) -> c_int;
}
//
// Discards all uncommitted changes.  Rereads the superblock, rolling back
// to the last good transaction.  Thin devices remain open.
// dm_thin_aborted_changes() tells you if they had uncommitted changes.
//
// If this call fails it's only useful to call dm_pool_metadata_close().
// All other methods will fail with -EINVAL.
//
extern "C" {
    pub fn dm_pool_abort_metadata(pmd: *mut dm_pool_metadata) -> c_int;
}
//
// Set/get userspace transaction id.
//
// Hold/get root for userspace transaction.
//
// The metadata snapshot is a copy of the current superblock (minus the
// space maps).  Userland can access the data structures for READ
// operations only.  A small performance hit is incurred by providing this
// copy of the metadata to userland due to extra copy-on-write operations
// on the metadata nodes.  Release this as soon as you finish with it.
//
extern "C" {
    pub fn dm_pool_reserve_metadata_snap(pmd: *mut dm_pool_metadata) -> c_int;
}
extern "C" {
    pub fn dm_pool_release_metadata_snap(pmd: *mut dm_pool_metadata) -> c_int;
}
//
// Actions on a single virtual device.
//
// Opening the same device more than once will fail with -EBUSY.
//
extern "C" {
    pub fn dm_pool_close_thin_device(td: *mut dm_thin_device) -> c_int;
}
extern "C" {
    pub fn dm_thin_dev_id(td: *mut dm_thin_device) -> dm_thin_id;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_thin_lookup_result {
    pub block: dm_block_t,
    pub shared:1: bool,
}

//
// Returns:
// -EWOULDBLOCK iff @can_issue_io is set and would issue IO
// -ENODATA iff that mapping is not present.
// 0 success
//
// Retrieve the next run of contiguously mapped blocks.  Useful for working
// out where to break up IO.  Returns 0 on success, < 0 on error.
//
// Obtain an unused block.
//
extern "C" {
    pub fn dm_pool_alloc_data_block(pmd: *mut dm_pool_metadata, result: *mut dm_block_t) -> c_int;
}
//
// Insert or remove block.
//
// Queries.
//
extern "C" {
    pub fn dm_thin_changed_this_transaction(td: *mut dm_thin_device) -> bool;
}
extern "C" {
    pub fn dm_pool_changed_this_transaction(pmd: *mut dm_pool_metadata) -> bool;
}
extern "C" {
    pub fn dm_thin_aborted_changes(td: *mut dm_thin_device) -> bool;
}
extern "C" {
    pub fn dm_thin_get_mapped_count(td: *mut dm_thin_device, result: *mut dm_block_t) -> c_int;
}
extern "C" {
    pub fn dm_pool_get_data_dev_size(pmd: *mut dm_pool_metadata, result: *mut dm_block_t) -> c_int;
}
extern "C" {
    pub fn dm_pool_block_is_shared(pmd: *mut dm_pool_metadata, b: dm_block_t, result: *mut bool) -> c_int;
}
extern "C" {
    pub fn dm_pool_inc_data_range(pmd: *mut dm_pool_metadata, b: dm_block_t, e: dm_block_t) -> c_int;
}
extern "C" {
    pub fn dm_pool_dec_data_range(pmd: *mut dm_pool_metadata, b: dm_block_t, e: dm_block_t) -> c_int;
}
//
// Returns -ENOSPC if the new size is too small and already allocated
// blocks would be lost.
//
extern "C" {
    pub fn dm_pool_resize_data_dev(pmd: *mut dm_pool_metadata, new_size: dm_block_t) -> c_int;
}
extern "C" {
    pub fn dm_pool_resize_metadata_dev(pmd: *mut dm_pool_metadata, new_size: dm_block_t) -> c_int;
}
//
// Flicks the underlying block manager into read only mode, so you know
// that nothing is changing.
//
extern "C" {
    pub fn dm_pool_metadata_read_only(pmd: *mut dm_pool_metadata);
}
extern "C" {
    pub fn dm_pool_metadata_read_write(pmd: *mut dm_pool_metadata);
}
//
// Updates the superblock immediately.
//
extern "C" {
    pub fn dm_pool_metadata_set_needs_check(pmd: *mut dm_pool_metadata) -> c_int;
}
extern "C" {
    pub fn dm_pool_metadata_needs_check(pmd: *mut dm_pool_metadata) -> bool;
}
//
// Issue any prefetches that may be useful.
//
extern "C" {
    pub fn dm_pool_issue_prefetches(pmd: *mut dm_pool_metadata);
}
// Pre-commit callback
extern "C" {
    pub fn int(context: *mut *mut dm_pool_pre_commit_fn)(void) -> typedef;
}
// ----------------------------------------------------------------
