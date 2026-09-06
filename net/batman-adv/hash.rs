//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/hash.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) B.A.T.M.A.N. contributors:
//
// Simon Wunderlich, Marek Lindner
//

//
// typedef batadv_hashdata_compare_cb - hash element comparison callback
// @node: hlist node of the element currently stored in the bucket
// @key: opaque payload to compare @node's key against
//
// Compare hash element by its keys.
//
// Return: true if both elements are considered equal, false otherwise.
//
// typedef batadv_hashdata_choose_cb - hash bucket selection callback
// @key: opaque payload whose key selects the bucket
// @size: number of buckets in the hash table
//
// Return: bucket index derived from the key in @key and the table @size.
//
extern "C" {
    pub fn u32(key: *const *const batadv_hashdata_choose_cb)(void, size: u32) -> typedef;
}
//
// typedef batadv_hashdata_free_cb - hash element free callback
// @node: hlist node of the element being removed
// @arg: opaque caller-supplied argument forwarded from the caller
//
// Release a previously inserted hash element.
//
extern "C" {
    pub fn void(node: *mut *mut batadv_hashdata_free_cb)(struct hlist_node, arg: *mut c_void) -> typedef;
}
//
// struct batadv_hashtable - Wrapper of simple hlist based hashtable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct batadv_hashtable {
// @table: the hashtable itself with the buckets
    pub table: *mut hlist_head,
// @list_locks: spinlock for each hash list entry
    pub list_locks: *mut spinlock_t,
// @size: size of hashtable
    pub size: u32,
// @generation: current (generation) sequence number
    pub generation: core::sync::atomic::AtomicI32,
}

// allocates and clears the hash
// set class key for all locks
// free only the hashtable and the hash itself.
extern "C" {
    pub fn batadv_hash_destroy(hash: *mut batadv_hashtable);
}
//
// batadv_hash_add() - adds data to the hashtable
// @hash: storage hash table
// @compare: callback to determine if 2 hash elements are identical
// @choose: callback calculating the hash index
// @data: data passed to the aforementioned callbacks as argument
// @data_node: to be added element
//
// Return: 0 on success, 1 if the element already is in the hash
// and -1 on error.
//
// no duplicate found in list, add new element
//
// batadv_hash_remove() - Removes data from hash, if found
// @hash: hash table
// @compare: callback to determine if 2 hash elements are identical
// @choose: callback calculating the hash index
// @data: data passed to the aforementioned callbacks as argument
//
// data could be the structure you use with just the key filled, we just need
// the key for comparing.
//
// Return: returns pointer to data on success, so you can remove the used
// structure yourself, or NULL on error
//
