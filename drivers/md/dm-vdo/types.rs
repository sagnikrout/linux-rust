//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/types.h
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

// A size type in blocks.
pub type block_count_t = u64;
// The size of a block.
pub type block_size_t = u16;
// A counter for data_vios
pub type data_vio_count_t = u16;
// A height within a tree.
pub type height_t = u8;
// The logical block number as used by the consumer.
pub type logical_block_number_t = u64;
// The type of the nonce used to identify instances of VDO.
pub type nonce_t = u64;
// A size in pages.
pub type page_count_t = u32;
// A page number.
pub type page_number_t = u32;
//
// The physical (well, less logical) block number at which the block is found on the underlying
// device.
//
pub type physical_block_number_t = u64;
// A count of tree roots.
pub type root_count_t = u8;
// A number of sectors.
pub type sector_count_t = u8;
// A sequence number.
pub type sequence_number_t = u64;
// The offset of a block within a slab.
pub type slab_block_number = u32;
// A size type in slabs.
pub type slab_count_t = u16;
// A slot in a bin or block map page.
pub type slot_number_t = u16;
// typedef thread_count_t - A thread counter.
pub type thread_count_t = u8;
// typedef thread_id_t - A thread ID, vdo threads are numbered sequentially from 0.
pub type thread_id_t = u8;
// A zone counter
pub type zone_count_t = u8;
// The following enums are persisted on storage, so the values must be preserved.
// The current operating mode of the VDO.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_state {
    VDO_DIRTY = 0,
    VDO_NEW = 1,
    VDO_CLEAN = 2,
    VDO_READ_ONLY_MODE = 3,
    VDO_FORCE_REBUILD = 4,
    VDO_RECOVERING = 5,
    VDO_REPLAYING = 6, /* VDO_REPLAYING is never set anymore, but retained for upgrade */
    VDO_REBUILD_FOR_UPGRADE = 7,

// Keep VDO_STATE_COUNT at the bottom.
    VDO_STATE_COUNT
}

//
// vdo_state_requires_read_only_rebuild() - Check whether a vdo_state indicates
// that a read-only rebuild is required.
// @state: The vdo_state to check.
//
// Return: true if the state indicates a rebuild is required
//
// vdo_state_requires_recovery() - Check whether a vdo state indicates that recovery is needed.
// @state: The state to check.
//
// Return: true if the state indicates a recovery is required
//
// The current operation on a physical block (from the point of view of the recovery journal, slab
// journals, and reference counts.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum journal_operation {
    VDO_JOURNAL_DATA_REMAPPING = 0,
    VDO_JOURNAL_BLOCK_MAP_REMAPPING = 1,
    } __packed;

// Partition IDs encoded in the volume layout in the super block.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum partition_id {
    VDO_BLOCK_MAP_PARTITION = 0,
    VDO_SLAB_DEPOT_PARTITION = 1,
    VDO_RECOVERY_JOURNAL_PARTITION = 2,
    VDO_SLAB_SUMMARY_PARTITION = 3,
    } __packed;

// Metadata types for the vdo.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_metadata_type {
    VDO_METADATA_RECOVERY_JOURNAL = 1,
    VDO_METADATA_SLAB_JOURNAL = 2,
    VDO_METADATA_RECOVERY_JOURNAL_2 = 3,
    } __packed;

// A position in the block map where a block map entry is stored.
    struct block_map_slot {
    physical_block_number_t pbn;
    slot_number_t slot;
}

//
// Four bits of each five-byte block map entry contain a mapping state value used to distinguish
// unmapped or discarded logical blocks (which are treated as mapped to the zero block) from entries
// that have been mapped to a physical block, including the zero block.
//
// FIXME: these should maybe be defines.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_mapping_state {
    VDO_MAPPING_STATE_UNMAPPED = 0, /* Must be zero to be the default value */
    VDO_MAPPING_STATE_UNCOMPRESSED = 1, /* A normal (uncompressed) block */
    VDO_MAPPING_STATE_COMPRESSED_BASE = 2, /* Compressed in slot 0 */
    VDO_MAPPING_STATE_COMPRESSED_MAX = 15, /* Compressed in slot 13 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_location {
    pub pbn: physical_block_number_t,
    pub state: block_mapping_state,
}

// The configuration of a single slab derived from the configured block size and slab size.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_config {
// total number of blocks in the slab
    pub slab_blocks: block_count_t,
// number of blocks available for data
    pub data_blocks: block_count_t,
// number of blocks for reference counts
    pub reference_count_blocks: block_count_t,
// number of blocks for the slab journal
    pub slab_journal_blocks: block_count_t,
//
// Number of blocks after which the slab journal starts pushing out a reference_block for
// each new entry it receives.
//
    pub slab_journal_flushing_threshold: block_count_t,
//
// Number of blocks after which the slab journal pushes out all reference_blocks and makes
// all vios wait.
//
    pub slab_journal_blocking_threshold: block_count_t,
// Number of blocks after which the slab must be scrubbed before coming online.
    pub slab_journal_scrubbing_threshold: block_count_t,
    pub __packed: },
//
// This structure is memcmp'd for equality. Keep it packed and don't add any fields that are not
// properly set in both extant and parsed configs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_count_config {
    pub bio_ack_threads: c_uint,
    pub bio_threads: c_uint,
    pub bio_rotation_interval: c_uint,
    pub cpu_threads: c_uint,
    pub logical_zones: c_uint,
    pub physical_zones: c_uint,
    pub hash_zones: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_config {
    pub owning_target: *mut dm_target,
    pub owned_device: *mut dm_dev,
    pub vdo: *mut vdo,
// All configs referencing a layer are kept on a list in the layer
    pub config_list: list_head,
    pub original_string: *mut c_char,
    pub version: c_uint,
    pub parent_device_name: *mut c_char,
    pub physical_blocks: block_count_t,
//
// This is the number of logical blocks from VDO's internal point of view. It is the number
// of 4K blocks regardless of the value of the logical_block_size parameter below.
//
    pub logical_blocks: block_count_t,
    pub logical_block_size: c_uint,
    pub cache_size: c_uint,
    pub block_map_maximum_age: c_uint,
    pub deduplication: bool,
    pub compression: bool,
    pub thread_counts: thread_count_config,
    pub max_discard_blocks: block_count_t,
    pub slab_blocks: block_count_t,
    pub index_memory: c_int,
    pub index_sparse: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_completion_type {
// Keep VDO_UNSET_COMPLETION_TYPE at the top.
    VDO_UNSET_COMPLETION_TYPE,
    VDO_ACTION_COMPLETION,
    VDO_ADMIN_COMPLETION,
    VDO_BLOCK_ALLOCATOR_COMPLETION,
    VDO_DATA_VIO_POOL_COMPLETION,
    VDO_DECREMENT_COMPLETION,
    VDO_FLUSH_COMPLETION,
    VDO_FLUSH_NOTIFICATION_COMPLETION,
    VDO_GENERATION_FLUSHED_COMPLETION,
    VDO_HASH_ZONE_COMPLETION,
    VDO_HASH_ZONES_COMPLETION,
    VDO_LOCK_COUNTER_COMPLETION,
    VDO_PAGE_COMPLETION,
    VDO_READ_ONLY_MODE_COMPLETION,
    VDO_REPAIR_COMPLETION,
    VDO_SYNC_COMPLETION,
    VIO_COMPLETION,
    } __packed;

    struct vdo_completion;

//
// typedef vdo_action_fn - An asynchronous VDO operation.
// @completion: The completion of the operation.
//
    typedef void (*vdo_action_fn)(struct vdo_completion *completion);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_completion_priority {
    BIO_ACK_Q_ACK_PRIORITY = 0,
    BIO_ACK_Q_MAX_PRIORITY = 0,
    BIO_Q_COMPRESSED_DATA_PRIORITY = 0,
    BIO_Q_DATA_PRIORITY = 0,
    BIO_Q_FLUSH_PRIORITY = 2,
    BIO_Q_HIGH_PRIORITY = 2,
    BIO_Q_METADATA_PRIORITY = 1,
    BIO_Q_VERIFY_PRIORITY = 1,
    BIO_Q_MAX_PRIORITY = 2,
    CPU_Q_COMPLETE_VIO_PRIORITY = 0,
    CPU_Q_COMPLETE_READ_PRIORITY = 0,
    CPU_Q_COMPRESS_BLOCK_PRIORITY = 0,
    CPU_Q_EVENT_REPORTER_PRIORITY = 0,
    CPU_Q_HASH_BLOCK_PRIORITY = 0,
    CPU_Q_MAX_PRIORITY = 0,
    UDS_Q_PRIORITY = 0,
    UDS_Q_MAX_PRIORITY = 0,
    VDO_DEFAULT_Q_COMPLETION_PRIORITY = 1,
    VDO_DEFAULT_Q_FLUSH_PRIORITY = 2,
    VDO_DEFAULT_Q_MAP_BIO_PRIORITY = 0,
    VDO_DEFAULT_Q_SYNC_PRIORITY = 2,
    VDO_DEFAULT_Q_VIO_CALLBACK_PRIORITY = 1,
    VDO_DEFAULT_Q_MAX_PRIORITY = 2,
// The maximum allowable priority
    VDO_WORK_Q_MAX_PRIORITY = 2,
// A value which must be out of range for a valid priority
    VDO_WORK_Q_DEFAULT_PRIORITY = VDO_WORK_Q_MAX_PRIORITY + 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_completion {
// The type of completion this is
    pub type: vdo_completion_type,
//
// <code>true</code> once the processing of the operation is complete. This flag should not
// be used by waiters external to the VDO base as it is used to gate calling the callback.
//
    pub complete: bool,
//
// If true, queue this completion on the next callback invocation, even if it is already
// running on the correct thread.
//
    pub requeue: bool,
// The ID of the thread which should run the next callback
    pub callback_thread_id: thread_id_t,
// The result of the operation
    pub result: c_int,
// The VDO on which this completion operates
    pub vdo: *mut vdo,
// The callback which will be called once the operation is complete
    pub callback: vdo_action_fn,
// Callback which, if set, will be called if an error result is set
    pub error_handler: vdo_action_fn,
// The parent object, if any, that spawned this completion
    pub parent: *mut c_void,
// Entry link for lock-free work queue
    pub work_queue_entry_link: funnel_queue_entry,
    pub priority: vdo_completion_priority,
    pub my_queue: *mut vdo_work_queue,
}

// vio types for statistics and instrumentation.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vio_type {
    VIO_TYPE_UNINITIALIZED = 0,
    VIO_TYPE_DATA,
    VIO_TYPE_BLOCK_ALLOCATOR,
    VIO_TYPE_BLOCK_MAP,
    VIO_TYPE_BLOCK_MAP_INTERIOR,
    VIO_TYPE_GEOMETRY,
    VIO_TYPE_PARTITION_COPY,
    VIO_TYPE_RECOVERY_JOURNAL,
    VIO_TYPE_SLAB_JOURNAL,
    VIO_TYPE_SLAB_SUMMARY,
    VIO_TYPE_SUPER_BLOCK,
    } __packed;

// Priority levels for asynchronous I/O operations performed on a vio.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vio_priority {
    VIO_PRIORITY_LOW = 0,
    VIO_PRIORITY_DATA = VIO_PRIORITY_LOW,
    VIO_PRIORITY_COMPRESSED_DATA = VIO_PRIORITY_DATA,
    VIO_PRIORITY_METADATA,
    VIO_PRIORITY_HIGH,
    } __packed;

//
// A wrapper for a bio. All I/O to the storage below a vdo is conducted via vios.
//
    struct vio {
// The completion for this vio
    struct vdo_completion completion;

// The bio zone in which I/O should be processed
    zone_count_t bio_zone;

// The queueing priority of the vio operation
    enum vio_priority priority;

// The vio type is used for statistics and instrumentation.
    enum vio_type type;

// The size of this vio in blocks
    unsigned int block_count;

// The amount of data to be read or written, in bytes
    unsigned int io_size;

// The data being read or written.
    char *data;

// The VDO-owned bio to use for all IO for this vio
    struct bio *bio;

//
// A list of enqueued bios with consecutive block numbers, stored by vdo_submit_bio() under
// the first-enqueued vio. The other vios are found via their bio entries in this list, and
// are not added to the work queue as separate completions.
//
    struct bio_list bios_merged;
}
