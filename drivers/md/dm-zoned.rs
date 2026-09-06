//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-zoned.h
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
// Copyright (C) 2017 Western Digital Corporation or its affiliates.
//
// This file is released under the GPL.
//

//
// dm-zoned creates block devices with 4KB blocks, always.
//
pub const DMZ_BLOCK_SHIFT: c_int = 12;

//
// 4KB block <-> 512B sector conversion.
//

//
// Zoned block device information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmz_dev {
    pub bdev: *mut block_device,
    pub metadata: *mut dmz_metadata,
    pub reclaim: *mut dmz_reclaim,
    pub uuid: uuid_t,
    pub capacity: sector_t,
    pub dev_idx: c_uint,
    pub nr_zones: c_uint,
    pub zone_offset: c_uint,
    pub flags: c_uint,
    pub zone_nr_sectors: sector_t,
    pub nr_rnd: c_uint,
    pub unmap_nr_rnd: core::sync::atomic::AtomicI32,
    pub unmap_rnd_list: list_head,
    pub map_rnd_list: list_head,
    pub nr_seq: c_uint,
    pub unmap_nr_seq: core::sync::atomic::AtomicI32,
    pub unmap_seq_list: list_head,
    pub map_seq_list: list_head,
}

// Device flags.

//
// Zone descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_zone {
// For listing the zone depending on its state
    pub link: list_head,
// Device containing this zone
    pub dev: *mut dmz_dev,
// Zone type and state
    pub flags: c_ulong,
// Zone activation reference count
    pub refcount: core::sync::atomic::AtomicI32,
// Zone id
    pub id: c_uint,
// Zone write pointer block (relative to the zone start block)
    pub wp_block: c_uint,
// Zone weight (number of valid blocks in the zone)
    pub weight: c_uint,
// The chunk that the zone maps
    pub chunk: c_uint,
//
// For a sequential data zone, pointer to the random zone
// used as a buffer for processing unaligned writes.
// For a buffer zone, this points back to the data zone.
//
    pub bzone: *mut dm_zone,
}

//
// Zone flags.
//
// Zone write type
// Zone critical condition
// How the zone is being used
// Zone internal state
//
// Zone data accessors.
//

//
// Message functions.
//

//
// Functions defined in dm-zoned-metadata.c
//
extern "C" {
    pub fn dmz_dtr_metadata(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_lock_map(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_unlock_map(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_lock_metadata(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_unlock_metadata(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_lock_flush(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_unlock_flush(zmd: *mut dmz_metadata);
}
extern "C" {
    pub fn dmz_flush_metadata(zmd: *mut dmz_metadata) -> c_int;
}
extern "C" {
    pub fn dmz_start_sect(zmd: *mut dmz_metadata, zone: *mut dm_zone) -> sector_t;
}
extern "C" {
    pub fn dmz_start_block(zmd: *mut dmz_metadata, zone: *mut dm_zone) -> sector_t;
}
extern "C" {
    pub fn dmz_nr_chunks(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_check_dev(zmd: *mut dmz_metadata) -> bool;
}
extern "C" {
    pub fn dmz_dev_is_dying(zmd: *mut dmz_metadata) -> bool;
}
pub const DMZ_ALLOC_RND: c_uint = 0x01;
pub const DMZ_ALLOC_CACHE: c_uint = 0x02;
pub const DMZ_ALLOC_SEQ: c_uint = 0x04;
pub const DMZ_ALLOC_RECLAIM: c_uint = 0x10;
extern "C" {
    pub fn dmz_free_zone(zmd: *mut dmz_metadata, zone: *mut dm_zone);
}
extern "C" {
    pub fn dmz_unmap_zone(zmd: *mut dmz_metadata, zone: *mut dm_zone);
}
extern "C" {
    pub fn dmz_nr_zones(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_cache_zones(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_unmap_cache_zones(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_rnd_zones(zmd: *mut dmz_metadata, idx: c_int) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_unmap_rnd_zones(zmd: *mut dmz_metadata, idx: c_int) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_seq_zones(zmd: *mut dmz_metadata, idx: c_int) -> c_uint;
}
extern "C" {
    pub fn dmz_nr_unmap_seq_zones(zmd: *mut dmz_metadata, idx: c_int) -> c_uint;
}
extern "C" {
    pub fn dmz_zone_nr_blocks(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_zone_nr_sectors(zmd: *mut dmz_metadata) -> c_uint;
}
extern "C" {
    pub fn dmz_zone_nr_sectors_shift(zmd: *mut dmz_metadata) -> c_uint;
}
//
// Activate a zone (increment its reference count).
//
extern "C" {
    pub fn dmz_lock_zone_reclaim(zone: *mut dm_zone) -> c_int;
}
extern "C" {
    pub fn dmz_unlock_zone_reclaim(zone: *mut dm_zone);
}
extern "C" {
    pub fn dmz_put_chunk_mapping(zmd: *mut dmz_metadata, zone: *mut dm_zone);
}
//
// Functions defined in dm-zoned-reclaim.c
//
extern "C" {
    pub fn dmz_ctr_reclaim(zmd: *mut dmz_metadata, zrc: *mut dmz_reclaim, idx: c_int) -> c_int;
}
extern "C" {
    pub fn dmz_dtr_reclaim(zrc: *mut dmz_reclaim);
}
extern "C" {
    pub fn dmz_suspend_reclaim(zrc: *mut dmz_reclaim);
}
extern "C" {
    pub fn dmz_resume_reclaim(zrc: *mut dmz_reclaim);
}
extern "C" {
    pub fn dmz_reclaim_bio_acc(zrc: *mut dmz_reclaim);
}
extern "C" {
    pub fn dmz_schedule_reclaim(zrc: *mut dmz_reclaim);
}
//
// Functions defined in dm-zoned-target.c
//
extern "C" {
    pub fn dmz_bdev_is_dying(dmz_dev: *mut dmz_dev) -> bool;
}
extern "C" {
    pub fn dmz_check_bdev(dmz_dev: *mut dmz_dev) -> bool;
}
//
// Deactivate a zone. This decrement the zone reference counter
// indicating that all BIOs to the zone have completed when the count is 0.
//
// Test if a zone is active, that is, has a refcount > 0.
//
extern "C" {
    pub fn atomic_read(_arg: &zone->refcount) -> return;
}
