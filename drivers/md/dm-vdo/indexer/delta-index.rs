//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/delta-index.h
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
// A delta index is a key-value store, where each entry maps an address (the key) to a payload (the
// value). The entries are sorted by address, and only the delta between successive addresses is
// stored in the entry. The addresses are assumed to be uniformly distributed, and the deltas are
// therefore exponentially distributed.
//
// A delta_index can either be mutable or immutable depending on its expected use. The immutable
// form of a delta index is used for the indexes of closed chapters committed to the volume. The
// mutable form of a delta index is used by the volume index, and also by the chapter index in an
// open chapter. Like the index as a whole, each mutable delta index is divided into a number of
// independent zones.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_list {
// The offset of the delta list start, in bits
    pub start: u64,
// The number of bits in the delta list
    pub size: u16,
// Where the last search "found" the key, in bits
    pub save_offset: u16,
// The key for the record just before save_offset
    pub save_key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_zone {
// The delta list memory
    pub memory: *mut u8,
// The delta list headers
    pub delta_lists: *mut delta_list,
// Temporary starts of delta lists
    pub new_offsets: *mut u64,
// Buffered writer for saving an index
    pub buffered_writer: *mut buffered_writer,
// The size of delta list memory
    pub size: usize,
// Nanoseconds spent rebalancing
    pub rebalance_time: ktime_t,
// Number of memory rebalances
    pub rebalance_count: u32,
// The number of bits in a stored value
    pub value_bits: u8,
// The number of bits in the minimal key code
    pub min_bits: u16,
// The number of keys used in a minimal code
    pub min_keys: u32,
// The number of keys used for another code bit
    pub incr_keys: u32,
// The number of records in the index
    pub record_count: u64,
// The number of collision records
    pub collision_count: u64,
// The number of records removed
    pub discard_count: u64,
// The number of UDS_OVERFLOW errors detected
    pub overflow_count: u64,
// The index of the first delta list
    pub first_list: u32,
// The number of delta lists
    pub list_count: u32,
// Tag belonging to this delta index
    pub tag: u8,
    pub __aligned(L1_CACHE_BYTES): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_list_save_info {
// Tag identifying which delta index this list is in
    pub tag: u8,
// Bit offset of the start of the list data
    pub bit_offset: u8,
// Number of bytes of list data
    pub byte_count: u16,
// The delta list number within the delta index
    pub index: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_index {
// The zones
    pub delta_zones: *mut delta_zone,
// The number of zones
    pub zone_count: c_uint,
// The number of delta lists
    pub list_count: u32,
// Maximum lists per zone
    pub lists_per_zone: u32,
// Total memory allocated to this index
    pub memory_size: usize,
// The number of non-empty lists at load time per zone
    pub load_lists: [u32; MAX_ZONES],
// True if this index is mutable
    pub mutable: bool,
// Tag belonging to this delta index
    pub tag: u8,
}

//
// A delta_index_page describes a single page of a chapter index. The delta_index field allows the
// page to be treated as an immutable delta_index. We use the delta_zone field to treat the chapter
// index page as a single zone index, and without the need to do an additional memory allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_index_page {
    pub delta_index: delta_index,
// These values are loaded from the delta_page_header
    pub lowest_list_number: u32,
    pub highest_list_number: u32,
    pub virtual_chapter_number: u64,
// This structure describes the single zone of a delta index page.
    pub delta_zone: delta_zone,
}

//
// Notes on the delta_index_entries:
//
// The fields documented as "public" can be read by any code that uses a delta_index. The fields
// documented as "private" carry information between delta_index method calls and should not be
// used outside the delta_index module.
//
// (1) The delta_index_entry is used like an iterator when searching a delta list.
//
// (2) It is also the result of a successful search and can be used to refer to the element found
// by the search.
//
// (3) It is also the result of an unsuccessful search and can be used to refer to the insertion
// point for a new record.
//
// (4) If at_end is true, the delta_list entry can only be used as the insertion point for a new
// record at the end of the list.
//
// (5) If at_end is false and is_collision is true, the delta_list entry fields refer to a
// collision entry in the list, and the delta_list entry can be used as a reference to this
// entry.
//
// (6) If at_end is false and is_collision is false, the delta_list entry fields refer to a
// non-collision entry in the list. Such delta_list entries can be used as a reference to a
// found entry, or an insertion point for a non-collision entry before this entry, or an
// insertion point for a collision entry that collides with this entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_index_entry {
// Public fields
// The key for this entry
    pub key: u32,
// We are after the last list entry
    pub at_end: bool,
// This record is a collision
    pub is_collision: bool,
// Private fields
// This delta list overflowed
    pub list_overflow: bool,
// The number of bits used for the value
    pub value_bits: u8,
// The number of bits used for the entire entry
    pub entry_bits: u16,
// The delta index zone
    pub delta_zone: *mut delta_zone,
// The delta list containing the entry
    pub delta_list: *mut delta_list,
// The delta list number
    pub list_number: u32,
// Bit offset of this entry within the list
    pub offset: u16,
// The delta between this and previous entry
    pub delta: u32,
// Temporary delta list for immutable indices
    pub temp_delta_list: delta_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_index_stats {
// Number of bytes allocated
    pub memory_allocated: usize,
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
// The number of UDS_OVERFLOW errors detected
    pub overflow_count: u64,
// The number of delta lists
    pub list_count: u32,
}

extern "C" {
    pub fn uds_uninitialize_delta_index(delta_index: *mut delta_index);
}
extern "C" {
    pub fn uds_reset_delta_index(delta_index: *const delta_index);
}
extern "C" {
    pub fn uds_write_guard_delta_list(buffered_writer: *mut buffered_writer) -> int __must_check;
}
extern "C" {
    pub fn uds_next_delta_index_entry(delta_entry: *mut delta_index_entry) -> int __must_check;
}
extern "C" {
    pub fn uds_remember_delta_index_offset(delta_entry: *const delta_index_entry) -> int __must_check;
}
extern "C" {
    pub fn uds_get_delta_entry_value(delta_entry: *const delta_index_entry) -> u32 __must_check;
}
extern "C" {
    pub fn uds_set_delta_entry_value(delta_entry: *const delta_index_entry, value: u32) -> int __must_check;
}
extern "C" {
    pub fn uds_remove_delta_index_entry(delta_entry: *mut delta_index_entry) -> int __must_check;
}
extern "C" {
    pub fn uds_log_delta_index_entry(delta_entry: *mut delta_index_entry);
}
