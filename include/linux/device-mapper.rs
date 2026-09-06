//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-mapper.h
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
// Copyright (C) 2001 Sistina Software (UK) Limited.
// Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

//
// Type of table, mapped_device's mempool and request_queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_queue_mode {
    DM_TYPE_NONE		 = 0,
    DM_TYPE_BIO_BASED	 = 1,
    DM_TYPE_REQUEST_BASED	 = 2,
    DM_TYPE_DAX_BIO_BASED	 = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union map_info {
    pub ptr: *mut c_void,
}

//
// In the constructor the target parameter will already have the
// table, type, begin and len fields filled in.
//
// The destructor doesn't need to free the dm_target, just
// anything hidden ti->private.
//
extern "C" {
    pub fn void(ti: *mut *mut dm_dtr_fn) (struct dm_target) -> typedef;
}
//
// The map function must return:
// < 0: error
// = 0: The target will handle the io by resubmitting it later
// = 1: simple remap complete
// = 2: The target wants to push back the io
//
extern "C" {
    pub fn int(ti: *mut *mut dm_map_fn) (struct dm_target, bio: *mut bio) -> typedef;
}
//
// Returns:
// < 0 : error (currently ignored)
// 0   : ended successfully
// 1   : for some reason the io has still not completed (eg,
// multipath target might want to requeue a failed io).
// 2   : The target wants to push back the io
//
extern "C" {
    pub fn void(ti: *mut *mut dm_presuspend_fn) (struct dm_target) -> typedef;
}
extern "C" {
    pub fn void(ti: *mut *mut dm_presuspend_undo_fn) (struct dm_target) -> typedef;
}
extern "C" {
    pub fn void(ti: *mut *mut dm_postsuspend_fn) (struct dm_target) -> typedef;
}
extern "C" {
    pub fn int(ti: *mut *mut dm_preresume_fn) (struct dm_target) -> typedef;
}
extern "C" {
    pub fn void(ti: *mut *mut dm_resume_fn) (struct dm_target) -> typedef;
}
//
// Called with *forward == true. If it remains true, the ioctl should be
// forwarded to bdev. If it is reset to false, the target already fully handled
// the ioctl and the return value is the return value for the whole ioctl.
//

//
// Define dm_report_zones_fn so that targets can assign to NULL if
// CONFIG_BLK_DEV_ZONED disabled. Otherwise each target needs to do
// awkward #ifdefs in their target_type, etc.
//
extern "C" {
    pub fn int(dummy: *mut *mut dm_report_zones_fn) (struct dm_target) -> typedef;
}

//
// These iteration functions are typically used to check (and combine)
// properties of underlying devices.
// E.g. Does at least one underlying device support flush?
// Does any underlying device not support WRITE_SAME?
//
// The callout function is called once for each contiguous section of
// an underlying device.  State can be maintained in *data.
// Return non-zero to stop iterating through any further devices.
//
// This function must iterate through each section of device used by the
// target until it encounters a non-zero return code, which it then returns.
// Returns zero if no callout returned non-zero.
//
// Returns:
// 0: The target can handle the next I/O immediately.
// 1: The target can't handle the next I/O immediately.
//
extern "C" {
    pub fn int(ti: *mut *mut dm_busy_fn) (struct dm_target) -> typedef;
}
//
// Returns:
// < 0 : error
// >= 0 : the number of bytes accessible at the address
//
// Returns:
// != 0 : number of bytes transferred
// 0    : recovery write failed
//
extern "C" {
    pub fn dm_error(message: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_dev {
    pub bdev: *mut block_device,
    pub bdev_file: *mut file,
    pub dax_dev: *mut dax_device,
    pub mode: blk_mode_t,
    pub name: [c_char; 16],
}

//
// Constructors should call these functions to ensure destination devices
// are opened/closed correctly.
//
extern "C" {
    pub fn dm_put_device(ti: *mut dm_target, d: *mut dm_dev);
}
//
// Helper function for getting devices
//
extern "C" {
    pub fn dm_devt_from_path(path: *const c_char, dev_p: *mut dev_t) -> c_int;
}
//
// Information about a target type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_type {
    pub features: u64,
    pub name: *const c_char,
    pub module: *mut module,
    pub version: [c_uint; 3],
    pub ctr: dm_ctr_fn,
    pub dtr: dm_dtr_fn,
    pub map: dm_map_fn,
    pub clone_and_map_rq: dm_clone_and_map_request_fn,
    pub release_clone_rq: dm_release_clone_request_fn,
    pub end_io: dm_endio_fn,
    pub rq_end_io: dm_request_endio_fn,
    pub presuspend: dm_presuspend_fn,
    pub presuspend_undo: dm_presuspend_undo_fn,
    pub postsuspend: dm_postsuspend_fn,
    pub preresume: dm_preresume_fn,
    pub resume: dm_resume_fn,
    pub status: dm_status_fn,
    pub message: dm_message_fn,
    pub prepare_ioctl: dm_prepare_ioctl_fn,
    pub report_zones: dm_report_zones_fn,
    pub busy: dm_busy_fn,
    pub iterate_devices: dm_iterate_devices_fn,
    pub io_hints: dm_io_hints_fn,
    pub direct_access: dm_dax_direct_access_fn,
    pub dax_zero_page_range: dm_dax_zero_page_range_fn,
    pub dax_recovery_write: dm_dax_recovery_write_fn,
// For internal device-mapper use.
    pub list: list_head,
}

//
// Target features
//
// Any table that contains an instance of this target must have only one.
//
pub const DM_TARGET_SINGLETON: c_uint = 0x00000001;

//
// Indicates that a target does not support read-only devices.
//
pub const DM_TARGET_ALWAYS_WRITEABLE: c_uint = 0x00000002;

//
// Any device that contains a table with an instance of this target may never
// have tables containing any different target type.
//
pub const DM_TARGET_IMMUTABLE: c_uint = 0x00000004;

//
// Indicates that a target may replace any target; even immutable targets.
// .map, .map_rq, .clone_and_map_rq and .release_clone_rq are all defined.
//
pub const DM_TARGET_WILDCARD: c_uint = 0x00000008;

//
// A target implements own bio data integrity.
//
pub const DM_TARGET_INTEGRITY: c_uint = 0x00000010;

//
// A target passes integrity data to the lower device.
//
pub const DM_TARGET_PASSES_INTEGRITY: c_uint = 0x00000020;

//
// Indicates support for zoned block devices:
// - DM_TARGET_ZONED_HM: the target also supports host-managed zoned
// block devices but does not support combining different zoned models.
// - DM_TARGET_MIXED_ZONED_MODEL: the target supports combining multiple
// devices with different zoned models.
//

pub const DM_TARGET_ZONED_HM: c_uint = 0x00000040;

pub const DM_TARGET_ZONED_HM: c_uint = 0x00000000;

//
// A target handles REQ_NOWAIT
//
pub const DM_TARGET_NOWAIT: c_uint = 0x00000080;

//
// A target supports passing through inline crypto support.
//
pub const DM_TARGET_PASSES_CRYPTO: c_uint = 0x00000100;

pub const DM_TARGET_MIXED_ZONED_MODEL: c_uint = 0x00000200;

pub const DM_TARGET_MIXED_ZONED_MODEL: c_uint = 0x00000000;

pub const DM_TARGET_ATOMIC_WRITES: c_uint = 0x00000400;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_target {
    pub table: *mut dm_table,
    pub type: *mut target_type,
// target limits
    pub begin: sector_t,
    pub len: sector_t,
// If non-zero, maximum size of I/O submitted to a target.
    pub max_io_len: u32,
//
// A number of zero-length barrier bios that will be submitted
// to the target for the purpose of flushing cache.
//
// The bio number can be accessed with dm_bio_get_target_bio_nr.
// It is a responsibility of the target driver to remap these bios
// to the real underlying devices.
//
    pub num_flush_bios: c_uint,
//
// The number of discard bios that will be submitted to the target.
// The bio number can be accessed with dm_bio_get_target_bio_nr.
//
    pub num_discard_bios: c_uint,
//
// The number of secure erase bios that will be submitted to the target.
// The bio number can be accessed with dm_bio_get_target_bio_nr.
//
    pub num_secure_erase_bios: c_uint,
//
// The number of WRITE ZEROES bios that will be submitted to the target.
// The bio number can be accessed with dm_bio_get_target_bio_nr.
//
    pub num_write_zeroes_bios: c_uint,
//
// The minimum number of extra bytes allocated in each io for the
// target to use.
//
    pub per_io_data_size: c_uint,
// target specific data
    pub private: *mut c_void,
// Used to provide an error string from the ctr
    pub error: *mut c_char,
//
// Set if this target needs to receive flushes regardless of
// whether or not its underlying devices have support.
//
    pub flush_supported:1: bool,
//
// Set if this target needs to receive discards regardless of
// whether or not its underlying devices have support.
//
    pub discards_supported:1: bool,
//
// Automatically set by dm-core if this target supports
// REQ_OP_ZONE_RESET_ALL. Otherwise, this operation will be emulated
// using REQ_OP_ZONE_RESET. Target drivers must not set this manually.
//
    pub zone_reset_all_supported:1: bool,
//
// Set if this target requires that discards be split on
// 'max_discard_sectors' boundaries.
//
    pub max_discard_granularity:1: bool,
//
// Set if we need to limit the number of in-flight bios when swapping.
//
    pub limit_swap_bios:1: bool,
//
// Set if this target implements a zoned device and needs emulation of
// zone append operations using regular writes.
//
    pub emulate_zone_append:1: bool,
//
// Set if the target will submit IO using dm_submit_bio_remap()
// after returning DM_MAPIO_SUBMITTED from its map function.
//
    pub accounts_remapped_io:1: bool,
//
// Set if the target will submit the DM bio without first calling
// bio_set_dev(). NOTE: ideally a target should _not_ need this.
//
    pub needs_bio_set_dev:1: bool,
//
// Set if the target supports flush optimization. If all the targets in
// a table have flush_bypasses_map set, the dm core will not send
// flushes to the targets via a ->map method. It will iterate over
// dm_table->devices and send flushes to the devices directly. This
// optimization reduces the number of flushes being sent when multiple
// targets in a table use the same underlying device.
//
// This optimization may be enabled on targets that just pass the
// flushes to the underlying devices without performing any other
// actions on the flush request. Currently, dm-linear and dm-stripe
// support it.
//
    pub flush_bypasses_map:1: bool,
//
// Set if the target calls bio_integrity_alloc on bios received
// in the map method.
//
    pub mempool_needs_integrity:1: bool,
}

extern "C" {
    pub fn dm_bio_get_target_bio_nr(bio: *const bio) -> c_uint;
}
extern "C" {
    pub fn dm_start_time_ns_from_clone(bio: *mut bio) -> u64;
}
extern "C" {
    pub fn dm_register_target(t: *mut target_type) -> c_int;
}
extern "C" {
    pub fn dm_unregister_target(t: *mut target_type);
}
//
// Target argument parsing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_arg_set {
    pub argc: c_uint,
    pub argv: *mut c_char,
}

//
// The minimum and maximum value of a numeric argument, together with
// the error message to use if the number is found to be outside that range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_arg {
    pub min: c_uint,
    pub max: c_uint,
    pub error: *mut c_char,
}

//
// Validate the next argument, either returning it as *value or, if invalid,
// returning -EINVAL and setting *error.
//
// Process the next argument as the start of a group containing between
// arg->min and arg->max further arguments. Either return the size as
// *num_args or, if invalid, return -EINVAL and set *error.
//
// Return the current argument and shift to the next.
//
// Move through num_args arguments.
//
extern "C" {
    pub fn dm_consume_args(as: *mut dm_arg_set, num_args: c_uint);
}
//
// ----------------------------------------------------------------
// Functions for creating and manipulating mapped devices.
// Drop the reference with dm_put when you finish with the object.
// ----------------------------------------------------------------
//
// DM_ANY_MINOR chooses the next available minor number.
//

extern "C" {
    pub fn dm_create(minor: c_int, md: *mut mapped_device) -> c_int;
}
//
// Reference counting for md.
//
extern "C" {
    pub fn dm_get(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_hold(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_put(md: *mut mapped_device);
}
//
// An arbitrary pointer may be stored alongside a mapped device.
//
extern "C" {
    pub fn dm_set_mdptr(md: *mut mapped_device, ptr: *mut c_void);
}
//
// A device can still be used while suspended, but I/O is deferred.
//
extern "C" {
    pub fn dm_suspend(md: *mut mapped_device, suspend_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn dm_resume(md: *mut mapped_device) -> c_int;
}
//
// Event functions.
//
extern "C" {
    pub fn dm_get_event_nr(md: *mut mapped_device) -> u32;
}
extern "C" {
    pub fn dm_wait_event(md: *mut mapped_device, event_nr: c_int) -> c_int;
}
extern "C" {
    pub fn dm_next_uevent_seq(md: *mut mapped_device) -> u32;
}
extern "C" {
    pub fn dm_uevent_add(md: *mut mapped_device, elist: *mut list_head);
}
//
// Info functions.
//
extern "C" {
    pub fn dm_copy_name_and_uuid(md: *mut mapped_device, name: *mut c_char, uuid: *mut c_char) -> c_int;
}
extern "C" {
    pub fn dm_suspended(ti: *mut dm_target) -> c_int;
}
extern "C" {
    pub fn dm_post_suspending(ti: *mut dm_target) -> c_int;
}
extern "C" {
    pub fn dm_noflush_suspending(ti: *mut dm_target) -> c_int;
}
extern "C" {
    pub fn dm_accept_partial_bio(bio: *mut bio, n_sectors: c_uint);
}
extern "C" {
    pub fn dm_submit_bio_remap(clone: *mut bio, tgt_clone: *mut bio);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_report_zones_args {
    pub tgt: *mut dm_target,
    pub disk: *mut gendisk,
    pub next_sector: sector_t,
    pub zone_idx: c_uint,
// for block layer ->report_zones
    pub rep_args: *mut blk_report_zones_args,
// for internal users
    pub cb: report_zones_cb,
    pub data: *mut c_void,
// must be filled by ->report_zones before calling dm_report_zones_cb
    pub start: sector_t,
}

//
// Device mapper functions to parse and create devices specified by the
// parameter "dm-mod.create="
//
// Geometry functions.
//
extern "C" {
    pub fn dm_get_geometry(md: *mut mapped_device, geo: *mut hd_geometry) -> c_int;
}
extern "C" {
    pub fn dm_set_geometry(md: *mut mapped_device, geo: *mut hd_geometry) -> c_int;
}
//
// ---------------------------------------------------------------
// Functions for manipulating device-mapper tables.
// ---------------------------------------------------------------
//
// First create an empty table.
//
// Then call this once for each target.
//
// Target can use this to set the table's type.
// Can only ever be called from a target's ctr.
// Useful for "hybrid" target (supports both bio-based
// and request-based).
//
extern "C" {
    pub fn dm_table_set_type(t: *mut dm_table, type: dm_queue_mode);
}
//
// Finally call this to make the table ready for use.
//
extern "C" {
    pub fn dm_table_complete(t: *mut dm_table) -> c_int;
}
//
// Destroy the table when finished.
//
extern "C" {
    pub fn dm_table_destroy(t: *mut dm_table);
}
//
// Target may require that it is never sent I/O larger than len.
//
extern "C" {
    pub fn dm_set_target_max_io_len(ti: *mut dm_target, len: sector_t) -> int __must_check;
}
//
// Table reference counting.
//
extern "C" {
    pub fn dm_put_live_table(md: *mut mapped_device, srcu_idx: c_int);
}
extern "C" {
    pub fn dm_sync_table(md: *mut mapped_device);
}
//
// Queries
//
extern "C" {
    pub fn dm_table_get_size(t: *mut dm_table) -> sector_t;
}
extern "C" {
    pub fn dm_table_get_mode(t: *mut dm_table) -> blk_mode_t;
}
//
// Trigger an event.
//
extern "C" {
    pub fn dm_table_event(t: *mut dm_table);
}
//
// Run the queue for request-based targets.
//
extern "C" {
    pub fn dm_table_run_md_queue_async(t: *mut dm_table);
}
//
// The device must be suspended before calling this method.
// Returns the previous table, which the caller must destroy.
//
// Table blk_crypto_profile functions
//
extern "C" {
    pub fn dm_destroy_crypto_profile(profile: *mut blk_crypto_profile);
}
//
// ---------------------------------------------------------------
// Macros.
// ---------------------------------------------------------------
//

//
// module_dm() - Helper macro for DM targets that don't do anything
// special in their module_init and module_exit.
// Each module may only use this macro once, and calling it replaces
// module_init() and module_exit().
//
// @name: DM target's name
//

//
// Definitions of return values from target end_io function.
//
pub const DM_ENDIO_DONE: c_int = 0;
pub const DM_ENDIO_INCOMPLETE: c_int = 1;
pub const DM_ENDIO_REQUEUE: c_int = 2;
pub const DM_ENDIO_DELAY_REQUEUE: c_int = 3;
//
// Definitions of return values from target map function.
//
pub const DM_MAPIO_SUBMITTED: c_int = 0;
pub const DM_MAPIO_REMAPPED: c_int = 1;

pub const DM_MAPIO_KILL: c_int = 4;

//
// Ceiling(n / sz)
//

//
// ceiling(n / size) * size
//

//
// Sector offset taken relative to the start of the target instead of
// relative to the start of the device.
//

