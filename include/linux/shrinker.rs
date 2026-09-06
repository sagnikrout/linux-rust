//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/shrinker.h
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
// Bitmap and deferred work of shrinker::id corresponding to memcg-aware
// shrinkers, which have elements charged to the memcg.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shrinker_info_unit {
    pub nr_deferred: [atomic_long_t; SHRINKER_UNIT_BITS],
    pub SHRINKER_UNIT_BITS): DECLARE_BITMAP(map,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shrinker_info {
    pub rcu: rcu_head,
    pub map_nr_max: c_int,
    pub unit: [*mut shrinker_info_unit; ],
}

//
// This struct is used to pass information from page reclaim to the shrinkers.
// We consolidate the values for easier extension later.
//
// The 'gfpmask' refers to the allocation we are currently trying to
// fulfil.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shrink_control {
    pub gfp_mask: gfp_t,
// current node being shrunk (for NUMA aware shrinkers)
    pub nid: c_int,
//
// How many objects scan_objects should scan and try to reclaim.
// This is reset before every call, so it is safe for callees
// to modify.
//
    pub nr_to_scan: c_ulong,
//
// How many objects did scan_objects process?
// This defaults to nr_to_scan before every call, but the callee
// should track its actual progress.
//
    pub nr_scanned: c_ulong,
// current memcg being shrunk (for memcg aware shrinkers)
    pub memcg: *mut mem_cgroup,
}

//
// A callback you can register to apply pressure to ageable caches.
//
// @count_objects should return the number of freeable items in the cache. If
// there are no objects to free, it should return SHRINK_EMPTY, while 0 is
// returned in cases of the number of freeable items cannot be determined
// or shrinker should skip this cache for this time (e.g., their number
// is below shrinkable limit). No deadlock checks should be done during the
// count callback - the shrinker relies on aggregating scan counts that couldn't
// be executed due to potential deadlocks to be run at a later call when the
// deadlock condition is no longer pending.
//
// @scan_objects will only be called if @count_objects returned a non-zero
// value for the number of freeable objects. The callout should scan the cache
// and attempt to free items from the cache. It should then return the number
// of objects freed during the scan, or SHRINK_STOP if progress cannot be made
// due to potential deadlocks. If SHRINK_STOP is returned, then no further
// attempts to call the @scan_objects will be made from the current reclaim
// context.
//
// @flags determine the shrinker abilities, like numa awareness
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shrinker {
    pub sc): *mut shrink_control,
    pub sc): *mut shrink_control,
    pub /: *mut *mut long batch; / reclaim batch size, 0 = default,
    pub /: *mut *mut int seeks; / seeks to recreate an obj,
    pub flags: unsigned,
//
// The reference count of this shrinker. Registered shrinker have an
// initial refcount of 1, then the lookup operations are now allowed
// to use it via shrinker_try_get(). Later in the unregistration step,
// the initial refcount will be discarded, and will free the shrinker
// asynchronously via RCU after its refcount reaches 0.
//
    pub refcount: refcount_t,
    pub /: *mut *mut completion done; / use to wait for refcount to reach 0,
    pub rcu: rcu_head,
    pub private_data: *mut c_void,
// These are for internal use
    pub list: list_head,

// ID in shrinker_idr
    pub id: c_int,

    pub debugfs_id: c_int,
    pub name: *const c_char,
    pub debugfs_entry: *mut dentry,

// objs pending delete, per node
    pub nr_deferred: *mut atomic_long_t,
}

// Internal flags

// Flags for users to use

//
// It just makes sense when the shrinker is also MEMCG_AWARE for now,
// non-MEMCG_AWARE shrinker should not have this flag set.
//

extern "C" {
    pub fn shrinker_register(shrinker: *mut shrinker);
}
extern "C" {
    pub fn shrinker_free(shrinker: *mut shrinker);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &shrinker->refcount) -> return;
}

