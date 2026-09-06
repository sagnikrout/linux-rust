//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/vio.h
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
pub struct pooled_vio {
// The underlying vio
    pub vio: vio,
// The list entry for chaining pooled vios together
    pub list_entry: list_head,
// The context set by the pool
    pub context: *mut c_void,
// The list entry used by the pool
    pub pool_entry: list_head,
// The pool this vio is allocated from
    pub pool: *mut vio_pool,
}

//
// as_vio() - Convert a generic vdo_completion to a vio.
// @completion: The completion to convert.
//
// Return: The completion as a vio.
//
extern "C" {
    pub fn container_of(_arg: completion, vio: struct, _arg: completion) -> return;
}
//
// get_vio_bio_zone_thread_id() - Get the thread id of the bio zone in which a vio should submit
// its I/O.
// @vio: The vio.
//
// Return: The id of the bio zone thread the vio should use.
//
extern "C" {
    pub fn pbn_from_vio_bio(bio: *mut bio) -> physical_block_number_t __must_check;
}
//
// assert_vio_in_bio_zone() - Check that a vio is running on the correct thread for its bio zone.
// @vio: The vio to check.
//
extern "C" {
    pub fn vdo_create_bio(bio_ptr: *mut bio) -> c_int;
}
extern "C" {
    pub fn vdo_free_bio(bio: *mut bio);
}
extern "C" {
    pub fn free_vio_components(vio: *mut vio);
}
extern "C" {
    pub fn free_vio(vio: *mut vio);
}
//
// initialize_vio() - Initialize a vio.
// @vio: The vio to initialize.
// @bio: The bio this vio should use for its I/O.
// @block_count: The size of this vio in vdo blocks.
// @vio_type: The vio type.
// @priority: The relative priority of the vio.
// @vdo: The vdo for this vio.
//
// data_vio's may not span multiple blocks
//
// is_data_vio() - Check whether a vio is servicing an external data request.
// @vio: The vio to check.
//
// get_metadata_priority() - Convert a vio's priority to a work item priority.
// @vio: The vio.
//
// Return: The priority with which to submit the vio's bio.
//
// continue_vio() - Enqueue a vio to run its next callback.
// @vio: The vio to continue.
// @result: The result of the current operation.
//
extern "C" {
    pub fn vdo_count_bios(bio_stats: *mut atomic_bio_stats, bio: *mut bio);
}
extern "C" {
    pub fn vdo_count_completed_bios(bio: *mut bio);
}
//
// continue_vio_after_io() - Continue a vio now that its I/O has returned.
// @vio: The vio to continue.
// @callback: The next operation for this vio.
// @thread: Which thread to run the next operation on.
//
extern "C" {
    pub fn vio_record_metadata_io_error(vio: *mut vio);
}
// A vio_pool is a collection of preallocated vios used to write arbitrary metadata blocks.
extern "C" {
    pub fn container_of(_arg: vio, pooled_vio: struct, _arg: vio) -> return;
}
extern "C" {
    pub fn free_vio_pool(pool: *mut vio_pool);
}
extern "C" {
    pub fn is_vio_pool_busy(pool: *mut vio_pool) -> bool __must_check;
}
extern "C" {
    pub fn acquire_vio_from_pool(pool: *mut vio_pool, waiter: *mut vdo_waiter);
}
extern "C" {
    pub fn return_vio_to_pool(vio: *mut pooled_vio);
}
