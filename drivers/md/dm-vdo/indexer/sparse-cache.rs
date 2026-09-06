//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/sparse-cache.h
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
// Copyright 2023 Red Hat
//

//
// The sparse cache is a cache of entire chapter indexes from sparse chapters used for searching
// for names after all other search paths have failed. It contains only complete chapter indexes;
// record pages from sparse chapters and single index pages used for resolving hooks are kept in
// the regular page cache in the volume.
//
// The most important property of this cache is the absence of synchronization for read operations.
// Safe concurrent access to the cache by the zone threads is controlled by the triage queue and
// the barrier requests it issues to the zone queues. The set of cached chapters does not and must
// not change between the carefully coordinated calls to uds_update_sparse_cache() from the zone
// threads. Outside of updates, every zone will get the same result when calling
// uds_sparse_cache_contains() as every other zone.
//
extern "C" {
    pub fn uds_free_sparse_cache(cache: *mut sparse_cache);
}
extern "C" {
    pub fn uds_update_sparse_cache(zone: *mut index_zone, virtual_chapter: u64) -> int __must_check;
}
extern "C" {
    pub fn uds_invalidate_sparse_cache(cache: *mut sparse_cache);
}
