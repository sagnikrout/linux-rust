//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm-region-hash.h
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
// Copyright (C) 2003 Sistina Software Limited.
// Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
//
// Device-Mapper dirty region hash interface.
//
// This file is released under the GPL.
//

//
// ----------------------------------------------------------------
// Region hash
// ----------------------------------------------------------------
//
// States a region can have.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_rh_region_states {
    DM_RH_CLEAN	 = 0x01,	/* No writes in flight. */
    DM_RH_DIRTY	 = 0x02,	/* Writes in flight. */
    DM_RH_NOSYNC	 = 0x04,	/* Out of sync. */
    DM_RH_RECOVERING = 0x08,	/* Under resynchronization. */
}

//
// Region hash create/destroy.
//
extern "C" {
    pub fn dm_region_hash_destroy(rh: *mut dm_region_hash);
}
//
// Conversion functions.
//
extern "C" {
    pub fn dm_rh_bio_to_region(rh: *mut dm_region_hash, bio: *mut bio) -> region_t;
}
extern "C" {
    pub fn dm_rh_region_to_sector(rh: *mut dm_region_hash, region: region_t) -> sector_t;
}
//
// Get region size and key (ie. number of the region).
//
extern "C" {
    pub fn dm_rh_get_region_size(rh: *mut dm_region_hash) -> sector_t;
}
extern "C" {
    pub fn dm_rh_get_region_key(reg: *mut dm_region) -> region_t;
}
//
// Get/set/update region state (and dirty log).
//
extern "C" {
    pub fn dm_rh_get_state(rh: *mut dm_region_hash, region: region_t, may_block: c_int) -> c_int;
}
// Non-zero errors_handled leaves the state of the region NOSYNC
extern "C" {
    pub fn dm_rh_update_states(rh: *mut dm_region_hash, errors_handled: c_int);
}
// Flush the region hash and dirty log.
extern "C" {
    pub fn dm_rh_flush(rh: *mut dm_region_hash) -> c_int;
}
// Inc/dec pending count on regions.
extern "C" {
    pub fn dm_rh_inc_pending(rh: *mut dm_region_hash, bios: *mut bio_list);
}
extern "C" {
    pub fn dm_rh_dec(rh: *mut dm_region_hash, region: region_t);
}
// Delay bios on regions.
extern "C" {
    pub fn dm_rh_delay(rh: *mut dm_region_hash, bio: *mut bio);
}
extern "C" {
    pub fn dm_rh_mark_nosync(rh: *mut dm_region_hash, bio: *mut bio);
}
//
// Region recovery control.
//
// Prepare some regions for recovery by starting to quiesce them.
extern "C" {
    pub fn dm_rh_recovery_prepare(rh: *mut dm_region_hash);
}
// Try fetching a quiesced region for recovery.
// Report recovery end on a region.
extern "C" {
    pub fn dm_rh_recovery_end(reg: *mut dm_region, error: c_int);
}
// Returns number of regions with recovery work outstanding.
extern "C" {
    pub fn dm_rh_recovery_in_flight(rh: *mut dm_region_hash) -> c_int;
}
// Start/stop recovery.
extern "C" {
    pub fn dm_rh_start_recovery(rh: *mut dm_region_hash);
}
extern "C" {
    pub fn dm_rh_stop_recovery(rh: *mut dm_region_hash);
}
