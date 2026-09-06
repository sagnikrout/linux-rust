//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/open-chapter.h
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
// The open chapter tracks the newest records in memory. Like the index as a whole, each open
// chapter is divided into a number of independent zones which are interleaved when the chapter is
// committed to the volume.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_chapter_zone_slot {
// If non-zero, the record number addressed by this hash slot
    pub OPEN_CHAPTER_RECORD_NUMBER_BITS: unsigned int record_number :,
// If true, the record at the index of this hash slot was deleted
    pub 1: bool deleted :,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_chapter_zone {
// The maximum number of records that can be stored
    pub capacity: c_uint,
// The number of records stored
    pub size: c_uint,
// The number of deleted records
    pub deletions: c_uint,
// Array of chunk records, 1-based
    pub records: *mut uds_volume_record,
// The number of slots in the hash table
    pub slot_count: c_uint,
// The hash table slots, referencing virtual record numbers
    pub __counted_by(slot_count): open_chapter_zone_slot slots[],
}

extern "C" {
    pub fn uds_reset_open_chapter(open_chapter: *mut open_chapter_zone);
}
extern "C" {
    pub fn uds_free_open_chapter(open_chapter: *mut open_chapter_zone);
}
extern "C" {
    pub fn uds_compute_saved_open_chapter_size(geometry: *const index_geometry) -> u64;
}
