//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/btree.h
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

//
// DOC: B+Tree basics
//
// A B+Tree is a data structure for looking up arbitrary (currently allowing
// unsigned long, u32, u64 and 2 * u64) keys into pointers. The data structure
// is described at https://en.wikipedia.org/wiki/B-tree, we currently do not
// use binary search to find the key on lookups.
//
// Each B+Tree consists of a head, that contains bookkeeping information and
// a variable number (starting with zero) nodes. Each node contains the keys
// and pointers to sub-nodes, or, for leaf nodes, the keys and values for the
// tree entries.
//
// Each node in this implementation has the following layout:
// [key1, key2, ..., keyN] [val1, val2, ..., valN]
//
// Each key here is an array of unsigned longs, geo->no_longs in total. The
// number of keys and values (N) is geo->no_pairs.
//
// struct btree_head - btree head
//
// @node: the first node in the tree
// @mempool: mempool used for node allocations
// @height: current of the tree
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_head {
    pub node: *mut c_ulong,
    pub mempool: *mut mempool_t,
    pub height: c_int,
}

// btree geometry
//
// btree_alloc - allocate function for the mempool
// @gfp_mask: gfp mask for the allocation
// @pool_data: unused
//
// btree_free - free function for the mempool
// @element: the element to free
// @pool_data: unused
//
extern "C" {
    pub fn btree_free(element: *mut c_void, pool_data: *mut c_void);
}
//
// btree_init_mempool - initialise a btree with given mempool
//
// @head: the btree head to initialise
// @mempool: the mempool to use
//
// When this function is used, there is no need to destroy
// the mempool.
//
extern "C" {
    pub fn btree_init_mempool(head: *mut btree_head, mempool: *mut mempool_t);
}
//
// btree_init - initialise a btree
//
// @head: the btree head to initialise
//
// This function allocates the memory pool that the
// btree needs. Returns zero or a negative error code
// (-%ENOMEM) when memory allocation fails.
//
extern "C" {
    pub fn btree_init(head: *mut btree_head) -> int __must_check;
}
//
// btree_destroy - destroy mempool
//
// @head: the btree head to destroy
//
// This function destroys the internal memory pool, use only
// when using btree_init(), not with btree_init_mempool().
//
extern "C" {
    pub fn btree_destroy(head: *mut btree_head);
}
//
// btree_lookup - look up a key in the btree
//
// @head: the btree to look in
// @geo: the btree geometry
// @key: the key to look up
//
// This function returns the value for the given key, or %NULL.
//
// btree_insert - insert an entry into the btree
//
// @head: the btree to add to
// @geo: the btree geometry
// @key: the key to add (must not already be present)
// @val: the value to add (must not be %NULL)
// @gfp: allocation flags for node allocations
//
// This function returns 0 if the item could be added, or an
// error code if it failed (may fail due to memory pressure).
//
// btree_update - update an entry in the btree
//
// @head: the btree to update
// @geo: the btree geometry
// @key: the key to update
// @val: the value to change it to (must not be %NULL)
//
// This function returns 0 if the update was successful, or
// -%ENOENT if the key could not be found.
//
// btree_remove - remove an entry from the btree
//
// @head: the btree to update
// @geo: the btree geometry
// @key: the key to remove
//
// This function returns the removed entry, or %NULL if the key
// could not be found.
//
// btree_merge - merge two btrees
//
// @target: the tree that gets all the entries
// @victim: the tree that gets merged into @target
// @geo: the btree geometry
// @gfp: allocation flags
//
// The two trees @target and @victim may not contain the same keys,
// that is a bug and triggers a BUG(). This function returns zero
// if the trees were merged successfully, and may return a failure
// when memory allocation fails, in which case both trees might have
// been partially merged, i.e. some entries have been moved from
// @victim to @target.
//
// btree_last - get last entry in btree
//
// @head: btree head
// @geo: btree geometry
// @key: last key
//
// Returns the last entry in the btree, and sets @key to the key
// of that entry; returns NULL if the tree is empty, in that case
// key is not changed.
//
// btree_get_prev - get previous entry
//
// @head: btree head
// @geo: btree geometry
// @key: pointer to key
//
// The function returns the next item right before the value pointed to by
// @key, and updates @key with its key, or returns %NULL when there is no
// entry with a key smaller than the given key.
//
// internal use, use btree_visitor{l,32,64,128}
// internal use, use btree_grim_visitor{l,32,64,128}

pub const BTREE_TYPE_SUFFIX: c_int = 32;
pub const BTREE_TYPE_BITS: c_int = 32;

pub const BTREE_TYPE_SUFFIX: c_int = 64;
pub const BTREE_TYPE_BITS: c_int = 64;

