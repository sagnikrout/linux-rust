//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/hashmap.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Generic non-thread safe hash map implementation.
//
// Copyright (c) 2019 Facebook
//

// shuffle bits and return requested number of upper bits

// LP64 case

// generic C-string hashing function
extern "C" {
    pub fn size_t(key: *mut *mut hashmap_hash_fn)(long, ctx: *mut c_void) -> typedef;
}
extern "C" {
    pub fn bool(key1: *mut *mut hashmap_equal_fn)(long, key2: c_long, ctx: *mut c_void) -> typedef;
}
//
// Hashmap interface is polymorphic, keys and values could be either
// long-sized integers or pointers, this is achieved as follows:
// - interface functions that operate on keys and values are hidden
// behind auxiliary macros, e.g. hashmap_insert <-> hashmap__insert;
// - these auxiliary macros cast the key and value parameters as
// long or long *, so the user does not have to specify the casts explicitly;
// - for pointer parameters (e.g. old_key) the size of the pointed
// type is verified by hashmap_cast_ptr using _Static_assert;
// - when iterating using hashmap__for_each_* forms
// hasmap_entry->key should be used for integer keys and
// hasmap_entry->pkey should be used for pointer keys,
// same goes for values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashmap_entry {
    pub key: c_long,
    pub pkey: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashmap {
    pub hash_fn: hashmap_hash_fn,
    pub equal_fn: hashmap_equal_fn,
    pub ctx: *mut c_void,
    pub buckets: *mut hashmap_entry,
    pub cap: usize,
    pub cap_bits: usize,
    pub sz: usize,
}

extern "C" {
    pub fn hashmap__clear(map: *mut hashmap);
}
extern "C" {
    pub fn hashmap__free(map: *mut hashmap);
}
extern "C" {
    pub fn hashmap__size(map: *const hashmap) -> usize;
}
extern "C" {
    pub fn hashmap__capacity(map: *const hashmap) -> usize;
}
//
// Hashmap insertion strategy:
// - HASHMAP_ADD - only add key/value if key doesn't exist yet;
// - HASHMAP_SET - add key/value pair if key doesn't exist yet; otherwise,
// update value;
// - HASHMAP_UPDATE - update value, if key already exists; otherwise, do
// nothing and return -ENOENT;
// - HASHMAP_APPEND - always add key/value pair, even if key already exists.
// This turns hashmap into a multimap by allowing multiple values to be
// associated with the same key. Most useful read API for such hashmap is
// hashmap__for_each_key_entry() iteration. If hashmap__find() is still
// used, it will return last inserted key/value entry (first in a bucket
// chain).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hashmap_insert_strategy {
    HASHMAP_ADD,
    HASHMAP_SET,
    HASHMAP_UPDATE,
    HASHMAP_APPEND,
}

//
// hashmap__insert() adds key/value entry w/ various semantics, depending on
// provided strategy value. If a given key/value pair replaced already
// existing key/value pair, both old key and old value will be returned
// through old_key and old_value to allow calling code do proper memory
// management.
//

extern "C" {
    pub fn hashmap_delete(map: *mut hashmap, key: c_long, old_key: *mut c_long, old_value: *mut c_long) -> bool;
}

extern "C" {
    pub fn hashmap_find(map: *const hashmap, key: c_long, value: *mut c_long) -> bool;
}

//
// hashmap__for_each_entry - iterate over all entries in hashmap
// @map: hashmap to iterate
// @cur: struct hashmap_entry * used as a loop cursor
// @bkt: integer used as a bucket loop cursor
//

//
// hashmap__for_each_entry_safe - iterate over all entries in hashmap, safe
// against removals
// @map: hashmap to iterate
// @cur: struct hashmap_entry * used as a loop cursor
// @tmp: struct hashmap_entry * used as a temporary next cursor storage
// @bkt: integer used as a bucket loop cursor
//

//
// hashmap__for_each_key_entry - iterate over entries associated with given key
// @map: hashmap to iterate
// @cur: struct hashmap_entry * used as a loop cursor
// @key: key to iterate entries for
//

