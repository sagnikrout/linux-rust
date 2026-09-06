//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-block-manager.h
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
// Copyright (C) 2011 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// Block number.
//
pub type dm_block_t = u64;
extern "C" {
    pub fn dm_block_location(b: *mut dm_block) -> dm_block_t;
}
// ----------------------------------------------------------------
//
// @name should be a unique identifier for the block manager, no longer
// than 32 chars.
//
// @max_held_per_thread should be the maximum number of locks, read or
// write, that an individual thread holds at any one time.
//
extern "C" {
    pub fn dm_block_manager_destroy(bm: *mut dm_block_manager);
}
extern "C" {
    pub fn dm_block_manager_reset(bm: *mut dm_block_manager);
}
extern "C" {
    pub fn dm_bm_block_size(bm: *mut dm_block_manager) -> c_uint;
}
extern "C" {
    pub fn dm_bm_nr_blocks(bm: *mut dm_block_manager) -> dm_block_t;
}
// ----------------------------------------------------------------
//
// The validator allows the caller to verify newly-read data and modify
// the data just before writing, e.g. to calculate checksums.  It's
// important to be consistent with your use of validators.  The only time
// you can change validators is if you call dm_bm_write_lock_zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_block_validator {
    pub name: *const c_char,
    pub block_size): *mut *mut dm_block b, size_t,
//
// Return 0 if the checksum is valid or < 0 on error.
//
    pub block_size): *mut *mut dm_block b, size_t,
}

// ----------------------------------------------------------------
//
// You can have multiple concurrent readers or a single writer holding a
// block lock.
//
// dm_bm_lock() locks a block and returns through @result a pointer to
// memory that holds a copy of that block.  If you have write-locked the
// block then any changes you make to memory pointed to by @result will be
// written back to the disk sometime after dm_bm_unlock is called.
//
// The *_try_lock variants return -EWOULDBLOCK if the block isn't
// available immediately.
//
// Use dm_bm_write_lock_zero() when you know you're going to
// overwrite the block completely.  It saves a disk read.
//
extern "C" {
    pub fn dm_bm_unlock(b: *mut dm_block);
}
//
// It's a common idiom to have a superblock that should be committed last.
//
// @superblock should be write-locked on entry. It will be unlocked during
// this function.  All dirty blocks are guaranteed to be written and flushed
// before the superblock.
//
// This method always blocks.
//
extern "C" {
    pub fn dm_bm_flush(bm: *mut dm_block_manager) -> c_int;
}
//
// Request data is prefetched into the cache.
//
extern "C" {
    pub fn dm_bm_prefetch(bm: *mut dm_block_manager, b: dm_block_t);
}
//
// Switches the bm to a read only mode.  Once read-only mode
// has been entered the following functions will return -EPERM.
//
// dm_bm_write_lock
// dm_bm_write_lock_zero
// dm_bm_flush_and_unlock
//
// Additionally you should not use dm_bm_unlock_move, however no error will
// be returned if you do.
//
extern "C" {
    pub fn dm_bm_is_read_only(bm: *mut dm_block_manager) -> bool;
}
extern "C" {
    pub fn dm_bm_set_read_only(bm: *mut dm_block_manager);
}
extern "C" {
    pub fn dm_bm_set_read_write(bm: *mut dm_block_manager);
}
extern "C" {
    pub fn dm_bm_checksum(data: *const c_void, len: usize, init_xor: u32) -> u32;
}
// ----------------------------------------------------------------
