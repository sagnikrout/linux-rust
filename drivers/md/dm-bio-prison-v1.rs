//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-bio-prison-v1.h
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
pub struct dm_cell_key {
    pub virtual: c_int,
    pub dev: dm_thin_id,
    pub block_end: dm_block_t block_begin,,
}

//
// The range of a key (block_end - block_begin) must not
// exceed BIO_PRISON_MAX_RANGE.  Also the range must not
// cross a similarly sized boundary.
//
// Must be a power of 2.
//
pub const BIO_PRISON_MAX_RANGE: c_int = 1024;
pub const BIO_PRISON_MAX_RANGE_SHIFT: c_int = 10;
//
// Treat this as opaque, only in header so callers can manage allocation
// themselves.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_bio_prison_cell {
    pub /: *mut *mut list_head user_list; / for client use,
    pub node: rb_node,
    pub key: dm_cell_key,
    pub holder: *mut bio,
    pub bios: bio_list,
}

extern "C" {
    pub fn dm_bio_prison_destroy(prison: *mut dm_bio_prison);
}
//
// These two functions just wrap a mempool.  This is a transitory step:
// Eventually all bio prison clients should manage their own cell memory.
//
// Like mempool_alloc(), dm_bio_prison_alloc_cell() can only fail if called
// in interrupt context or passed GFP_NOWAIT.
//
// Returns false if key is beyond BIO_PRISON_MAX_RANGE or spans a boundary.
//
extern "C" {
    pub fn dm_cell_key_has_valid_range(key: *mut dm_cell_key) -> bool;
}
//
// An atomic op that combines retrieving or creating a cell, and adding a
// bio to it.
//
// Returns 1 if the cell was already held, 0 if @inmate is the new holder.
//
// Visits the cell and then releases.  Guarantees no new inmates are
// inserted between the visit and release.
//
// ----------------------------------------------------------------
//
// We use the deferred set to keep track of pending reads to shared blocks.
// We do this to ensure the new mapping caused by a write isn't performed
// until these prior reads have completed.  Otherwise the insertion of the
// new mapping could free the old block that the read bios are mapped to.
//
extern "C" {
    pub fn dm_deferred_set_destroy(ds: *mut dm_deferred_set);
}
extern "C" {
    pub fn dm_deferred_entry_dec(entry: *mut dm_deferred_entry, head: *mut list_head);
}
extern "C" {
    pub fn dm_deferred_set_add_work(ds: *mut dm_deferred_set, work: *mut list_head) -> c_int;
}
// ----------------------------------------------------------------
