//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/physical-zone.h
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
// The type of a PBN lock.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pbn_lock_type {
    VIO_READ_LOCK,
    VIO_WRITE_LOCK,
    VIO_BLOCK_MAP_WRITE_LOCK,
}

//
// A PBN lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pbn_lock {
// The implementation of the lock
    pub implementation: *const pbn_lock_implementation,
// The number of VIOs holding or sharing this lock
    pub holder_count: data_vio_count_t,
//
// The number of compressed block writers holding a share of this lock while they are
// acquiring a reference to the PBN.
//
    pub fragment_locks: u8,
// Whether the locked PBN has been provisionally referenced on behalf of the lock holder.
    pub has_provisional_reference: bool,
//
// For read locks, the number of references that were known to be available on the locked
// block at the time the lock was acquired.
//
    pub increment_limit: u8,
//
// For read locks, the number of data_vios that have tried to claim one of the available
// increments during the lifetime of the lock. Each claim will first increment this
// counter, so it can exceed the increment limit.
//
    pub increments_claimed: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physical_zone {
// Which physical zone this is
    pub zone_number: zone_count_t,
// The thread ID for this zone
    pub thread_id: thread_id_t,
// In progress operations keyed by PBN
    pub pbn_operations: *mut int_map,
// Pool of unused pbn_lock instances
    pub lock_pool: *mut pbn_lock_pool,
// The block allocator for this zone
    pub allocator: *mut block_allocator,
// The next zone from which to attempt an allocation
    pub next: *mut physical_zone,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct physical_zones {
// The number of zones
    pub zone_count: zone_count_t,
// The physical zones themselves
    pub zones: [physical_zone; ],
}

extern "C" {
    pub fn vdo_is_pbn_read_lock(lock: *const pbn_lock) -> bool __must_check;
}
extern "C" {
    pub fn vdo_downgrade_pbn_write_lock(lock: *mut pbn_lock, compressed_write: bool);
}
extern "C" {
    pub fn vdo_claim_pbn_lock_increment(lock: *mut pbn_lock) -> bool __must_check;
}
//
// vdo_pbn_lock_has_provisional_reference() - Check whether a PBN lock has a provisional reference.
// @lock: The PBN lock.
//
extern "C" {
    pub fn vdo_assign_pbn_lock_provisional_reference(lock: *mut pbn_lock);
}
extern "C" {
    pub fn vdo_unassign_pbn_lock_provisional_reference(lock: *mut pbn_lock);
}
extern "C" {
    pub fn vdo_free_physical_zones(zones: *mut physical_zones);
}
extern "C" {
    pub fn vdo_allocate_block_in_zone(data_vio: *mut data_vio) -> bool __must_check;
}
extern "C" {
    pub fn vdo_dump_physical_zone(zone: *const physical_zone);
}
