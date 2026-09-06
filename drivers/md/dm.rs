//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm.h
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


//
// Internal header file for device mapper
//
// Copyright (C) 2001, 2002 Sistina Software
// Copyright (C) 2004-2006 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

//
// Suspend feature flags
//

//
// Status feature flags
//

//
// List of devices that a metadevice uses and should open/close.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_dev_internal {
    pub list: list_head,
    pub count: refcount_t,
    pub dm_dev: *mut dm_dev,
}

//
// ---------------------------------------------------------------
// Internal table functions.
// ---------------------------------------------------------------
//
extern "C" {
    pub fn dm_table_has_no_data_devices(table: *mut dm_table) -> bool;
}
extern "C" {
    pub fn dm_table_is_wildcard(t: *mut dm_table) -> bool;
}
extern "C" {
    pub fn dm_table_presuspend_targets(t: *mut dm_table);
}
extern "C" {
    pub fn dm_table_presuspend_undo_targets(t: *mut dm_table);
}
extern "C" {
    pub fn dm_table_postsuspend_targets(t: *mut dm_table);
}
extern "C" {
    pub fn dm_table_resume_targets(t: *mut dm_table) -> c_int;
}
extern "C" {
    pub fn dm_table_get_type(t: *mut dm_table) -> dm_queue_mode;
}
extern "C" {
    pub fn dm_table_request_based(t: *mut dm_table) -> bool;
}
extern "C" {
    pub fn dm_lock_md_type(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_unlock_md_type(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_get_md_type(md: *mut mapped_device) -> dm_queue_mode;
}
extern "C" {
    pub fn dm_setup_md_queue(md: *mut mapped_device, t: *mut dm_table) -> c_int;
}
//
// To check whether the target type is bio-based or not (request-based).
//

//
// To check whether the target type is request-based or not (bio-based).
//

//
// To check whether the target type is a hybrid (capable of being
// either request-based or bio-based).
//

//
// Zoned targets related functions.
//
extern "C" {
    pub fn dm_revalidate_zones(t: *mut dm_table, q: *mut request_queue) -> c_int;
}
extern "C" {
    pub fn dm_finalize_zone_settings(t: *mut dm_table, lim: *mut queue_limits);
}
extern "C" {
    pub fn dm_zone_endio(io: *mut dm_io, clone: *mut bio);
}

extern "C" {
    pub fn dm_is_zone_write(md: *mut mapped_device, bio: *mut bio) -> bool;
}

//
// ---------------------------------------------------------------
// A registry of target types.
// ---------------------------------------------------------------
//
extern "C" {
    pub fn dm_target_init() -> c_int;
}
extern "C" {
    pub fn dm_target_exit();
}
extern "C" {
    pub fn dm_put_target_type(tt: *mut target_type);
}
extern "C" {
    pub fn dm_split_args(argc: *mut c_int, argvp: *mut c_char, input: *mut c_char) -> c_int;
}
//
// Is this mapped_device being deleted?
//
extern "C" {
    pub fn dm_deleting_md(md: *mut mapped_device) -> c_int;
}
//
// Is this mapped_device suspended?
//
extern "C" {
    pub fn dm_suspended_md(md: *mut mapped_device) -> c_int;
}
//
// Internal suspend and resume methods.
//
extern "C" {
    pub fn dm_suspended_internally_md(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_internal_suspend_fast(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_internal_resume_fast(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_internal_suspend_noflush(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_internal_resume(md: *mut mapped_device);
}
//
// Test if the device is scheduled for deferred remove.
//
extern "C" {
    pub fn dm_test_deferred_remove_flag(md: *mut mapped_device) -> c_int;
}
//
// Try to remove devices marked for deferred removal.
//
extern "C" {
    pub fn dm_deferred_remove();
}
//
// The device-mapper can be driven through one of two interfaces;
// ioctl or filesystem, depending which patch you have applied.
//
extern "C" {
    pub fn dm_interface_init() -> c_int;
}
extern "C" {
    pub fn dm_interface_exit();
}
//
// sysfs interface
//
extern "C" {
    pub fn dm_sysfs_init(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_sysfs_exit(md: *mut mapped_device);
}
//
// The kobject helper
//
extern "C" {
    pub fn dm_kobject_release(kobj: *mut kobject);
}
//
// Targets for linear and striped mappings
//
extern "C" {
    pub fn linear_map(ti: *mut dm_target, bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn dm_linear_init() -> c_int;
}
extern "C" {
    pub fn dm_linear_exit();
}
extern "C" {
    pub fn stripe_map(ti: *mut dm_target, bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn dm_stripe_init() -> c_int;
}
extern "C" {
    pub fn dm_stripe_exit();
}
//
// mapped_device operations
//
extern "C" {
    pub fn dm_destroy(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_destroy_immediate(md: *mut mapped_device);
}
extern "C" {
    pub fn dm_open_count(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_lock_for_deletion(md: *mut mapped_device, mark_deferred: bool, only_deferred: bool) -> c_int;
}
extern "C" {
    pub fn dm_cancel_deferred_remove(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_request_based(md: *mut mapped_device) -> c_int;
}
extern "C" {
    pub fn dm_put_table_device(md: *mut mapped_device, d: *mut dm_dev);
}
extern "C" {
    pub fn dm_io_init() -> c_int;
}
extern "C" {
    pub fn dm_io_exit();
}
extern "C" {
    pub fn dm_kcopyd_init() -> c_int;
}
extern "C" {
    pub fn dm_kcopyd_exit();
}
//
// Mempool operations
//
extern "C" {
    pub fn dm_free_md_mempools(pools: *mut dm_md_mempools);
}
//
// Various helpers
//
extern "C" {
    pub fn dm_get_reserved_bio_based_ios() -> c_uint;
}
pub const DM_HASH_LOCKS_MAX: c_int = 64;
extern "C" {
    pub fn min_t(int: unsigned, _arg: num_locks, _arg: DM_HASH_LOCKS_MAX) -> return;
}

pub const DM_HASH_LOCKS_SHIFT: c_int = 6;
