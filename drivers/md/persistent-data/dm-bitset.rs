//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-bitset.h
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
// Copyright (C) 2012 Red Hat, Inc.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
//
// This bitset type is a thin wrapper round a dm_array of 64bit words.  It
// uses a tiny, one word cache to reduce the number of array lookups and so
// increase performance.
//
// Like the dm-array that it's based on, the caller needs to keep track of
// the size of the bitset separately.  The underlying dm-array implicitly
// knows how many words it's storing and will return -ENODATA if you try
// and access an out of bounds word.  However, an out of bounds bit in the
// final word will _not_ be detected, you have been warned.
//
// Bits are indexed from zero.
// Typical use:
//
// a) Initialise a dm_disk_bitset structure with dm_disk_bitset_init().
// This describes the bitset and includes the cache.  It's not called it
// dm_bitset_info in line with other data structures because it does
// include instance data.
//
// b) Get yourself a root.  The root is the index of a block of data on the
// disk that holds a particular instance of an bitset.  You may have a
// pre existing root in your metadata that you wish to use, or you may
// want to create a brand new, empty bitset with dm_bitset_empty().
//
// Like the other data structures in this library, dm_bitset objects are
// immutable between transactions.  Update functions will return you the
// root for a _new_ array.  If you've incremented the old root, via
// dm_tm_inc(), before calling the update function you may continue to use
// it in parallel with the new root.
//
// Even read operations may trigger the cache to be flushed and as such
// return a root for a new, updated bitset.
//
// c) resize a bitset with dm_bitset_resize().
//
// d) Set a bit with dm_bitset_set_bit().
//
// e) Clear a bit with dm_bitset_clear_bit().
//
// f) Test a bit with dm_bitset_test_bit().
//
// g) Flush all updates from the cache with dm_bitset_flush().
//
// h) Destroy the bitset with dm_bitset_del().  This tells the transaction
// manager that you're no longer using this data structure so it can
// recycle it's blocks.  (dm_bitset_dec() would be a better name for it,
// but del is in keeping with dm_btree_del()).
//
// Opaque object.  Unlike dm_array_info, you should have one of these per
// bitset.  Initialise with dm_disk_bitset_init().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_disk_bitset {
    pub array_info: dm_array_info,
    pub current_index: u32,
    pub current_bits: u64,
    pub current_index_set:1: bool,
    pub dirty:1: bool,
}

//
// Sets up a dm_disk_bitset structure.  You don't need to do anything with
// this structure when you finish using it.
//
// tm - the transaction manager that should supervise this structure
// info - the structure being initialised
//
// Create an empty, zero length bitset.
//
// info - describes the bitset
// new_root - on success, points to the new root block
//
extern "C" {
    pub fn dm_bitset_empty(info: *mut dm_disk_bitset, new_root: *mut dm_block_t) -> c_int;
}
//
// Creates a new bitset populated with values provided by a callback
// function.  This is more efficient than creating an empty bitset,
// resizing, and then setting values since that process incurs a lot of
// copying.
//
// info - describes the array
// root - the root block of the array on disk
// size - the number of entries in the array
// fn - the callback
// context - passed to the callback
//
extern "C" {
    pub fn int(index: *mut *mut bit_value_fn)(uint32_t, value: *mut bool, context: *mut c_void) -> typedef;
}
//
// Resize the bitset.
//
// info - describes the bitset
// old_root - the root block of the array on disk
// old_nr_entries - the number of bits in the old bitset
// new_nr_entries - the number of bits you want in the new bitset
// default_value - the value for any new bits
// new_root - on success, points to the new root block
//
// Frees the bitset.
//
extern "C" {
    pub fn dm_bitset_del(info: *mut dm_disk_bitset, root: dm_block_t) -> c_int;
}
//
// Set a bit.
//
// info - describes the bitset
// root - the root block of the bitset
// index - the bit index
// new_root - on success, points to the new root block
//
// -ENODATA will be returned if the index is out of bounds.
//
// Clears a bit.
//
// info - describes the bitset
// root - the root block of the bitset
// index - the bit index
// new_root - on success, points to the new root block
//
// -ENODATA will be returned if the index is out of bounds.
//
// Tests a bit.
//
// info - describes the bitset
// root - the root block of the bitset
// index - the bit index
// new_root - on success, points to the new root block (cached values may have been written)
// result - the bit value you're after
//
// -ENODATA will be returned if the index is out of bounds.
//
// Flush any cached changes to disk.
//
// info - describes the bitset
// root - the root block of the bitset
// new_root - on success, points to the new root block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_bitset_cursor {
    pub info: *mut dm_disk_bitset,
    pub cursor: dm_array_cursor,
    pub entries_remaining: u32,
    pub array_index: u32,
    pub bit_index: u32,
    pub current_bits: u64,
}

//
// Make sure you've flush any dm_disk_bitset and updated the root before
// using this.
//
extern "C" {
    pub fn dm_bitset_cursor_end(c: *mut dm_bitset_cursor);
}
extern "C" {
    pub fn dm_bitset_cursor_next(c: *mut dm_bitset_cursor) -> c_int;
}
extern "C" {
    pub fn dm_bitset_cursor_skip(c: *mut dm_bitset_cursor, count: u32) -> c_int;
}
extern "C" {
    pub fn dm_bitset_cursor_get_value(c: *mut dm_bitset_cursor) -> bool;
}
// ----------------------------------------------------------------
