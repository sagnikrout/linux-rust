//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/logical-zone.h
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
pub struct logical_zone {
// The completion for flush notifications
    pub completion: vdo_completion,
// The owner of this zone
    pub zones: *mut logical_zones,
// Which logical zone this is
    pub zone_number: zone_count_t,
// The thread id for this zone
    pub thread_id: thread_id_t,
// In progress operations keyed by LBN
    pub lbn_operations: *mut int_map,
// The logical to physical map
    pub block_map_zone: *mut block_map_zone,
// The current flush generation
    pub flush_generation: sequence_number_t,
//
// The oldest active generation in this zone. This is mutated only on the logical zone
// thread but is queried from the flusher thread.
//
    pub oldest_active_generation: sequence_number_t,
// The number of IOs in the current flush generation
    pub ios_in_flush_generation: block_count_t,
// The youngest generation of the current notification
    pub notification_generation: sequence_number_t,
// Whether a notification is in progress
    pub notifying: bool,
// The queue of active data write VIOs
    pub write_vios: list_head,
// The administrative state of the zone
    pub state: admin_state,
// The physical zone from which to allocate
    pub allocation_zone: *mut physical_zone,
// The number of allocations done from the current allocation_zone
    pub allocation_count: block_count_t,
// The next zone
    pub next: *mut logical_zone,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logical_zones {
// The vdo whose zones these are
    pub vdo: *mut vdo,
// The manager for administrative actions
    pub manager: *mut action_manager,
// The number of zones
    pub zone_count: zone_count_t,
// The logical zones themselves
    pub __counted_by(zone_count): logical_zone zones[],
}

extern "C" {
    pub fn vdo_free_logical_zones(zones: *mut logical_zones);
}
extern "C" {
    pub fn vdo_acquire_flush_generation_lock(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_release_flush_generation_lock(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_get_next_allocation_zone(zone: *mut logical_zone) -> *mut physical_zone  __must_check;
}
extern "C" {
    pub fn vdo_dump_logical_zone(zone: *const logical_zone);
}
