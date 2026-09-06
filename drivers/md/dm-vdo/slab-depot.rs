//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/slab-depot.h
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
// A slab_depot is responsible for managing all of the slabs and block allocators of a VDO. It has
// a single array of slabs in order to eliminate the need for additional math in order to compute
// which physical zone a PBN is in. It also has a block_allocator per zone.
//
// Each physical zone has a single dedicated queue and thread for performing all updates to the
// slabs assigned to that zone. The concurrency guarantees of this single-threaded model allow the
// code to omit more fine-grained locking for the various slab structures. Each physical zone
// maintains a separate copy of the slab summary to remove the need for explicit locking on that
// structure as well.
//
// Load operations must be performed on the admin thread. Normal operations, such as allocations
// and reference count updates, must be performed on the appropriate physical zone thread. Requests
// from the recovery journal to commit slab journal tail blocks must be scheduled from the recovery
// journal thread to run on the appropriate physical zone thread. Save operations must be launched
// from the same admin thread as the original load operation.
//
// The number of vios in the vio pool is proportional to the throughput of the VDO.
//
// The number of vios in the vio pool used for loading reference count data. A slab's
// refcounts is capped at ~8MB, and we process one at a time in a zone, so 9 should be
// plenty.
//
// Represents the possible status of a block.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reference_status {
    RS_FREE, /* this block is free */
    RS_SINGLE, /* this block is singly-referenced */
    RS_SHARED, /* this block is shared */
    RS_PROVISIONAL /* this block is provisionally allocated */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct journal_lock {
    pub count: u16,
    pub recovery_start: sequence_number_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_journal {
// A waiter object for getting a VIO pool entry
    pub resource_waiter: vdo_waiter,
// A waiter object for updating the slab summary
    pub slab_summary_waiter: vdo_waiter,
// A waiter object for getting a vio with which to flush
    pub flush_waiter: vdo_waiter,
// The queue of VIOs waiting to make an entry
    pub entry_waiters: vdo_wait_queue,
// The parent slab reference of this journal
    pub slab: *mut vdo_slab,
// Whether a tail block commit is pending
    pub waiting_to_commit: bool,
// Whether the journal is updating the slab summary
    pub updating_slab_summary: bool,
// Whether the journal is adding entries from the entry_waiters queue
    pub adding_entries: bool,
// Whether a partial write is in progress
    pub partial_write_in_progress: bool,
// The oldest block in the journal on disk
    pub head: sequence_number_t,
// The oldest block in the journal which may not be reaped
    pub unreapable: sequence_number_t,
// The end of the half-open interval of the active journal
    pub tail: sequence_number_t,
// The next journal block to be committed
    pub next_commit: sequence_number_t,
// The tail sequence number that is written in the slab summary
    pub summarized: sequence_number_t,
// The tail sequence number that was last summarized in slab summary
    pub last_summarized: sequence_number_t,
// The sequence number of the recovery journal lock
    pub recovery_lock: sequence_number_t,
//
// The number of entries which fit in a single block. Can't use the constant because unit
// tests change this number.
//
    pub entries_per_block: journal_entry_count_t,
//
// The number of full entries which fit in a single block. Can't use the constant because
// unit tests change this number.
//
    pub full_entries_per_block: journal_entry_count_t,
// The recovery journal of the VDO (slab journal holds locks on it)
    pub recovery_journal: *mut recovery_journal,
// The statistics shared by all slab journals in our physical zone
    pub events: *mut slab_journal_statistics,
// A list of the VIO pool entries for outstanding journal block writes
    pub uncommitted_blocks: list_head,
//
// The current tail block header state. This will be packed into the block just before it
// is written.
//
    pub tail_header: slab_journal_block_header,
// A pointer to a block-sized buffer holding the packed block data
    pub block: *mut packed_slab_journal_block,
// The number of blocks in the on-disk journal
    pub size: block_count_t,
// The number of blocks at which to start pushing reference blocks
    pub flushing_threshold: block_count_t,
// The number of blocks at which all reference blocks should be writing
    pub flushing_deadline: block_count_t,
// The number of blocks at which to wait for reference blocks to write
    pub blocking_threshold: block_count_t,
// The number of blocks at which to scrub the slab before coming online
    pub scrubbing_threshold: block_count_t,
// This list entry is for block_allocator to keep a queue of dirty journals
    pub dirty_entry: list_head,
// The lock for the oldest unreaped block of the journal
    pub reap_lock: *mut journal_lock,
// The locks for each on disk block
    pub locks: *mut journal_lock,
}

//
// Reference_block structure
//
// Blocks are used as a proxy, permitting saves of partial refcounts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reference_block {
// This block waits on the ref_counts to tell it to write
    pub waiter: vdo_waiter,
// The slab to which this reference_block belongs
    pub slab: *mut vdo_slab,
// The number of references in this block that represent allocations
    pub allocated_count: block_size_t,
// The slab journal block on which this block must hold a lock
    pub slab_journal_lock: sequence_number_t,
// The slab journal block which should be released when this block is committed
    pub slab_journal_lock_to_release: sequence_number_t,
// The point up to which each sector is accurate on disk
    pub commit_points: [journal_point; VDO_SECTORS_PER_BLOCK],
// Whether this block has been modified since it was written to disk
    pub is_dirty: bool,
// Whether this block is currently writing
    pub is_writing: bool,
}

// The search_cursor represents the saved position of a free block search.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct search_cursor {
// The reference block containing the current search index
    pub block: *mut reference_block,
// The position at which to start searching for the next free counter
    pub index: slab_block_number,
// The position just past the last valid counter in the current block
    pub end_index: slab_block_number,
// A pointer to the first reference block in the slab
    pub first_block: *mut reference_block,
// A pointer to the last reference block in the slab
    pub last_block: *mut reference_block,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slab_rebuild_status {
    VDO_SLAB_REBUILT,
    VDO_SLAB_REPLAYING,
    VDO_SLAB_REQUIRES_SCRUBBING,
    VDO_SLAB_REQUIRES_HIGH_PRIORITY_SCRUBBING,
    VDO_SLAB_REBUILDING,
}

//
// This is the type declaration for the vdo_slab type. A vdo_slab currently consists of a run of
// 2^23 data blocks, but that will soon change to dedicate a small number of those blocks for
// metadata storage for the reference counts and slab journal for the slab.
//
// A reference count is maintained for each physical block number. The vast majority of blocks have
// a very small reference count (usually 0 or 1). For references less than or equal to MAXIMUM_REFS
// (254) the reference count is stored in counters[pbn].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_slab {
// A list entry to queue this slab in a block_allocator list
    pub allocq_entry: list_head,
// The struct block_allocator that owns this slab
    pub allocator: *mut block_allocator,
// The journal for this slab
    pub journal: slab_journal,
// The slab number of this slab
    pub slab_number: slab_count_t,
// The offset in the allocator partition of the first block in this slab
    pub start: physical_block_number_t,
// The offset of the first block past the end of this slab
    pub end: physical_block_number_t,
// The starting translated PBN of the slab journal
    pub journal_origin: physical_block_number_t,
// The starting translated PBN of the reference counts
    pub ref_counts_origin: physical_block_number_t,
// The administrative state of the slab
    pub state: admin_state,
// The status of the slab
    pub status: slab_rebuild_status,
// Whether the slab was ever queued for scrubbing
    pub was_queued_for_scrubbing: bool,
// The priority at which this slab has been queued for allocation
    pub priority: u8,
// Fields beyond this point are the reference counts for the data blocks in this slab.
// The size of the counters array
    pub block_count: u32,
// The number of free blocks
    pub free_blocks: u32,
// The array of reference counts
    pub /: *mut *mut *mut vdo_refcount_t counters; / use vdo_allocate() to align data ptr,
// The saved block pointer and array indexes for the free block search
    pub search_cursor: search_cursor,
// A list of the dirty blocks waiting to be written out
    pub dirty_blocks: vdo_wait_queue,
// The number of blocks which are currently reading or writing
    pub active_count: usize,
// A waiter object for updating the slab summary
    pub summary_waiter: vdo_waiter,
// The latest slab journal for which there has been a reference count update
    pub slab_journal_point: journal_point,
// The number of reference count blocks
    pub reference_block_count: u32,
// reference count block array
    pub reference_blocks: *mut reference_block,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_allocator_drain_step {
    VDO_DRAIN_ALLOCATOR_START,
    VDO_DRAIN_ALLOCATOR_STEP_SCRUBBER,
    VDO_DRAIN_ALLOCATOR_STEP_SLABS,
    VDO_DRAIN_ALLOCATOR_STEP_SUMMARY,
    VDO_DRAIN_ALLOCATOR_STEP_FINISHED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_scrubber {
// The queue of slabs to scrub first
    pub high_priority_slabs: list_head,
// The queue of slabs to scrub once there are no high_priority_slabs
    pub slabs: list_head,
// The queue of VIOs waiting for a slab to be scrubbed
    pub waiters: vdo_wait_queue,
//
// The number of slabs that are unrecovered or being scrubbed. This field is modified by
// the physical zone thread, but is queried by other threads.
//
    pub slab_count: slab_count_t,
// The administrative state of the scrubber
    pub admin_state: admin_state,
// Whether to only scrub high-priority slabs
    pub high_priority_only: bool,
// The slab currently being scrubbed
    pub slab: *mut vdo_slab,
// The vio for loading slab journal blocks
    pub vio: vio,
}

// A sub-structure for applying actions in parallel to all an allocator's slabs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_actor {
// The number of slabs performing a slab action
    pub slab_action_count: slab_count_t,
// The method to call when a slab action has been completed by all slabs
    pub callback: vdo_action_fn,
}

// A slab_iterator is a structure for iterating over a set of slabs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_iterator {
    pub slabs: *mut vdo_slab,
    pub next: *mut vdo_slab,
    pub end: slab_count_t,
    pub stride: slab_count_t,
}

//
// The slab_summary provides hints during load and recovery about the state of the slabs in order
// to avoid the need to read the slab journals in their entirety before a VDO can come online.
//
// The information in the summary for each slab includes the rough number of free blocks (which is
// used to prioritize scrubbing), the cleanliness of a slab (so that clean slabs containing free
// space will be used on restart), and the location of the tail block of the slab's journal.
//
// The slab_summary has its own partition at the end of the volume which is sized to allow for a
// complete copy of the summary for each of up to 16 physical zones.
//
// During resize, the slab_summary moves its backing partition and is saved once moved; the
// slab_summary is not permitted to overwrite the previous recovery journal space.
//
// The slab_summary does not have its own version information, but relies on the VDO volume version
// number.
//
// A slab status is a very small structure for use in determining the ordering of slabs in the
// scrubbing process.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_status {
    pub slab_number: slab_count_t,
    pub is_clean: bool,
    pub emptiness: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_summary_block {
// The block_allocator to which this block belongs
    pub allocator: *mut block_allocator,
// The index of this block in its zone's summary
    pub index: block_count_t,
// Whether this block has a write outstanding
    pub writing: bool,
// Ring of updates waiting on the outstanding write
    pub current_update_waiters: vdo_wait_queue,
// Ring of updates waiting on the next write
    pub next_update_waiters: vdo_wait_queue,
// The active slab_summary_entry array for this block
    pub entries: *mut slab_summary_entry,
// The vio used to write this block
    pub vio: vio,
// The packed entries, one block long, backing the vio
    pub outgoing_entries: *mut c_char,
}

//
// The statistics for all the slab summary zones owned by this slab summary. These fields are all
// mutated only by their physical zone threads, but are read by other threads when gathering
// statistics for the entire depot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atomic_slab_summary_statistics {
// Number of blocks written
    pub blocks_written: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_allocator {
    pub completion: vdo_completion,
// The slab depot for this allocator
    pub depot: *mut slab_depot,
// The nonce of the VDO
    pub nonce: nonce_t,
// The physical zone number of this allocator
    pub zone_number: zone_count_t,
// The thread ID for this allocator's physical zone
    pub thread_id: thread_id_t,
// The number of slabs in this allocator
    pub slab_count: slab_count_t,
// The number of the last slab owned by this allocator
    pub last_slab: slab_count_t,
// The reduced priority level used to preserve unopened slabs
    pub unopened_slab_priority: c_uint,
// The state of this allocator
    pub state: admin_state,
// The actor for applying an action to all slabs
    pub slab_actor: slab_actor,
// The slab from which blocks are currently being allocated
    pub open_slab: *mut vdo_slab,
// A priority queue containing all slabs available for allocation
    pub prioritized_slabs: *mut priority_table,
// The slab scrubber
    pub scrubber: slab_scrubber,
// What phase of the close operation the allocator is to perform
    pub drain_step: block_allocator_drain_step,
//
// These statistics are all mutated only by the physical zone thread, but are read by other
// threads when gathering statistics for the entire depot.
//
// The count of allocated blocks in this zone. Not in block_allocator_statistics for
// historical reasons.
//
    pub allocated_blocks: u64,
// Statistics for this block allocator
    pub statistics: block_allocator_statistics,
// Cumulative statistics for the slab journals in this zone
    pub slab_journal_statistics: slab_journal_statistics,
// Cumulative statistics for the reference counters in this zone
    pub ref_counts_statistics: ref_counts_statistics,
//
// This is the head of a queue of slab journals which have entries in their tail blocks
// which have not yet started to commit. When the recovery journal is under space pressure,
// slab journals which have uncommitted entries holding a lock on the recovery journal head
// are forced to commit their blocks early. This list is kept in order, with the tail
// containing the slab journal holding the most recent recovery journal lock.
//
    pub dirty_slab_journals: list_head,
// The vio pool for reading and writing block allocator metadata
    pub vio_pool: *mut vio_pool,
// The vio pool for large initial reads of ref count areas
    pub refcount_big_vio_pool: *mut vio_pool,
// How many ref count blocks are read per vio at initial load
    pub refcount_blocks_per_big_vio: u32,
// The dm_kcopyd client for erasing slab journals
    pub eraser: *mut dm_kcopyd_client,
// Iterator over the slabs to be erased
    pub slabs_to_erase: slab_iterator,
// The portion of the slab summary managed by this allocator
// The state of the slab summary
    pub summary_state: admin_state,
// The number of outstanding summary writes
    pub summary_write_count: block_count_t,
// The array (owned by the blocks) of all entries
    pub summary_entries: *mut slab_summary_entry,
// The array of slab_summary_blocks
    pub summary_blocks: *mut slab_summary_block,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slab_depot_load_type {
    VDO_SLAB_DEPOT_NORMAL_LOAD,
    VDO_SLAB_DEPOT_RECOVERY_LOAD,
    VDO_SLAB_DEPOT_REBUILD_LOAD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_depot {
    pub zone_count: zone_count_t,
    pub old_zone_count: zone_count_t,
    pub vdo: *mut vdo,
    pub slab_config: slab_config,
    pub action_manager: *mut action_manager,
    pub first_block: physical_block_number_t,
    pub last_block: physical_block_number_t,
    pub origin: physical_block_number_t,
// slab_size == (1 << slab_size_shift)
    pub slab_size_shift: c_uint,
// Determines how slabs should be queued during load
    pub load_type: slab_depot_load_type,
// The state for notifying slab journals to release recovery journal
    pub active_release_request: sequence_number_t,
    pub new_release_request: sequence_number_t,
// State variables for scrubbing complete handling
    pub zones_to_scrub: core::sync::atomic::AtomicI32,
// Array of pointers to individually allocated slabs
    pub slabs: *mut vdo_slab,
// The number of slabs currently allocated and stored in 'slabs'
    pub slab_count: slab_count_t,
// Array of pointers to a larger set of slabs (used during resize)
    pub new_slabs: *mut vdo_slab,
// The number of slabs currently allocated and stored in 'new_slabs'
    pub new_slab_count: slab_count_t,
// The size that 'new_slabs' was allocated for
    pub new_size: block_count_t,
// The last block before resize, for rollback
    pub old_last_block: physical_block_number_t,
// The last block after resize, for resize
    pub new_last_block: physical_block_number_t,
// The statistics for the slab summary
    pub summary_statistics: atomic_slab_summary_statistics,
// The start of the slab summary partition
    pub summary_origin: physical_block_number_t,
// The number of bits to shift to get a 7-bit fullness hint
    pub hint_shift: c_uint,
// The slab summary entries for all of the zones the partition can hold
    pub summary_entries: *mut slab_summary_entry,
// The block allocators for this depot
    pub __counted_by(zone_count): block_allocator allocators[],
}

extern "C" {
    pub fn container_of(_arg: completion, block_allocator: struct, _arg: completion) -> return;
}
extern "C" {
    pub fn vdo_notify_slab_journals_are_recovered(completion: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_dump_block_allocator(allocator: *const block_allocator);
}
extern "C" {
    pub fn vdo_free_slab_depot(depot: *mut slab_depot);
}
extern "C" {
    pub fn vdo_record_slab_depot(depot: *const slab_depot) -> slab_depot_state_2_0 __must_check;
}
extern "C" {
    pub fn vdo_allocate_reference_counters(depot: *mut slab_depot) -> int __must_check;
}
extern "C" {
    pub fn vdo_get_slab_depot_allocated_blocks(depot: *const slab_depot) -> block_count_t __must_check;
}
extern "C" {
    pub fn vdo_get_slab_depot_data_blocks(depot: *const slab_depot) -> block_count_t __must_check;
}
extern "C" {
    pub fn vdo_update_slab_depot_size(depot: *mut slab_depot);
}
extern "C" {
    pub fn vdo_use_new_slabs(depot: *mut slab_depot, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_abandon_new_slabs(depot: *mut slab_depot);
}
extern "C" {
    pub fn vdo_resume_slab_depot(depot: *mut slab_depot, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_dump_slab_depot(depot: *const slab_depot);
}
