//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/dedupe.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dedupe_context {
    pub zone: *mut hash_zone,
    pub request: uds_request,
    pub list_entry: list_head,
    pub queue_entry: funnel_queue_entry,
    pub submission_jiffies: u64,
    pub requestor: *mut data_vio,
    pub state: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_zone {
// Which hash zone this is
    pub zone_number: zone_count_t,
// The administrative state of the zone
    pub state: admin_state,
// The thread ID for this zone
    pub thread_id: thread_id_t,
// Mapping from record name fields to hash_locks
    pub hash_lock_map: *mut int_map,
// List containing all unused hash_locks
    pub lock_pool: list_head,
//
// Statistics shared by all hash locks in this zone. Only modified on the hash zone thread,
// but queried by other threads.
//
    pub statistics: hash_lock_statistics,
// Array of all hash_locks
    pub lock_array: *mut hash_lock,
// These fields are used to manage the dedupe contexts
    pub available: list_head,
    pub pending: list_head,
    pub timed_out_complete: *mut funnel_queue,
    pub timer: timer_list,
    pub completion: vdo_completion,
    pub active: c_uint,
    pub timer_state: core::sync::atomic::AtomicI32,
// The dedupe contexts for querying the index from this zone
    pub contexts: [dedupe_context; MAXIMUM_VDO_USER_VIOS],
}

extern "C" {
    pub fn vdo_get_duplicate_lock(data_vio: *mut data_vio) -> *mut pbn_lock  __must_check;
}
extern "C" {
    pub fn vdo_acquire_hash_lock(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_continue_hash_lock(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_release_hash_lock(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_clean_failed_hash_lock(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_make_hash_zones(vdo: *mut vdo, zones_ptr: *mut hash_zones) -> int __must_check;
}
extern "C" {
    pub fn vdo_free_hash_zones(zones: *mut hash_zones);
}
extern "C" {
    pub fn vdo_drain_hash_zones(zones: *mut hash_zones, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_get_dedupe_statistics(zones: *mut hash_zones, stats: *mut vdo_statistics);
}
extern "C" {
    pub fn vdo_dump_hash_zones(zones: *mut hash_zones);
}
extern "C" {
    pub fn vdo_get_dedupe_index_timeout_count(zones: *mut hash_zones) -> u64;
}
extern "C" {
    pub fn vdo_message_dedupe_index(zones: *mut hash_zones, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn vdo_set_dedupe_state_normal(zones: *mut hash_zones);
}
extern "C" {
    pub fn vdo_start_dedupe_index(zones: *mut hash_zones, create_flag: bool);
}
extern "C" {
    pub fn vdo_resume_hash_zones(zones: *mut hash_zones, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_finish_dedupe_index(zones: *mut hash_zones);
}
// Interval (in milliseconds) from submission until switching to fast path and skipping UDS.
//
// Minimum time interval (in milliseconds) between timer invocations to check for requests waiting
// for UDS that should now time out.
//
extern "C" {
    pub fn vdo_set_dedupe_index_timeout_interval(value: c_uint);
}
extern "C" {
    pub fn vdo_set_dedupe_index_min_timer_interval(value: c_uint);
}
