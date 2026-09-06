//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/volume-index.h
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
// The volume index is the primary top-level index for UDS. It contains records which map a record
// name to the chapter where a record with that name is stored. This mapping can definitively say
// when no record exists. However, because we only use a subset of the name for this index, it
// cannot definitively say that a record for the entry does exist. It can only say that if a record
// exists, it will be in a particular chapter. The request can then be dispatched to that chapter
// for further processing.
//
// If the volume_index_record does not actually match the record name, the index can store a more
// specific collision record to disambiguate the new entry from the existing one. Index entries are
// managed with volume_index_record structures.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_index_stats {
// Nanoseconds spent rebalancing
    pub rebalance_time: ktime_t,
// Number of memory rebalances
    pub rebalance_count: u32,
// The number of records in the index
    pub record_count: u64,
// The number of collision records
    pub collision_count: u64,
// The number of records removed
    pub discard_count: u64,
// The number of UDS_OVERFLOWs detected
    pub overflow_count: u64,
// The number of delta lists
    pub delta_lists: u32,
// Number of early flushes
    pub early_flushes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_sub_index_zone {
    pub virtual_chapter_low: u64,
    pub virtual_chapter_high: u64,
    pub early_flushes: u64,
    pub __aligned(L1_CACHE_BYTES): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_sub_index {
// The delta index
    pub delta_index: delta_index,
// The first chapter to be flushed in each zone
    pub flush_chapters: *mut u64,
// The zones
    pub zones: *mut volume_sub_index_zone,
// The volume nonce
    pub volume_nonce: u64,
// Expected size of a chapter (per zone)
    pub chapter_zone_bits: u64,
// Maximum size of the index (per zone)
    pub max_zone_bits: u64,
// The number of bits in address mask
    pub address_bits: u8,
// Mask to get address within delta list
    pub address_mask: u32,
// The number of bits in chapter number
    pub chapter_bits: u8,
// The largest storable chapter number
    pub chapter_mask: u32,
// The number of chapters used
    pub chapter_count: u32,
// The number of delta lists
    pub list_count: u32,
// The number of zones
    pub zone_count: c_uint,
// The amount of memory allocated
    pub memory_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_index_zone {
// Protects the sampled index in this zone
    pub hook_mutex: mutex,
    pub __aligned(L1_CACHE_BYTES): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_index {
    pub sparse_sample_rate: u32,
    pub zone_count: c_uint,
    pub memory_size: u64,
    pub vi_non_hook: volume_sub_index,
    pub vi_hook: volume_sub_index,
    pub zones: *mut volume_index_zone,
}

//
// The volume_index_record structure is used to facilitate processing of a record name. A client
// first calls uds_get_volume_index_record() to find the volume index record for a record name. The
// fields of the record can then be examined to determine the state of the record.
//
// If is_found is false, then the index did not find an entry for the record name. Calling
// uds_put_volume_index_record() will insert a new entry for that name at the proper place.
//
// If is_found is true, then we did find an entry for the record name, and the virtual_chapter and
// is_collision fields reflect the entry found. Subsequently, a call to
// uds_remove_volume_index_record() will remove the entry, a call to
// uds_set_volume_index_record_chapter() will update the existing entry, and a call to
// uds_put_volume_index_record() will insert a new collision record after the existing entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_index_record {
// Public fields
// Chapter where the record info is found
    pub virtual_chapter: u64,
// This record is a collision
    pub is_collision: bool,
// This record is the requested record
    pub is_found: bool,
// Private fields
// Zone that contains this name
    pub zone_number: c_uint,
// The volume index
    pub sub_index: *mut volume_sub_index,
// Mutex for accessing this delta index entry in the hook index
    pub mutex: *mut mutex,
// The record name to which this record refers
    pub name: *const uds_record_name,
// The delta index entry for this record
    pub delta_entry: delta_index_entry,
}

extern "C" {
    pub fn uds_free_volume_index(volume_index: *mut volume_index);
}
//
// This function is only used to manage sparse cache membership. Most requests should use
// uds_get_volume_index_record() to look up index records instead.
//
extern "C" {
    pub fn uds_remove_volume_index_record(record: *mut volume_index_record) -> int __must_check;
}
