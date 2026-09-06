//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/data-vio.h
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

// Codes for describing the last asynchronous operation performed on a vio.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum async_operation_number {
    MIN_VIO_ASYNC_OPERATION_NUMBER,
    VIO_ASYNC_OP_LAUNCH = MIN_VIO_ASYNC_OPERATION_NUMBER,
    VIO_ASYNC_OP_ACKNOWLEDGE_WRITE,
    VIO_ASYNC_OP_ACQUIRE_VDO_HASH_LOCK,
    VIO_ASYNC_OP_ATTEMPT_LOGICAL_BLOCK_LOCK,
    VIO_ASYNC_OP_LOCK_DUPLICATE_PBN,
    VIO_ASYNC_OP_CHECK_FOR_DUPLICATION,
    VIO_ASYNC_OP_CLEANUP,
    VIO_ASYNC_OP_COMPRESS_DATA_VIO,
    VIO_ASYNC_OP_FIND_BLOCK_MAP_SLOT,
    VIO_ASYNC_OP_GET_MAPPED_BLOCK_FOR_READ,
    VIO_ASYNC_OP_GET_MAPPED_BLOCK_FOR_WRITE,
    VIO_ASYNC_OP_HASH_DATA_VIO,
    VIO_ASYNC_OP_JOURNAL_REMAPPING,
    VIO_ASYNC_OP_ATTEMPT_PACKING,
    VIO_ASYNC_OP_PUT_MAPPED_BLOCK,
    VIO_ASYNC_OP_READ_DATA_VIO,
    VIO_ASYNC_OP_UPDATE_DEDUPE_INDEX,
    VIO_ASYNC_OP_UPDATE_REFERENCE_COUNTS,
    VIO_ASYNC_OP_VERIFY_DUPLICATION,
    VIO_ASYNC_OP_WRITE_DATA_VIO,
    MAX_VIO_ASYNC_OPERATION_NUMBER,
    } __packed;

    struct lbn_lock {
    logical_block_number_t lbn;
    bool locked;
    struct vdo_wait_queue waiters;
    struct logical_zone *zone;
}

// A position in the arboreal block map at a specific level.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_map_tree_slot {
    pub page_index: page_number_t,
    pub block_map_slot: block_map_slot,
}

// Fields for using the arboreal block map.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_lock {
// The current height at which this data_vio is operating
    pub height: height_t,
// The block map tree for this LBN
    pub root_index: root_count_t,
// Whether we hold a page lock
    pub locked: bool,
// The key for the lock map
    pub key: u64,
// The queue of waiters for the page this vio is allocating or loading
    pub waiters: vdo_wait_queue,
// The block map tree slots for this LBN
    pub 1]: block_map_tree_slot tree_slots[VDO_BLOCK_MAP_TREE_HEIGHT +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoned_pbn {
    pub pbn: physical_block_number_t,
    pub state: block_mapping_state,
    pub zone: *mut physical_zone,
}

//
// Where a data_vio is on the compression path; advance_data_vio_compression_stage()
// depends on the order of this enum.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_vio_compression_stage {
// A data_vio which has not yet entered the compression path
    DATA_VIO_PRE_COMPRESSOR,
// A data_vio which is in the compressor
    DATA_VIO_COMPRESSING,
// A data_vio which is blocked in the packer
    DATA_VIO_PACKING,
// A data_vio which is no longer on the compression path (and never will be)
    DATA_VIO_POST_PACKER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_vio_compression_status {
    pub stage: data_vio_compression_stage,
    pub may_not_compress: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compression_state {
//
// The current compression status of this data_vio. This field contains a value which
// consists of a data_vio_compression_stage and a flag indicating whether a request has
// been made to cancel (or prevent) compression for this data_vio.
//
// This field should be accessed through the get_data_vio_compression_status() and
// set_data_vio_compression_status() methods. It should not be accessed directly.
//
    pub status: core::sync::atomic::AtomicI32,
// The compressed size of this block
    pub size: u16,
// The packer input or output bin slot which holds the enclosing data_vio
    pub slot: slot_number_t,
// The packer bin to which the enclosing data_vio has been assigned
    pub bin: *mut packer_bin,
// A link in the chain of data_vios which have been packed together
    pub next_in_batch: *mut data_vio,
// A vio which is blocked in the packer while holding a lock this vio needs.
    pub lock_holder: *mut data_vio,
//
// The compressed block used to hold the compressed form of this block and that of any
// other blocks for which this data_vio is the compressed write agent.
//
    pub block: *mut compressed_block,
}

// Fields supporting allocation of data blocks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocation {
// The physical zone in which to allocate a physical block
    pub zone: *mut physical_zone,
// The block allocated to this vio
    pub pbn: physical_block_number_t,
//
// If non-NULL, the pooled PBN lock held on the allocated block. Must be a write lock until
// the block has been written, after which it will become a read lock.
//
    pub lock: *mut pbn_lock,
// The type of write lock to obtain on the allocated block
    pub write_lock_type: pbn_lock_type,
// The zone which was the start of the current allocation cycle
    pub first_allocation_zone: zone_count_t,
// Whether this vio should wait for a clean slab
    pub wait_for_clean_slab: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reference_updater {
    pub operation: journal_operation,
    pub increment: bool,
    pub zpbn: zoned_pbn,
    pub lock: *mut pbn_lock,
    pub waiter: vdo_waiter,
}

// A vio for processing user data requests.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_vio {
// The vdo_wait_queue entry structure
    pub waiter: vdo_waiter,
// The logical block of this request
    pub logical: lbn_lock,
// The state for traversing the block map tree
    pub tree_lock: tree_lock,
// The current partition address of this block
    pub mapped: zoned_pbn,
// The hash of this vio (if not zero)
    pub record_name: uds_record_name,
// Used for logging and debugging
    pub last_async_operation: async_operation_number,
// The operations to record in the recovery and slab journals
    pub increment_updater: reference_updater,
    pub decrement_updater: reference_updater,
    pub 1: u16 read :,
    pub 1: u16 write :,
    pub 1: u16 fua :,
    pub 1: u16 is_zero :,
    pub 1: u16 is_discard :,
    pub 1: u16 is_partial :,
    pub 1: u16 is_duplicate :,
    pub 1: u16 first_reference_operation_complete :,
    pub 1: u16 downgrade_allocation_lock :,
    pub allocation: allocation,
//
// Whether this vio has received an allocation. This field is examined from threads not in
// the allocation zone.
//
    pub allocation_succeeded: bool,
// The new partition address of this block after the vio write completes
    pub new_mapped: zoned_pbn,
// The hash zone responsible for the name (NULL if is_zero_block)
    pub hash_zone: *mut hash_zone,
// The lock this vio holds or shares with other vios with the same data
    pub hash_lock: *mut hash_lock,
// All data_vios sharing a hash lock are kept in a list linking these list entries
    pub hash_lock_entry: list_head,
// The block number in the partition of the UDS deduplication advice
    pub duplicate: zoned_pbn,
//
// The sequence number of the recovery journal block containing the increment entry for
// this vio.
//
    pub recovery_sequence_number: sequence_number_t,
// The point in the recovery journal where this write last made an entry
    pub recovery_journal_point: journal_point,
// The list of vios in user initiated write requests
    pub write_entry: list_head,
// The generation number of the VDO that this vio belongs to
    pub flush_generation: sequence_number_t,
// The completion to use for fetching block map pages for this vio
    pub page_completion: vdo_page_completion,
// The user bio that initiated this VIO
    pub user_bio: *mut bio,
// partial block support
    pub offset: block_size_t,
//
// The number of bytes to be discarded. For discards, this field will always be positive,
// whereas for non-discards it will always be 0. Hence it can be used to determine whether
// a data_vio is processing a discard, even after the user_bio has been acknowledged.
//
    pub remaining_discard: u32,
    pub dedupe_context: *mut dedupe_context,
// Fields beyond this point will not be reset when a pooled data_vio is reused.
    pub vio: vio,
// The completion for making reference count decrements
    pub decrement_completion: vdo_completion,
// All of the fields necessary for the compression path
    pub compression: compression_state,
// A block used as output during compression or uncompression
    pub scratch_block: *mut c_char,
    pub pool_entry: list_head,
}

extern "C" {
    pub fn container_of(_arg: vio, data_vio: struct, _arg: vio) -> return;
}
extern "C" {
    pub fn vio_as_data_vio(_arg: as_vio(completion)) -> return;
}
extern "C" {
    pub fn container_of(_arg: waiter, data_vio: struct, _arg: waiter) -> return;
}
extern "C" {
    pub fn container_of(_arg: updater, data_vio: struct, _arg: increment_updater) -> return;
}
extern "C" {
    pub fn container_of(_arg: updater, data_vio: struct, _arg: decrement_updater) -> return;
}
extern "C" {
    pub fn cancel_data_vio_compression(data_vio: *mut data_vio) -> bool;
}
extern "C" {
    pub fn free_data_vio_pool(pool: *mut data_vio_pool);
}
extern "C" {
    pub fn vdo_launch_bio(pool: *mut data_vio_pool, bio: *mut bio);
}
extern "C" {
    pub fn drain_data_vio_pool(pool: *mut data_vio_pool, completion: *mut vdo_completion);
}
extern "C" {
    pub fn resume_data_vio_pool(pool: *mut data_vio_pool, completion: *mut vdo_completion);
}
extern "C" {
    pub fn dump_data_vio_pool(pool: *mut data_vio_pool, dump_vios: bool);
}
extern "C" {
    pub fn get_data_vio_pool_active_requests(pool: *mut data_vio_pool) -> data_vio_count_t;
}
extern "C" {
    pub fn get_data_vio_pool_request_limit(pool: *mut data_vio_pool) -> data_vio_count_t;
}
extern "C" {
    pub fn get_data_vio_pool_maximum_requests(pool: *mut data_vio_pool) -> data_vio_count_t;
}
extern "C" {
    pub fn complete_data_vio(completion: *mut vdo_completion);
}
extern "C" {
    pub fn handle_data_vio_error(completion: *mut vdo_completion);
}
//
// continue_data_vio_with_error() - Set an error code and then continue processing a data_vio.
//
// This will not mask older errors. This function can be called with a success code, but it is more
// efficient to call continue_data_vio() if the caller knows the result was a success.
//
extern "C" {
    pub fn get_data_vio_operation_name(data_vio: *mut data_vio) -> *const char  __must_check;
}
//
// It's odd to use the LBN, but converting the record name to hex is a bit clunky for an
// inline, and the LBN better than nothing as an identifier.
//
// launch_data_vio_hash_zone_callback() - Set a callback as a hash zone operation and invoke it
// immediately.
//
// launch_data_vio_logical_callback() - Set a callback as a logical block operation and invoke it
// immediately.
//
// launch_data_vio_allocated_zone_callback() - Set a callback as a physical block operation in a
// data_vio's allocated zone and queue the data_vio and
// invoke it immediately.
//
// launch_data_vio_duplicate_zone_callback() - Set a callback as a physical block operation in a
// data_vio's duplicate zone and queue the data_vio and
// invoke it immediately.
//
// launch_data_vio_journal_callback() - Set a callback as a journal operation and invoke it
// immediately.
//
// launch_data_vio_packer_callback() - Set a callback as a packer operation and invoke it
// immediately.
//
// launch_data_vio_cpu_callback() - Set a callback to run on the CPU queues and invoke it
// immediately.
//
// launch_data_vio_bio_zone_callback() - Set a callback as a bio zone operation and invoke it
// immediately.
//
// launch_data_vio_on_bio_ack_queue() - If the vdo uses a bio_ack queue, set a callback to run on
// it and invoke it immediately, otherwise, just run the
// callback on the current thread.
//
extern "C" {
    pub fn release_data_vio_allocation_lock(data_vio: *mut data_vio, reset: bool);
}
extern "C" {
    pub fn write_data_vio(data_vio: *mut data_vio);
}
extern "C" {
    pub fn launch_compress_data_vio(data_vio: *mut data_vio);
}
extern "C" {
    pub fn continue_data_vio_with_block_map_slot(completion: *mut vdo_completion);
}
