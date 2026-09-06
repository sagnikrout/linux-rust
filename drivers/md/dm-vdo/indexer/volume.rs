//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/volume.h
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
// The volume manages deduplication records on permanent storage. The term "volume" can also refer
// to the region of permanent storage where the records (and the chapters containing them) are
// stored. The volume handles all I/O to this region by reading, caching, and writing chapter pages
// as necessary.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum index_lookup_mode {
// Always do lookups in all chapters normally
    LOOKUP_NORMAL,
// Only do a subset of lookups needed when rebuilding an index
    LOOKUP_FOR_REBUILD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queued_read {
    pub invalid: bool,
    pub reserved: bool,
    pub physical_page: u32,
    pub first_request: *mut uds_request,
    pub last_request: *mut uds_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cached_page {
// Whether this page is currently being read asynchronously
    pub read_pending: bool,
// The physical page stored in this cache entry
    pub physical_page: u32,
// The value of the volume clock when this page was last used
    pub last_used: i64,
// The cached page buffer
    pub buffer: *mut dm_buffer,
// The chapter index page, meaningless for record pages
    pub index_page: delta_index_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_cache {
// The number of zones
    pub zone_count: c_uint,
// The number of volume pages that can be cached
    pub indexable_pages: u32,
// The maximum number of simultaneously cached pages
    pub cache_slots: u16,
// An index for each physical page noting where it is in the cache
    pub index: *mut u16,
// The array of cached pages
    pub cache: *mut cached_page,
// A counter for each zone tracking if a search is occurring there
    pub search_pending_counters: *mut search_pending_counter,
// The read queue entries as a circular array
    pub read_queue: *mut queued_read,
// All entries above this point are constant after initialization.
//
// These values are all indexes into the array of read queue entries. New entries in the
// read queue are enqueued at read_queue_last. To dequeue entries, a reader thread gets the
// lock and then claims the entry pointed to by read_queue_next_read and increments that
// value. After the read is completed, the reader thread calls release_read_queue_entry(),
// which increments read_queue_first until it points to a pending read, or is equal to
// read_queue_next_read. This means that if multiple reads are outstanding,
// read_queue_first might not advance until the last of the reads finishes.
//
    pub read_queue_first: u16,
    pub read_queue_next_read: u16,
    pub read_queue_last: u16,
    pub clock: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume {
    pub geometry: index_geometry,
    pub client: *mut dm_bufio_client,
    pub nonce: u64,
    pub cache_size: usize,
// A single page worth of records, for sorting
    pub record_pointers: *const uds_volume_record,
// Sorter for sorting records within each page
    pub radix_sorter: *mut radix_sorter,
    pub sparse_cache: *mut sparse_cache,
    pub page_cache: page_cache,
    pub index_page_map: *mut index_page_map,
    pub read_threads_mutex: mutex,
    pub read_threads_cond: cond_var,
    pub read_threads_read_done_cond: cond_var,
    pub reader_threads: *mut thread,
    pub read_thread_count: c_uint,
    pub read_threads_exiting: bool,
    pub lookup_mode: index_lookup_mode,
    pub reserved_buffers: c_uint,
}

extern "C" {
    pub fn uds_free_volume(volume: *mut volume);
}
extern "C" {
    pub fn uds_forget_chapter(volume: *mut volume, chapter: u64);
}
extern "C" {
    pub fn uds_prefetch_volume_chapter(volume: *const volume, chapter: u32);
}
