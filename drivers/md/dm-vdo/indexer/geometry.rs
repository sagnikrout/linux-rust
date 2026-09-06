//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/geometry.h
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
// The index_geometry records parameters that define the layout of a UDS index volume, and the size and
// shape of various index structures. It is created when the index is created, and is referenced by
// many index sub-components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_geometry {
// Size of a chapter page, in bytes
    pub bytes_per_page: usize,
// Number of record pages in a chapter
    pub record_pages_per_chapter: u32,
// Total number of chapters in a volume
    pub chapters_per_volume: u32,
// Number of sparsely-indexed chapters in a volume
    pub sparse_chapters_per_volume: u32,
// Number of bits used to determine delta list numbers
    pub chapter_delta_list_bits: u8,
// Virtual chapter remapped from physical chapter 0
    pub remapped_virtual: u64,
// New physical chapter where the remapped chapter can be found
    pub remapped_physical: u64,
//
// The following properties are derived from the ones above, but they are computed and
// recorded as fields for convenience.
//
// Total number of pages in a volume, excluding the header
    pub pages_per_volume: u32,
// Total number of bytes in a volume, including the header
    pub bytes_per_volume: usize,
// Number of pages in a chapter
    pub pages_per_chapter: u32,
// Number of index pages in a chapter index
    pub index_pages_per_chapter: u32,
// Number of records that fit on a page
    pub records_per_page: u32,
// Number of records that fit in a chapter
    pub records_per_chapter: u32,
// Number of records that fit in a volume
    pub records_per_volume: u64,
// Number of delta lists per chapter index
    pub delta_lists_per_chapter: u32,
// Mean delta for chapter indexes
    pub chapter_mean_delta: u32,
// Number of bits needed for record page numbers
    pub chapter_payload_bits: u8,
// Number of bits used to compute addresses for chapter delta lists
    pub chapter_address_bits: u8,
// Number of densely-indexed chapters in a volume
    pub dense_chapters_per_volume: u32,
}

// The number of bytes in a record (name + metadata)
// The default length of a page in a chapter, in bytes
// The default maximum number of records per page
// The default number of record pages in a chapter
// The default number of record pages in a chapter for a small index
// The default number of chapters in a volume
// The default number of sparsely-indexed chapters in a volume
// The log2 of the default mean delta
// The log2 of the number of delta lists in a large chapter
// The log2 of the number of delta lists in a small chapter
// The number of header pages per volume
//
// Check whether this geometry is reduced by a chapter. This will only be true if the volume was
// converted from a non-lvm volume to an lvm volume.
//
