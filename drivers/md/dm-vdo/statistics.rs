//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/statistics.h
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
pub struct block_allocator_statistics {
// The total number of slabs from which blocks may be allocated
    pub slab_count: u64,
// The total number of slabs from which blocks have ever been allocated
    pub slabs_opened: u64,
// The number of times since loading that a slab has been re-opened
    pub slabs_reopened: u64,
}

//
// Counters for tracking the number of items written (blocks, requests, etc.)
// that keep track of totals at steps in the write pipeline. Three counters
// allow the number of buffered, in-memory items and the number of in-flight,
// unacknowledged writes to be derived, while still tracking totals for
// reporting purposes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct commit_statistics {
// The total number of items on which processing has started
    pub started: u64,
// The total number of items for which a write operation has been issued
    pub written: u64,
// The total number of items for which a write operation has completed
    pub committed: u64,
}

// Counters for events in the recovery journal
#[repr(C)]
#[derive(Copy, Clone)]
pub struct recovery_journal_statistics {
// Number of times the on-disk journal was full
    pub disk_full: u64,
// Number of times the recovery journal requested slab journal commits.
    pub slab_journal_commits_requested: u64,
// Write/Commit totals for individual journal entries
    pub entries: commit_statistics,
// Write/Commit totals for journal blocks
    pub blocks: commit_statistics,
}

// The statistics for the compressed block packer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packer_statistics {
// Number of compressed data items written since startup
    pub compressed_fragments_written: u64,
// Number of blocks containing compressed items written since startup
    pub compressed_blocks_written: u64,
// Number of VIOs that are pending in the packer
    pub compressed_fragments_in_packer: u64,
}

// The statistics for the slab journals.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_journal_statistics {
// Number of times the on-disk journal was full
    pub disk_full_count: u64,
// Number of times an entry was added over the flush threshold
    pub flush_count: u64,
// Number of times an entry was added over the block threshold
    pub blocked_count: u64,
// Number of times a tail block was written
    pub blocks_written: u64,
// Number of times we had to wait for the tail to write
    pub tail_busy_count: u64,
}

// The statistics for the slab summary.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_summary_statistics {
// Number of blocks written
    pub blocks_written: u64,
}

// The statistics for the reference counts.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_counts_statistics {
// Number of reference blocks written
    pub blocks_written: u64,
}

// The statistics for the block map.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_statistics {
// number of dirty (resident) pages
    pub dirty_pages: u32,
// number of clean (resident) pages
    pub clean_pages: u32,
// number of free pages
    pub free_pages: u32,
// number of pages in failed state
    pub failed_pages: u32,
// number of pages incoming
    pub incoming_pages: u32,
// number of pages outgoing
    pub outgoing_pages: u32,
// how many times free page not avail
    pub cache_pressure: u32,
// number of get_vdo_page() calls for read
    pub read_count: u64,
// number of get_vdo_page() calls for write
    pub write_count: u64,
// number of times pages failed to read
    pub failed_reads: u64,
// number of times pages failed to write
    pub failed_writes: u64,
// number of gets that are reclaimed
    pub reclaimed: u64,
// number of gets for outgoing pages
    pub read_outgoing: u64,
// number of gets that were already there
    pub found_in_cache: u64,
// number of gets requiring discard
    pub discard_required: u64,
// number of gets enqueued for their page
    pub wait_for_page: u64,
// number of gets that have to fetch
    pub fetch_required: u64,
// number of page fetches
    pub pages_loaded: u64,
// number of page saves
    pub pages_saved: u64,
// the number of flushes issued
    pub flush_count: u64,
}

// The dedupe statistics from hash locks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_lock_statistics {
// Number of times the UDS advice proved correct
    pub dedupe_advice_valid: u64,
// Number of times the UDS advice proved incorrect
    pub dedupe_advice_stale: u64,
// Number of writes with the same data as another in-flight write
    pub concurrent_data_matches: u64,
// Number of writes whose hash collided with an in-flight write
    pub concurrent_hash_collisions: u64,
// Current number of dedupe queries that are in flight
    pub curr_dedupe_queries: u32,
}

// Counts of error conditions in VDO.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_statistics {
// number of times VDO got an invalid dedupe advice PBN from UDS
    pub invalid_advice_pbn_count: u64,
// number of times a VIO completed with a VDO_NO_SPACE error
    pub no_space_error_count: u64,
// number of times a VIO completed with a VDO_READ_ONLY error
    pub read_only_error_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_stats {
// Number of REQ_OP_READ bios
    pub read: u64,
// Number of REQ_OP_WRITE bios with data
    pub write: u64,
// Number of bios tagged with REQ_PREFLUSH and containing no data
    pub empty_flush: u64,
// Number of REQ_OP_DISCARD bios
    pub discard: u64,
// Number of bios tagged with REQ_PREFLUSH
    pub flush: u64,
// Number of bios tagged with REQ_FUA
    pub fua: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_usage {
// Tracked bytes currently allocated.
    pub bytes_used: u64,
// Maximum tracked bytes allocated.
    pub peak_bytes_used: u64,
}

// UDS index statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_statistics {
// Number of records stored in the index
    pub entries_indexed: u64,
// Number of post calls that found an existing entry
    pub posts_found: u64,
// Number of post calls that added a new entry
    pub posts_not_found: u64,
// Number of query calls that found an existing entry
    pub queries_found: u64,
// Number of query calls that added a new entry
    pub queries_not_found: u64,
// Number of update calls that found an existing entry
    pub updates_found: u64,
// Number of update calls that added a new entry
    pub updates_not_found: u64,
// Number of entries discarded
    pub entries_discarded: u64,
}

// The statistics of the vdo service.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_statistics {
    pub version: u32,
// Number of blocks used for data
    pub data_blocks_used: u64,
// Number of blocks used for VDO metadata
    pub overhead_blocks_used: u64,
// Number of logical blocks that are currently mapped to physical blocks
    pub logical_blocks_used: u64,
// number of physical blocks
    pub physical_blocks: block_count_t,
// number of logical blocks
    pub logical_blocks: block_count_t,
// Size of the block map page cache, in bytes
    pub block_map_cache_size: u64,
// The physical block size
    pub block_size: u64,
// Number of times the VDO has successfully recovered
    pub complete_recoveries: u64,
// Number of times the VDO has recovered from read-only mode
    pub read_only_recoveries: u64,
// String describing the operating mode of the VDO
    pub mode: [c_char; 15],
// Whether the VDO is in recovery mode
    pub in_recovery_mode: bool,
// What percentage of recovery mode work has been completed
    pub recovery_percentage: u8,
// The statistics for the compressed block packer
    pub packer: packer_statistics,
// Counters for events in the block allocator
    pub allocator: block_allocator_statistics,
// Counters for events in the recovery journal
    pub journal: recovery_journal_statistics,
// The statistics for the slab journals
    pub slab_journal: slab_journal_statistics,
// The statistics for the slab summary
    pub slab_summary: slab_summary_statistics,
// The statistics for the reference counts
    pub ref_counts: ref_counts_statistics,
// The statistics for the block map
    pub block_map: block_map_statistics,
// The dedupe statistics from hash locks
    pub hash_lock: hash_lock_statistics,
// Counts of error conditions
    pub errors: error_statistics,
// The VDO instance
    pub instance: u32,
// Current number of active VIOs
    pub current_vios_in_progress: u32,
// Maximum number of active VIOs
    pub max_vios: u32,
// Number of times the UDS index was too slow in responding
    pub dedupe_advice_timeouts: u64,
// Number of flush requests submitted to the storage device
    pub flush_out: u64,
// Logical block size
    pub logical_block_size: u64,
// Bios submitted into VDO from above
    pub bios_in: bio_stats,
    pub bios_in_partial: bio_stats,
// Bios submitted onward for user data
    pub bios_out: bio_stats,
// Bios submitted onward for metadata
    pub bios_meta: bio_stats,
    pub bios_journal: bio_stats,
    pub bios_page_cache: bio_stats,
    pub bios_out_completed: bio_stats,
    pub bios_meta_completed: bio_stats,
    pub bios_journal_completed: bio_stats,
    pub bios_page_cache_completed: bio_stats,
    pub bios_acknowledged: bio_stats,
    pub bios_acknowledged_partial: bio_stats,
// Current number of bios in progress
    pub bios_in_progress: bio_stats,
// Memory usage stats.
    pub memory_usage: memory_usage,
// The statistics for the UDS index
    pub index: index_statistics,
}
