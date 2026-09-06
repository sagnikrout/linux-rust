//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-cache-metadata.h
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
// Copyright (C) 2012 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------

// FIXME: remove this restriction
//
// The metadata device is currently limited in size.
//

//
// A metadata device larger than 16GB triggers a warning.
//

// ----------------------------------------------------------------
//
// Ext[234]-style compat feature flags.
//
// A new feature which old metadata will still be compatible with should
// define a DM_CACHE_FEATURE_COMPAT_* flag (rarely useful).
//
// A new feature that is not compatible with old code should define a
// DM_CACHE_FEATURE_INCOMPAT_* flag and guard the relevant code with
// that flag.
//
// A new feature that is not compatible with old code accessing the
// metadata RDWR should define a DM_CACHE_FEATURE_RO_COMPAT_* flag and
// guard the relevant code with that flag.
//
// As these various flags are defined they should be added to the
// following masks.
//

//
// Reopens or creates a new, empty metadata volume.  Returns an ERR_PTR on
// failure.  If reopening then features must match.
//
extern "C" {
    pub fn dm_cache_metadata_close(cmd: *mut dm_cache_metadata);
}
//
// The metadata needs to know how many cache blocks there are.  We don't
// care about the origin, assuming the core target is giving us valid
// origin blocks to map to.
//
extern "C" {
    pub fn dm_cache_resize(cmd: *mut dm_cache_metadata, new_cache_size: dm_cblock_t) -> c_int;
}
extern "C" {
    pub fn dm_cache_set_discard(cmd: *mut dm_cache_metadata, dblock: dm_dblock_t, discard: bool) -> c_int;
}
extern "C" {
    pub fn dm_cache_remove_mapping(cmd: *mut dm_cache_metadata, cblock: dm_cblock_t) -> c_int;
}
extern "C" {
    pub fn dm_cache_insert_mapping(cmd: *mut dm_cache_metadata, cblock: dm_cblock_t, oblock: dm_oblock_t) -> c_int;
}
extern "C" {
    pub fn dm_cache_changed_this_transaction(cmd: *mut dm_cache_metadata) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_cache_statistics {
    pub read_hits: u32,
    pub read_misses: u32,
    pub write_hits: u32,
    pub write_misses: u32,
}

//
// 'void' because it's no big deal if it fails.
//
extern "C" {
    pub fn dm_cache_commit(cmd: *mut dm_cache_metadata, clean_shutdown: bool) -> c_int;
}
//
// The policy is invited to save a 32bit hint value for every cblock (eg,
// for a hit count).  These are stored against the policy name.  If
// policies are changed, then hints will be lost.  If the machine crashes,
// hints will be lost.
//
// The hints are indexed by the cblock, but many policies will not
// necessarily have a fast way of accessing efficiently via cblock.  So
// rather than querying the policy for each cblock, we let it walk its data
// structures and fill in the hints in whatever order it wishes.
//
extern "C" {
    pub fn dm_cache_write_hints(cmd: *mut dm_cache_metadata, p: *mut dm_cache_policy) -> c_int;
}
extern "C" {
    pub fn dm_cache_metadata_needs_check(cmd: *mut dm_cache_metadata, result: *mut bool) -> c_int;
}
extern "C" {
    pub fn dm_cache_metadata_set_needs_check(cmd: *mut dm_cache_metadata) -> c_int;
}
extern "C" {
    pub fn dm_cache_metadata_set_read_only(cmd: *mut dm_cache_metadata);
}
extern "C" {
    pub fn dm_cache_metadata_set_read_write(cmd: *mut dm_cache_metadata);
}
extern "C" {
    pub fn dm_cache_metadata_abort(cmd: *mut dm_cache_metadata) -> c_int;
}
//
// Query method.  Was the metadata cleanly shut down when opened?
//
extern "C" {
    pub fn dm_cache_metadata_clean_when_opened(cmd: *mut dm_cache_metadata, result: *mut bool) -> c_int;
}
// ----------------------------------------------------------------
