//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/persistent-data/dm-btree.h
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
// Annotations used to check on-disk metadata is handled as little-endian.
//

// ----------------------------------------------------------------
//
// Manipulates hierarchical B+ trees with 64-bit keys and arbitrary-sized
// values.
//
// Information about the values stored within the btree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_btree_value_type {
    pub context: *mut c_void,
//
// The size in bytes of each value.
//
    pub size: u32,
//
// Any of these methods can be safely set to NULL if you do not
// need the corresponding feature.
//
// The btree is making a duplicate of a run of values, for instance
// because previously-shared btree nodes have now diverged.
// @value argument is the new copy that the copy function may modify.
// (Probably it just wants to increment a reference count
// somewhere.) This method is _not_ called for insertion of a new
// value: It is assumed the ref count is already 1.
//
    pub count): *const *const *const *const void (inc)(void context, void value, unsigned int,
//
// These values are being deleted.  The btree takes care of freeing
// the memory pointed to by @value.  Often the del function just
// needs to decrement a reference counts somewhere.
//
    pub count): *const *const *const *const void (dec)(void context, void value, unsigned int,
//
// A test for equality between two values.  When a value is
// overwritten with a new one, the old one has the dec method
// called _unless_ the new and old value are deemed equal.
//
    pub value2): *const *const *const *const int (equal)(void context, void value1, void,
}

//
// The shape and contents of a btree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_btree_info {
    pub tm: *mut dm_transaction_manager,
//
// Number of nested btrees. (Not the depth of a single tree.)
//
    pub levels: c_uint,
    pub value_type: dm_btree_value_type,
}

//
// Set up an empty tree.  O(1).
//
extern "C" {
    pub fn dm_btree_empty(info: *mut dm_btree_info, root: *mut dm_block_t) -> c_int;
}
//
// Delete a tree.  O(n) - this is the slow one!  It can also block, so
// please don't call it on an IO path.
//
extern "C" {
    pub fn dm_btree_del(info: *mut dm_btree_info, root: dm_block_t) -> c_int;
}
//
// All the lookup functions return -ENODATA if the key cannot be found.
//
// Tries to find a key that matches exactly.  O(ln(n))
//
// Tries to find the first key where the bottom level key is >= to that
// given.  Useful for skipping empty sections of the btree.
//
// Insertion (or overwrite an existing value).  O(ln(n))
//
// A variant of insert that indicates whether it actually inserted or just
// overwrote.  Useful if you're keeping track of the number of entries in a
// tree.
//
// Remove a key if present.  This doesn't remove empty sub trees.  Normally
// subtrees represent a separate entity, like a snapshot map, so this is
// correct behaviour.  O(ln(n)).
//
// Removes a _contiguous_ run of values starting from 'keys' and not
// reaching keys2 (where keys2 is keys with the final key replaced with
// 'end_key').  'end_key' is the one-past-the-end value.  'keys' may be
// altered.
//
// Returns < 0 on failure.  Otherwise the number of key entries that have
// been filled out.  Remember trees can have zero entries, and as such have
// no lowest key.
//
// Returns < 0 on failure.  Otherwise the number of key entries that have
// been filled out.  Remember trees can have zero entries, and as such have
// no highest key.
//
// Iterate through the a btree, calling fn() on each entry.
// It only works for single level trees and is internally recursive, so
// monitor stack usage carefully.
//
// ----------------------------------------------------------------
//
// Cursor API.  This does not follow the rolling lock convention.  Since we
// know the order that values are required we can issue prefetches to speed
// up iteration.  Use on a single level btree only.
//
pub const DM_BTREE_CURSOR_MAX_DEPTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_node {
    pub b: *mut dm_block,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_btree_cursor {
    pub info: *mut dm_btree_info,
    pub root: dm_block_t,
    pub prefetch_leaves: bool,
    pub depth: c_uint,
    pub nodes: [cursor_node; DM_BTREE_CURSOR_MAX_DEPTH],
}

//
// Creates a fresh cursor.  If prefetch_leaves is set then it is assumed
// the btree contains block indexes that will be prefetched.  The cursor is
// quite large, so you probably don't want to put it on the stack.
//
extern "C" {
    pub fn dm_btree_cursor_end(c: *mut dm_btree_cursor);
}
extern "C" {
    pub fn dm_btree_cursor_next(c: *mut dm_btree_cursor) -> c_int;
}
extern "C" {
    pub fn dm_btree_cursor_skip(c: *mut dm_btree_cursor, count: u32) -> c_int;
}
extern "C" {
    pub fn dm_btree_cursor_get_value(c: *mut dm_btree_cursor, key: *mut u64, value_le: *mut c_void) -> c_int;
}
