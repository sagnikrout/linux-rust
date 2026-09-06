//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/vdo.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum notifier_state {
// Notifications are allowed but not in progress
    MAY_NOTIFY,
// A notification is in progress
    NOTIFYING,
// Notifications are not allowed
    MAY_NOT_NOTIFY,
// A notification has completed
    NOTIFIED,
}

//
// typedef vdo_read_only_notification_fn - A function to notify a listener that the VDO has gone
// read-only.
// @listener: The object to notify.
// @parent: The completion to notify in order to acknowledge the notification.
//
extern "C" {
    pub fn void(listener: *mut *mut vdo_read_only_notification_fn)(void, parent: *mut vdo_completion) -> typedef;
}
//
// An object to be notified when the VDO enters read-only mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_only_listener {
// The listener
    pub listener: *mut c_void,
// The method to call to notify the listener
    pub notify: vdo_read_only_notification_fn,
// A pointer to the next listener
    pub next: *mut read_only_listener,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_thread {
    pub vdo: *mut vdo,
    pub thread_id: thread_id_t,
    pub queue: *mut vdo_work_queue,
//
// Each thread maintains its own notion of whether the VDO is read-only so that the
// read-only state can be checked from any base thread without worrying about
// synchronization or thread safety. This does mean that knowledge of the VDO going
// read-only does not occur simultaneously across the VDO's threads, but that does not seem
// to cause any problems.
//
    pub is_read_only: bool,
//
// A list of objects waiting to be notified on this thread that the VDO has entered
// read-only mode.
//
    pub listeners: *mut read_only_listener,
    pub allocating_thread: registered_thread,
}

// Keep struct bio statistics atomically
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atomic_bio_stats {
    pub /: *mut *mut atomic64_t read; / Number of not REQ_WRITE bios,
    pub /: *mut *mut atomic64_t write; / Number of REQ_WRITE bios,
    pub /: *mut *mut atomic64_t discard; / Number of REQ_DISCARD bios,
    pub /: *mut *mut atomic64_t flush; / Number of REQ_FLUSH bios,
    pub /: *mut *mut atomic64_t empty_flush; / Number of REQ_PREFLUSH bios without data,
    pub /: *mut *mut atomic64_t fua; / Number of REQ_FUA bios,
}

// Counters are atomic since updates can arrive concurrently from arbitrary threads.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atomic_statistics {
    pub bios_submitted: core::sync::atomic::AtomicI64,
    pub bios_completed: core::sync::atomic::AtomicI64,
    pub flush_out: core::sync::atomic::AtomicI64,
    pub invalid_advice_pbn_count: core::sync::atomic::AtomicI64,
    pub no_space_error_count: core::sync::atomic::AtomicI64,
    pub read_only_error_count: core::sync::atomic::AtomicI64,
    pub bios_in: atomic_bio_stats,
    pub bios_in_partial: atomic_bio_stats,
    pub bios_out: atomic_bio_stats,
    pub bios_out_completed: atomic_bio_stats,
    pub bios_acknowledged: atomic_bio_stats,
    pub bios_acknowledged_partial: atomic_bio_stats,
    pub bios_meta: atomic_bio_stats,
    pub bios_meta_completed: atomic_bio_stats,
    pub bios_journal: atomic_bio_stats,
    pub bios_journal_completed: atomic_bio_stats,
    pub bios_page_cache: atomic_bio_stats,
    pub bios_page_cache_completed: atomic_bio_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_only_notifier {
// The completion for entering read-only mode
    pub completion: vdo_completion,
// A completion waiting for notifications to be drained or enabled
    pub waiter: *mut vdo_completion,
// Lock to protect the next two fields
    pub lock: spinlock_t,
// The code of the error which put the VDO into read-only mode
    pub read_only_error: c_int,
// The current state of the notifier (values described above)
    pub state: notifier_state,
}

//
// The thread ID returned when the current thread is not a vdo thread, or can not be determined
// (usually due to being at interrupt context).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_config {
    pub logical_zone_count: zone_count_t,
    pub physical_zone_count: zone_count_t,
    pub hash_zone_count: zone_count_t,
    pub bio_thread_count: thread_count_t,
    pub thread_count: thread_count_t,
    pub admin_thread: thread_id_t,
    pub journal_thread: thread_id_t,
    pub packer_thread: thread_id_t,
    pub dedupe_thread: thread_id_t,
    pub bio_ack_thread: thread_id_t,
    pub cpu_thread: thread_id_t,
    pub logical_threads: *mut thread_id_t,
    pub physical_threads: *mut thread_id_t,
    pub hash_zone_threads: *mut thread_id_t,
    pub bio_threads: *mut thread_id_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_geometry_block {
// The vio for reading and writing the geometry block to disk
    pub vio: vio,
// A buffer to hold the geometry block
    pub buffer: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_super_block {
// The vio for reading and writing the super block to disk
    pub vio: vio,
// A buffer to hold the super block
    pub buffer: *mut u8,
// Whether this super block may not be written
    pub unwritable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_administrator {
    pub completion: vdo_completion,
    pub state: admin_state,
    pub busy: core::sync::atomic::AtomicI32,
    pub phase: u32,
    pub callback_sync: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo {
    pub thread_name_prefix: [c_char; MAX_VDO_WORK_QUEUE_NAME_LEN],
    pub threads: *mut vdo_thread,
    pub action: vdo_action_fn,
    pub completion: *mut vdo_completion,
    pub vio_tracer: *mut vio_tracer,
// The atomic version of the state of this vdo
    pub state: core::sync::atomic::AtomicI32,
// The full state of all components
    pub states: vdo_component_states,
//
// A counter value to attach to thread names and log messages to identify the individual
// device.
//
    pub instance: c_uint,
// The read-only notifier
    pub read_only_notifier: read_only_notifier,
// The load-time configuration of this vdo
    pub device_config: *mut device_config,
// The thread mapping
    pub thread_config: thread_config,
// The geometry block
    pub geometry_block: vdo_geometry_block,
// The super block
    pub super_block: vdo_super_block,
// The partitioning of the underlying storage
    pub layout: layout,
    pub next_layout: layout,
    pub partition_copier: *mut dm_kcopyd_client,
// The block map
    pub block_map: *mut block_map,
// The journal for block map recovery
    pub recovery_journal: *mut recovery_journal,
// The slab depot
    pub depot: *mut slab_depot,
// The compressed-block packer
    pub packer: *mut packer,
// Whether incoming data should be compressed
    pub compressing: bool,
// The handler for flush requests
    pub flusher: *mut flusher,
// The state the vdo was in when loaded (primarily for unit tests)
    pub load_state: vdo_state,
// The logical zones of this vdo
    pub logical_zones: *mut logical_zones,
// The physical zones of this vdo
    pub physical_zones: *mut physical_zones,
// The hash lock zones of this vdo
    pub hash_zones: *mut hash_zones,
// Bio submission manager used for sending bios to the storage device.
    pub io_submitter: *mut io_submitter,
// The pool of data_vios for servicing incoming bios
    pub data_vio_pool: *mut data_vio_pool,
// The manager for administrative operations
    pub admin: vdo_administrator,
// Flags controlling administrative operations
    pub suspend_type: *const admin_state_code,
    pub allocations_allowed: bool,
    pub dump_on_shutdown: bool,
    pub needs_formatting: bool,
    pub processing_message: core::sync::atomic::AtomicI32,
//
// Statistics
// Atomic stats counters
//
    pub stats: atomic_statistics,
// Used to gather statistics without allocating memory
    pub stats_buffer: vdo_statistics,
// Protects the stats_buffer
    pub stats_mutex: mutex,
// A list of all device_configs referencing this vdo
    pub device_config_list: list_head,
// This VDO's list entry for the device registry
    pub registration: list_head,
// Underlying block device info.
    pub starting_sector_offset: u64,
    pub geometry: volume_geometry,
// N blobs of context data for LZ4 code, one per CPU thread.
    pub compression_context: *mut c_char,
}

//
// vdo_uses_bio_ack_queue() - Indicate whether the vdo is configured to use a separate work queue
// for acknowledging received and processed bios.
// @vdo: The vdo.
//
// Note that this directly controls the handling of write operations, but the compile-time flag
// VDO_USE_BIO_ACK_QUEUE_FOR_READ is also checked for read operations.
//
// Return: Whether a bio-acknowledgement work queue is in use.
//
// typedef vdo_filter_fn - Method type for vdo matching methods.
// @vdo: The vdo to match.
// @context: A parameter for the filter to use.
//
// Return: True if the vdo matches the filter criteria, false if it doesn't.
//
extern "C" {
    pub fn bool(vdo: *mut *mut vdo_filter_fn)(struct vdo, context: *const c_void) -> typedef;
}
extern "C" {
    pub fn vdo_initialize_device_registry_once();
}
extern "C" {
    pub fn vdo_find_matching(filter: vdo_filter_fn, context: *const c_void) -> *mut vdo  __must_check;
}
extern "C" {
    pub fn vdo_make_thread(_arg: vdo, _arg: thread_id, _arg: NULL, _arg: 1, _arg: NULL) -> return;
}
extern "C" {
    pub fn vdo_destroy(vdo: *mut vdo);
}
extern "C" {
    pub fn vdo_format_components(vdo: *mut vdo) -> int __must_check;
}
extern "C" {
    pub fn vdo_format_super_block(vdo: *mut vdo, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_load_super_block(vdo: *mut vdo, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_get_backing_device(vdo: *const vdo) -> *mut block_device  __must_check;
}
extern "C" {
    pub fn vdo_get_device_name(target: *const dm_target) -> *const char  __must_check;
}
extern "C" {
    pub fn vdo_synchronous_flush(vdo: *mut vdo) -> int __must_check;
}
extern "C" {
    pub fn vdo_get_admin_state(vdo: *const vdo) -> *const admin_state_code  __must_check;
}
extern "C" {
    pub fn vdo_set_compressing(vdo: *mut vdo, enable: bool) -> bool;
}
extern "C" {
    pub fn vdo_get_compressing(vdo: *mut vdo) -> bool;
}
extern "C" {
    pub fn vdo_fetch_statistics(vdo: *mut vdo, stats: *mut vdo_statistics);
}
extern "C" {
    pub fn vdo_get_callback_thread_id() -> thread_id_t;
}
extern "C" {
    pub fn vdo_get_state(vdo: *const vdo) -> vdo_state __must_check;
}
extern "C" {
    pub fn vdo_set_state(vdo: *mut vdo, state: vdo_state);
}
extern "C" {
    pub fn vdo_clear_layout(vdo: *mut vdo) -> c_int;
}
extern "C" {
    pub fn vdo_save_geometry_block(vdo: *mut vdo, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_save_super_block(vdo: *mut vdo, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_save_components(vdo: *mut vdo, parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_enable_read_only_entry(vdo: *mut vdo) -> c_int;
}
extern "C" {
    pub fn vdo_wait_until_not_entering_read_only_mode(parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_allow_read_only_mode_entry(parent: *mut vdo_completion);
}
extern "C" {
    pub fn vdo_enter_read_only_mode(vdo: *mut vdo, error_code: c_int);
}
extern "C" {
    pub fn vdo_is_read_only(vdo: *mut vdo) -> bool __must_check;
}
extern "C" {
    pub fn vdo_in_read_only_mode(vdo: *const vdo) -> bool __must_check;
}
extern "C" {
    pub fn vdo_in_recovery_mode(vdo: *const vdo) -> bool __must_check;
}
extern "C" {
    pub fn vdo_enter_recovery_mode(vdo: *mut vdo);
}
extern "C" {
    pub fn vdo_assert_on_admin_thread(vdo: *const vdo, name: *const c_char);
}
extern "C" {
    pub fn vdo_dump_status(vdo: *const vdo);
}
