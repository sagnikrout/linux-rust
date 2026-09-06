//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/recovery-journal.h
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
// DOC: recovery journal.
//
// The recovery_journal provides a log of all block mapping and reference count changes which have
// not yet been stably written to the block map or slab journals. This log helps to reduce the
// write amplification of writes by providing amortization of slab journal and block map page
// updates.
//
// The recovery journal has a single dedicated queue and thread for performing all journal updates.
// The concurrency guarantees of this single-threaded model allow the code to omit more
// fine-grained locking for recovery journal structures.
//
// The journal consists of a set of on-disk blocks arranged as a circular log with monotonically
// increasing sequence numbers. Three sequence numbers serve to define the active extent of the
// journal. The 'head' is the oldest active block in the journal. The 'tail' is the end of the
// half-open interval containing the active blocks. 'active' is the number of the block actively
// receiving entries. In an empty journal, head == active == tail. Once any entries are added, tail
// = active + 1, and head may be any value in the interval [tail - size, active].
//
// The journal also contains a set of in-memory blocks which are used to buffer up entries until
// they can be committed. In general the number of in-memory blocks ('tail_buffer_count') will be
// less than the on-disk size. Each in-memory block is also a vdo_completion. Each in-memory block
// has a vio which is used to commit that block to disk. The vio's data is the on-disk
// representation of the journal block. In addition each in-memory block has a buffer which is used
// to accumulate entries while a partial commit of the block is in progress. In-memory blocks are
// kept on two lists. Free blocks live on the 'free_tail_blocks' list. When a block becomes active
// (see below) it is moved to the 'active_tail_blocks' list. When a block is fully committed, it is
// moved back to the 'free_tail_blocks' list.
//
// When entries are added to the journal, they are added to the active in-memory block, as
// indicated by the 'active_block' field. If the caller wishes to wait for the entry to be
// committed, the requesting VIO will be attached to the in-memory block to which the caller's
// entry was added. If the caller does wish to wait, or if the entry filled the active block, an
// attempt will be made to commit that block to disk. If there is already another commit in
// progress, the attempt will be ignored and then automatically retried when the in-progress commit
// completes. If there is no commit in progress, any data_vios waiting on the block are transferred
// to the block's vio which is then written, automatically waking all of the waiters when it
// completes. When the write completes, any entries which accumulated in the block are copied to
// the vio's data buffer.
//
// Finally, the journal maintains a set of counters, one for each on disk journal block. These
// counters are used as locks to prevent premature reaping of journal blocks. Each time a new
// sequence number is used, the counter for the corresponding block is incremented. The counter is
// subsequently decremented when that block is filled and then committed for the last time. This
// prevents blocks from being reaped while they are still being updated. The counter is also
// incremented once for each entry added to a block, and decremented once each time the block map
// is updated in memory for that request. This prevents blocks from being reaped while their VIOs
// are still active. Finally, each in-memory block map page tracks the oldest journal block that
// contains entries corresponding to uncommitted updates to that block map page. Each time an
// in-memory block map page is updated, it checks if the journal block for the VIO is earlier than
// the one it references, in which case it increments the count on the earlier journal block and
// decrements the count on the later journal block, maintaining a lock on the oldest journal block
// containing entries for that page. When a block map page has been flushed from the cache, the
// counter for the journal block it references is decremented. Whenever the counter for the head
// block goes to 0, the head is advanced until it comes to a block whose counter is not 0 or until
// it reaches the active block. This is the mechanism for reclaiming journal space on disk.
//
// If there is no in-memory space when a VIO attempts to add an entry, the VIO will be attached to
// the 'commit_completion' and will be woken the next time a full block has committed. If there is
// no on-disk space when a VIO attempts to add an entry, the VIO will be attached to the
// 'reap_completion', and will be woken the next time a journal block is reaped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_zone_type {
    VDO_ZONE_TYPE_ADMIN,
    VDO_ZONE_TYPE_JOURNAL,
    VDO_ZONE_TYPE_LOGICAL,
    VDO_ZONE_TYPE_PHYSICAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_counter {
// The completion for notifying the owner of a lock release
    pub completion: vdo_completion,
// The number of logical zones which may hold locks
    pub logical_zones: zone_count_t,
// The number of physical zones which may hold locks
    pub physical_zones: zone_count_t,
// The number of locks
    pub locks: block_count_t,
// Whether the lock release notification is in flight
    pub state: core::sync::atomic::AtomicI32,
// The number of logical zones which hold each lock
    pub logical_zone_counts: *mut core::sync::atomic::AtomicI32,
// The number of physical zones which hold each lock
    pub physical_zone_counts: *mut core::sync::atomic::AtomicI32,
// The per-lock counts for the journal zone
    pub journal_counters: *mut u16,
// The per-lock decrement counts for the journal zone
    pub journal_decrement_counts: *mut core::sync::atomic::AtomicI32,
// The per-zone, per-lock reference counts for logical zones
    pub logical_counters: *mut u16,
// The per-zone, per-lock reference counts for physical zones
    pub physical_counters: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct recovery_journal_block {
// The doubly linked pointers for the free or active lists
    pub list_node: list_head,
// The waiter for the pending full block list
    pub write_waiter: vdo_waiter,
// The journal to which this block belongs
    pub journal: *mut recovery_journal,
// A pointer to the current sector in the packed block buffer
    pub sector: *mut packed_journal_sector,
// The vio for writing this block
    pub vio: vio,
// The sequence number for this block
    pub sequence_number: sequence_number_t,
// The location of this block in the on-disk journal
    pub block_number: physical_block_number_t,
// Whether this block is being committed
    pub committing: bool,
// The total number of entries in this block
    pub entry_count: journal_entry_count_t,
// The total number of uncommitted entries (queued or committing)
    pub uncommitted_entry_count: journal_entry_count_t,
// The number of new entries in the current commit
    pub entries_in_commit: journal_entry_count_t,
// The queue of vios which will make entries for the next commit
    pub entry_waiters: vdo_wait_queue,
// The queue of vios waiting for the current commit
    pub commit_waiters: vdo_wait_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct recovery_journal {
// The thread ID of the journal zone
    pub thread_id: thread_id_t,
// The slab depot which can hold locks on this journal
    pub depot: *mut slab_depot,
// The block map which can hold locks on this journal
    pub block_map: *mut block_map,
// The queue of vios waiting to make entries
    pub entry_waiters: vdo_wait_queue,
// The number of free entries in the journal
    pub available_space: u64,
// The number of decrement entries which need to be made
    pub pending_decrement_count: data_vio_count_t,
// Whether the journal is adding entries from the increment or decrement waiters queues
    pub adding_entries: bool,
// The administrative state of the journal
    pub state: admin_state,
// Whether a reap is in progress
    pub reaping: bool,
// The location of the first journal block
    pub origin: physical_block_number_t,
// The oldest active block in the journal on disk for block map rebuild
    pub block_map_head: sequence_number_t,
// The oldest active block in the journal on disk for slab journal replay
    pub slab_journal_head: sequence_number_t,
// The newest block in the journal on disk to which a write has finished
    pub last_write_acknowledged: sequence_number_t,
// The end of the half-open interval of the active journal
    pub tail: sequence_number_t,
// The point at which the last entry will have been added
    pub append_point: journal_point,
// The journal point of the vio most recently released from the journal
    pub commit_point: journal_point,
// The nonce of the VDO
    pub nonce: nonce_t,
// The number of recoveries completed by the VDO
    pub recovery_count: u8,
// The number of entries which fit in a single block
    pub entries_per_block: journal_entry_count_t,
// Unused in-memory journal blocks
    pub free_tail_blocks: list_head,
// In-memory journal blocks with records
    pub active_tail_blocks: list_head,
// A pointer to the active block (the one we are adding entries to now)
    pub active_block: *mut recovery_journal_block,
// Journal blocks that need writing
    pub pending_writes: vdo_wait_queue,
// The new block map reap head after reaping
    pub block_map_reap_head: sequence_number_t,
// The head block number for the block map rebuild range
    pub block_map_head_block_number: block_count_t,
// The new slab journal reap head after reaping
    pub slab_journal_reap_head: sequence_number_t,
// The head block number for the slab journal replay range
    pub slab_journal_head_block_number: block_count_t,
// The data-less vio, usable only for flushing
    pub flush_vio: *mut vio,
// The number of blocks in the on-disk journal
    pub size: block_count_t,
// The number of logical blocks that are in-use
    pub logical_blocks_used: block_count_t,
// The number of block map pages that are allocated
    pub block_map_data_blocks: block_count_t,
// The number of journal blocks written but not yet acknowledged
    pub pending_write_count: block_count_t,
// The threshold at which slab journal tail blocks will be written out
    pub slab_journal_commit_threshold: block_count_t,
// Counters for events in the journal that are reported as statistics
    pub events: recovery_journal_statistics,
// The locks for each on-disk block
    pub lock_counter: lock_counter,
// The tail blocks
    pub blocks: [recovery_journal_block; ],
}

//
// vdo_get_recovery_journal_block_number() - Get the physical block number for a given sequence
// number.
// @journal: The journal.
// @sequence: The sequence number of the desired block.
//
// Return: The block number corresponding to the sequence number.
//
// Since journal size is a power of two, the block number modulus can just be extracted
// from the low-order bits of the sequence.
//
extern "C" {
    pub fn vdo_compute_recovery_journal_block_number(_arg: journal->size, _arg: sequence) -> return;
}
//
// vdo_compute_recovery_journal_check_byte() - Compute the check byte for a given sequence number.
// @journal: The journal.
// @sequence: The sequence number.
//
// Return: The check byte corresponding to the sequence number.
//
// The check byte must change with each trip around the journal.
extern "C" {
    pub fn vdo_free_recovery_journal(journal: *mut recovery_journal);
}
extern "C" {
    pub fn vdo_get_recovery_journal_thread_id(journal: *mut recovery_journal) -> thread_id_t __must_check;
}
extern "C" {
    pub fn vdo_get_recovery_journal_length(journal_size: block_count_t) -> block_count_t __must_check;
}
extern "C" {
    pub fn vdo_dump_recovery_journal_statistics(journal: *const recovery_journal);
}
