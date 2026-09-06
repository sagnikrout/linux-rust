//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-exception-store.h
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
// Copyright (C) 2001-2002 Sistina Software (UK) Limited.
// Copyright (C) 2008 Red Hat, Inc. All rights reserved.
//
// Device-mapper snapshot exception store.
//
// This file is released under the GPL.
//

//
// The snapshot code deals with largish chunks of the disk at a
// time. Typically 32k - 512k.
//
pub type chunk_t = sector_t;
//
// An exception is used where an old chunk of data has been
// replaced by a new one.
// If chunk_t is 64 bits in size, the top 8 bits of new_chunk hold the number
// of chunks that follow contiguously.  Remaining bits hold the number of the
// chunk within the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_exception {
    pub hash_list: hlist_node,
    pub old_chunk: chunk_t,
    pub new_chunk: chunk_t,
}

//
// Abstraction to handle the meta/layout of exception stores (the
// COW device).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_exception_store_type {
    pub name: *const c_char,
    pub module: *mut module,
    pub options): *mut *mut *mut int (ctr)(struct dm_exception_store store, char,
//
// Destroys this object when you've finished with it.
//
    pub store): *mut *mut void (dtr)(struct dm_exception_store,
//
// The target shouldn't read the COW device until this is
// called.  As exceptions are read from the COW, they are
// reported back via the callback.
//
    pub callback_context): *mut c_void,
//
// Find somewhere to store the next exception.
//
    pub e): *mut dm_exception,
//
// Update the metadata with this exception.
//
    pub callback_context): *mut c_void,
//
// Returns 0 if the exception store is empty.
//
// If there are exceptions still to be merged, sets
// *last_old_chunk and *last_new_chunk to the most recent
// still-to-be-merged chunk and returns the number of
// consecutive previous ones.
//
    pub last_new_chunk): *mut *mut chunk_t last_old_chunk, chunk_t,
//
// Clear the last n exceptions.
// nr_merged must be <= the value returned by prepare_merge.
//
    pub nr_merged): *mut *mut *mut int (commit_merge)(struct dm_exception_store store, int,
//
// The snapshot is invalid, note this in the metadata.
//
    pub store): *mut *mut void (drop_snapshot)(struct dm_exception_store,
    pub maxlen): c_uint,
//
// Return how full the snapshot is.
//
    pub metadata_sectors): *mut sector_t,
// For internal device-mapper use only.
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_exception_store {
    pub type: *mut dm_exception_store_type,
    pub snap: *mut dm_snapshot,
// Size of data blocks saved - must be a power of 2
    pub chunk_size: c_uint,
    pub chunk_mask: c_uint,
    pub chunk_shift: c_uint,
    pub context: *mut c_void,
    pub userspace_supports_overflow: bool,
}

//
// Obtain the origin or cow device used by a given snapshot.
//
// Funtions to manipulate consecutive chunks
//
pub const DM_CHUNK_CONSECUTIVE_BITS: c_int = 8;
pub const DM_CHUNK_NUMBER_BITS: c_int = 56;
//
// Return the number of sectors in the device.
//
extern "C" {
    pub fn bdev_nr_sectors(_arg: bdev) -> return;
}
extern "C" {
    pub fn dm_exception_store_type_register(type: *mut dm_exception_store_type) -> c_int;
}
extern "C" {
    pub fn dm_exception_store_type_unregister(type: *mut dm_exception_store_type) -> c_int;
}
extern "C" {
    pub fn dm_exception_store_destroy(store: *mut dm_exception_store);
}
extern "C" {
    pub fn dm_exception_store_init() -> c_int;
}
extern "C" {
    pub fn dm_exception_store_exit();
}
//
// Two exception store implementations.
//
extern "C" {
    pub fn dm_persistent_snapshot_init() -> c_int;
}
extern "C" {
    pub fn dm_persistent_snapshot_exit();
}
extern "C" {
    pub fn dm_transient_snapshot_init() -> c_int;
}
extern "C" {
    pub fn dm_transient_snapshot_exit();
}
