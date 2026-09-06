//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-space-map.h
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

extern "C" {
    pub fn void(context: *mut *mut dm_sm_threshold_fn)(void) -> typedef;
}
//
// struct dm_space_map keeps a record of how many times each block in a device
// is referenced.  It needs to be fixed on disk as part of the transaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_space_map {
    pub sm): *mut *mut void (destroy)(struct dm_space_map,
//
// You must commit before allocating the newly added space.
//
    pub extra_blocks): *mut *mut *mut int (extend)(struct dm_space_map sm, dm_block_t,
//
// Extensions do not appear in this count until after commit has
// been called.
//
    pub count): *mut *mut *mut int (get_nr_blocks)(struct dm_space_map sm, dm_block_t,
//
// Space maps must never allocate a block from the previous
// transaction, in case we need to rollback.  This complicates the
// semantics of get_nr_free(), it should return the number of blocks
// that are available for allocation _now_.  For instance you may
// have blocks with a zero reference count that will not be
// available for allocation until after the next commit.
//
    pub count): *mut *mut *mut int (get_nr_free)(struct dm_space_map sm, dm_block_t,
    pub result): *mut *mut *mut int (get_count)(struct dm_space_map sm, dm_block_t b, uint32_t,
    pub result): *mut c_int,
    pub count): *mut *mut *mut int (set_count)(struct dm_space_map sm, dm_block_t b, uint32_t,
    pub sm): *mut *mut int (commit)(struct dm_space_map,
    pub e): *mut *mut *mut int (inc_blocks)(struct dm_space_map sm, dm_block_t b, dm_block_t,
    pub e): *mut *mut *mut int (dec_blocks)(struct dm_space_map sm, dm_block_t b, dm_block_t,
//
// new_block will increment the returned block.
//
    pub b): *mut *mut *mut int (new_block)(struct dm_space_map sm, dm_block_t,
//
// The root contains all the information needed to fix the space map.
// Generally this info is small, so squirrel it away in a disk block
// along with other info.
//
    pub result): *mut *mut *mut int (root_size)(struct dm_space_map sm, size_t,
    pub len): *mut *mut *mut *mut int (copy_root)(struct dm_space_map sm, void copy_to_here_le, size_t,
//
// You can register one threshold callback which is edge-triggered
// when the free space in the space map drops below the threshold.
//
    pub context): *mut c_void,
}

// ----------------------------------------------------------------
extern "C" {
    pub fn dm_sm_inc_blocks(_arg: sm, _arg: b, 1: b +) -> return;
}
extern "C" {
    pub fn dm_sm_dec_blocks(_arg: sm, _arg: b, 1: b +) -> return;
}
