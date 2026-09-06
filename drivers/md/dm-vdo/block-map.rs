//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/block-map.h
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
// The block map is responsible for tracking all the logical to physical mappings of a VDO. It
// consists of a collection of 60 radix trees gradually allocated as logical addresses are used.
// Each tree is assigned to a logical zone such that it is easy to compute which zone must handle
// each logical address. Each logical zone also has a dedicated portion of the leaf page cache.
//
// Each logical zone has a single dedicated queue and thread for performing all updates to the
// radix trees assigned to that zone. The concurrency guarantees of this single-threaded model
// allow the code to omit more fine-grained locking for the block map structures.
//
// Load operations must be performed on the admin thread. Normal operations, such as reading and
// updating mappings, must be performed on the appropriate logical zone thread. Save operations
// must be launched from the same admin thread as the original load operation.
//
// Generation counter for page references.
//
pub type vdo_page_generation = u32;
// The VDO Page Cache abstraction.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_page_cache {
// the VDO which owns this cache
    pub vdo: *mut vdo,
// number of pages in cache
    pub page_count: page_count_t,
// number of pages to write in the current batch
    pub pages_in_batch: page_count_t,
// Whether the VDO is doing a read-only rebuild
    pub rebuilding: bool,
// array of page information entries
    pub infos: *mut page_info,
// raw memory for pages
    pub pages: *mut c_char,
// cache last found page info
    pub last_found: *mut page_info,
// map of page number to info
    pub page_map: *mut int_map,
// main LRU list (all infos)
    pub lru_list: list_head,
// free page list (oldest first)
    pub free_list: list_head,
// outgoing page list
    pub outgoing_list: list_head,
// number of read I/O operations pending
    pub outstanding_reads: page_count_t,
// number of write I/O operations pending
    pub outstanding_writes: page_count_t,
// number of pages covered by the current flush
    pub pages_in_flush: page_count_t,
// number of pages waiting to be included in the next flush
    pub pages_to_flush: page_count_t,
// number of discards in progress
    pub discard_count: c_uint,
// how many VPCs waiting for free page
    pub waiter_count: c_uint,
// queue of waiters who want a free page
    pub free_waiters: vdo_wait_queue,
//
// Statistics are only updated on the logical zone thread, but are accessed from other
// threads.
//
    pub stats: block_map_statistics,
// counter for pressure reports
    pub pressure_report: u32,
// the block map zone to which this cache belongs
    pub zone: *mut block_map_zone,
}

//
// The state of a page buffer. If the page buffer is free no particular page is bound to it,
// otherwise the page buffer is bound to particular page whose absolute pbn is in the pbn field. If
// the page is resident or dirty the page data is stable and may be accessed. Otherwise the page is
// in flight (incoming or outgoing) and its data should not be accessed.
//
// @note Update the static data in get_page_state_name() if you change this enumeration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_page_buffer_state {
// this page buffer is not being used
    PS_FREE,
// this page is being read from store
    PS_INCOMING,
// attempt to load this page failed
    PS_FAILED,
// this page is valid and un-modified
    PS_RESIDENT,
// this page is valid and modified
    PS_DIRTY,
// this page is being written and should not be used
    PS_OUTGOING,
// not a state
    PAGE_STATE_COUNT,
    } __packed;

//
// The write status of page
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_page_write_status {
    WRITE_STATUS_NORMAL,
    WRITE_STATUS_DISCARD,
    WRITE_STATUS_DEFERRED,
    } __packed;

// Per-page-slot information.
    struct page_info {
// Preallocated page struct vio
    struct vio *vio;
// back-link for references
    struct vdo_page_cache *cache;
// the pbn of the page
    physical_block_number_t pbn;
// page is busy (temporarily locked)
    u16 busy;
// the write status the page
    enum vdo_page_write_status write_status;
// page state
    enum vdo_page_buffer_state state;
// queue of completions awaiting this item
    struct vdo_wait_queue waiting;
// state linked list entry
    struct list_head state_entry;
// LRU entry
    struct list_head lru_entry;
//
// The earliest recovery journal block containing uncommitted updates to the block map page
// associated with this page_info. A reference (lock) is held on that block to prevent it
// from being reaped. When this value changes, the reference on the old value must be
// released and a reference on the new value must be acquired.
//
    sequence_number_t recovery_lock;
}

//
// A completion awaiting a specific page. Also a live reference into the page once completed, until
// freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_page_completion {
// The generic completion
    pub completion: vdo_completion,
// The cache involved
    pub cache: *mut vdo_page_cache,
// The waiter for the pending list
    pub waiter: vdo_waiter,
// The absolute physical block number of the page on disk
    pub pbn: physical_block_number_t,
// Whether the page may be modified
    pub writable: bool,
// Whether the page is available
    pub ready: bool,
// The info structure for the page, only valid when ready
    pub info: *mut page_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_page {
    pub waiter: vdo_waiter,
// Dirty list entry
    pub entry: list_head,
// If dirty, the tree zone flush generation in which it was last dirtied.
    pub generation: u8,
// Whether this page is an interior tree page being written out.
    pub writing: bool,
// If writing, the tree zone flush generation of the copy being written.
    pub writing_generation: u8,
//
// Sequence number of the earliest recovery journal block containing uncommitted updates to
// this page
//
    pub recovery_lock: sequence_number_t,
// The value of recovery_lock when the this page last started writing
    pub writing_recovery_lock: sequence_number_t,
    pub page_buffer: [c_char; VDO_BLOCK_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_map_page_type {
    VDO_TREE_PAGE,
    VDO_CACHE_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirty_lists {
// The number of periods after which an element will be expired
    pub maximum_age: block_count_t,
// The oldest period which has unexpired elements
    pub oldest_period: sequence_number_t,
// One more than the current period
    pub next_period: sequence_number_t,
// The offset in the array of lists of the oldest period
    pub offset: block_count_t,
// Expired pages
    pub expired: dirty_era_t,
// The lists of dirty pages
    pub eras: [dirty_era_t; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_zone {
    pub zone_number: zone_count_t,
    pub thread_id: thread_id_t,
    pub state: admin_state,
    pub block_map: *mut block_map,
// Dirty pages, by era
    pub dirty_lists: *mut dirty_lists,
    pub page_cache: vdo_page_cache,
    pub active_lookups: data_vio_count_t,
    pub loading_pages: *mut int_map,
    pub vio_pool: *mut vio_pool,
// The tree page which has issued or will be issuing a flush
    pub flusher: *mut tree_page,
    pub flush_waiters: vdo_wait_queue,
// The generation after the most recent flush
    pub generation: u8,
    pub oldest_generation: u8,
// The counts of dirty pages in each generation
    pub dirty_page_counts: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map {
    pub vdo: *mut vdo,
    pub action_manager: *mut action_manager,
// The absolute PBN of the first root of the tree part of the block map
    pub root_origin: physical_block_number_t,
    pub root_count: block_count_t,
// The era point we are currently distributing to the zones
    pub current_era_point: sequence_number_t,
// The next era point
    pub pending_era_point: sequence_number_t,
// The number of entries in block map
    pub entry_count: block_count_t,
    pub nonce: nonce_t,
    pub journal: *mut recovery_journal,
// The trees for finding block map pages
    pub forest: *mut forest,
// The expanded trees awaiting growth
    pub next_forest: *mut forest,
// The number of entries after growth
    pub next_entry_count: block_count_t,
    pub zone_count: zone_count_t,
    pub __counted_by(zone_count): block_map_zone zones[],
}

//
// typedef vdo_entry_callback_fn - A function to be called for each allocated PBN when traversing
// the forest.
// @pbn: A PBN of a tree node.
// @completion: The parent completion of the traversal.
//
// Return: VDO_SUCCESS or an error.
//
extern "C" {
    pub fn container_of(_arg: completion, vdo_page_completion: struct, _arg: completion) -> return;
}
extern "C" {
    pub fn vdo_release_page_completion(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_request_page_write(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_invalidate_page_cache(cache: *mut vdo_page_cache) -> int __must_check;
}
extern "C" {
    pub fn vdo_find_block_map_slot(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_write_tree_page(page: *mut tree_page, zone: *mut block_map_zone);
}
extern "C" {
    pub fn vdo_resume_block_map(map: *mut block_map, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_grow_block_map(map: *mut block_map, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_abandon_block_map_growth(map: *mut block_map);
}
extern "C" {
    pub fn vdo_free_block_map(map: *mut block_map);
}
extern "C" {
    pub fn vdo_record_block_map(map: *const block_map) -> block_map_state_2_0 __must_check;
}
extern "C" {
    pub fn vdo_compute_logical_zone(data_vio: *mut data_vio) -> zone_count_t;
}
extern "C" {
    pub fn vdo_get_mapped_block(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_put_mapped_block(data_vio: *mut data_vio);
}
extern "C" {
    pub fn vdo_get_block_map_statistics(map: *mut block_map) -> block_map_statistics __must_check;
}
//
// vdo_convert_maximum_age() - Convert the maximum age to reflect the new recovery journal format
// @age: The configured maximum age
//
// Return: The converted age
//
// In the old recovery journal format, each journal block held 311 entries, and every write bio
// made two entries. The old maximum age was half the usable journal length. In the new format,
// each block holds only 217 entries, but each bio only makes one entry. We convert the configured
// age so that the number of writes in a block map era is the same in the old and new formats. This
// keeps the bound on the amount of work required to recover the block map from the recovery
// journal the same across the format change. It also keeps the amortization of block map page
// writes to write bios the same.
//
