//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-bio-prison-v2.h
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
// Copyright (C) 2011-2017 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
extern "C" {
    pub fn dm_bio_prison_init_v2() -> c_int;
}
extern "C" {
    pub fn dm_bio_prison_exit_v2();
}
//
// Sometimes we can't deal with a bio straight away.  We put them in prison
// where they can't cause any mischief.  Bios are put in a cell identified
// by a key, multiple bios can be in the same cell.  When the cell is
// subsequently unlocked the bios become available.
//
// Keys define a range of blocks within either a virtual or physical
// device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_cell_key_v2 {
    pub virtual: c_int,
    pub dev: dm_thin_id,
    pub block_end: dm_block_t block_begin,,
}

//
// Treat this as opaque, only in header so callers can manage allocation
// themselves.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_bio_prison_cell_v2 {
// FIXME: pack these
    pub exclusive_lock: bool,
    pub exclusive_level: c_uint,
    pub shared_count: c_uint,
    pub quiesce_continuation: *mut work_struct,
    pub node: rb_node,
    pub key: dm_cell_key_v2,
    pub bios: bio_list,
}

extern "C" {
    pub fn dm_bio_prison_destroy_v2(prison: *mut dm_bio_prison_v2);
}
//
// These two functions just wrap a mempool.  This is a transitory step:
// Eventually all bio prison clients should manage their own cell memory.
//
// Like mempool_alloc(), dm_bio_prison_alloc_cell_v2() can only fail if called
// in interrupt context or passed GFP_NOWAIT.
//
// Shared locks have a bio associated with them.
//
// If the lock is granted the caller can continue to use the bio, and must
// call dm_cell_put_v2() to drop the reference count when finished using it.
//
// If the lock cannot be granted then the bio will be tracked within the
// cell, and later given to the holder of the exclusive lock.
//
// See dm_cell_lock_v2() for discussion of the lock_level parameter.
//
// Compare *cell_result with cell_prealloc to see if the prealloc was used.
// If cell_prealloc was used then inmate wasn't added to it.
//
// Returns true if the lock is granted.
//
// Decrement the shared reference count for the lock.  Returns true if
// returning ownership of the cell (ie. you should free it).
//
// Locks a cell.  No associated bio.  Exclusive locks get priority.  These
// locks constrain whether the io locks are granted according to level.
//
// Shared locks will still be granted if the lock_level is > (not = to) the
// exclusive lock level.
//
// If an _exclusive_ lock is already held then -EBUSY is returned.
//
// Return values:
// < 0 - error
// 0   - locked; no quiescing needed
// 1   - locked; quiescing needed
//
// Promotes an _exclusive_ lock to a higher lock level.
//
// Return values:
// < 0 - error
// 0   - promoted; no quiescing needed
// 1   - promoted; quiescing needed
//
// Adds any held bios to the bio list.
//
// There may be shared locks still held at this point even if you quiesced
// (ie. different lock levels).
//
// Returns true if returning ownership of the cell (ie. you should free
// it).
//
// ----------------------------------------------------------------
