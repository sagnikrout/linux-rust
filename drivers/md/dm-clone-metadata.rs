//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-clone-metadata.h
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
// Copyright (C) 2019 Arrikto, Inc. All Rights Reserved.
//

//
// The metadata device is currently limited in size.
//

//
// A metadata device larger than 16GB triggers a warning.
//

pub const SPACE_MAP_ROOT_SIZE: c_int = 128;
// dm-clone metadata
//
// Set region status to hydrated.
//
// @cmd: The dm-clone metadata
// @region_nr: The region number
//
// This function doesn't block, so it's safe to call it from interrupt context.
//
extern "C" {
    pub fn dm_clone_set_region_hydrated(cmd: *mut dm_clone_metadata, region_nr: c_ulong) -> c_int;
}
//
// Set status of all regions in the provided range to hydrated, if not already
// hydrated.
//
// @cmd: The dm-clone metadata
// @start: Starting region number
// @nr_regions: Number of regions in the range
//
// This function doesn't block, but since it uses spin_lock_irq()/spin_unlock_irq()
// it's NOT safe to call it from any context where interrupts are disabled, e.g.,
// from interrupt context.
//
// Read existing or create fresh metadata.
//
// @bdev: The device storing the metadata
// @target_size: The target size
// @region_size: The region size
//
// @returns: The dm-clone metadata
//
// This function reads the superblock of @bdev and checks if it's all zeroes.
// If it is, it formats @bdev and creates fresh metadata. If it isn't, it
// validates the metadata stored in @bdev.
//
// Free the resources related to metadata management.
//
extern "C" {
    pub fn dm_clone_metadata_close(cmd: *mut dm_clone_metadata);
}
//
// Commit dm-clone metadata to disk.
//
// We use a two phase commit:
//
// 1. dm_clone_metadata_pre_commit(): Prepare the current transaction for
// committing. After this is called, all subsequent metadata updates, done
// through either dm_clone_set_region_hydrated() or
// dm_clone_cond_set_range(), will be part of the **next** transaction.
//
// 2. dm_clone_metadata_commit(): Actually commit the current transaction to
// disk and start a new transaction.
//
// This allows dm-clone to flush the destination device after step (1) to
// ensure that all freshly hydrated regions, for which we are updating the
// metadata, are properly written to non-volatile storage and won't be lost in
// case of a crash.
//
extern "C" {
    pub fn dm_clone_metadata_pre_commit(cmd: *mut dm_clone_metadata) -> c_int;
}
extern "C" {
    pub fn dm_clone_metadata_commit(cmd: *mut dm_clone_metadata) -> c_int;
}
//
// Reload the in core copy of the on-disk bitmap.
//
// This should be used after aborting a metadata transaction and setting the
// metadata to read-only, to invalidate the in-core cache and make it match the
// on-disk metadata.
//
// WARNING: It must not be called concurrently with either
// dm_clone_set_region_hydrated() or dm_clone_cond_set_range(), as it updates
// the region bitmap without taking the relevant spinlock. We don't take the
// spinlock because dm_clone_reload_in_core_bitset() does I/O, so it may block.
//
// But, it's safe to use it after calling dm_clone_metadata_set_read_only(),
// because the latter sets the metadata to read-only mode. Both
// dm_clone_set_region_hydrated() and dm_clone_cond_set_range() refuse to touch
// the region bitmap, after calling dm_clone_metadata_set_read_only().
//
extern "C" {
    pub fn dm_clone_reload_in_core_bitset(cmd: *mut dm_clone_metadata) -> c_int;
}
//
// Check whether dm-clone's metadata changed this transaction.
//
extern "C" {
    pub fn dm_clone_changed_this_transaction(cmd: *mut dm_clone_metadata) -> bool;
}
//
// Abort current metadata transaction and rollback metadata to the last
// committed transaction.
//
extern "C" {
    pub fn dm_clone_metadata_abort(cmd: *mut dm_clone_metadata) -> c_int;
}
//
// Switches metadata to a read only mode. Once read-only mode has been entered
// the following functions will return -EPERM:
//
// dm_clone_metadata_pre_commit()
// dm_clone_metadata_commit()
// dm_clone_set_region_hydrated()
// dm_clone_cond_set_range()
// dm_clone_metadata_abort()
//
extern "C" {
    pub fn dm_clone_metadata_set_read_only(cmd: *mut dm_clone_metadata);
}
extern "C" {
    pub fn dm_clone_metadata_set_read_write(cmd: *mut dm_clone_metadata);
}
//
// Returns true if the hydration of the destination device is finished.
//
extern "C" {
    pub fn dm_clone_is_hydration_done(cmd: *mut dm_clone_metadata) -> bool;
}
//
// Returns true if region @region_nr is hydrated.
//
extern "C" {
    pub fn dm_clone_is_region_hydrated(cmd: *mut dm_clone_metadata, region_nr: c_ulong) -> bool;
}
//
// Returns true if all the regions in the range are hydrated.
//
// Returns the number of hydrated regions.
//
extern "C" {
    pub fn dm_clone_nr_of_hydrated_regions(cmd: *mut dm_clone_metadata) -> c_uint;
}
//
// Returns the first unhydrated region with region_nr >= @start
//
// Get the number of free metadata blocks.
//
extern "C" {
    pub fn dm_clone_get_free_metadata_block_count(cmd: *mut dm_clone_metadata, result: *mut dm_block_t) -> c_int;
}
//
// Get the total number of metadata blocks.
//
extern "C" {
    pub fn dm_clone_get_metadata_dev_size(cmd: *mut dm_clone_metadata, result: *mut dm_block_t) -> c_int;
}
