//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/radix-tree.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2001 Momchil Velikov
// Portions Copyright (C) 2001 Christoph Hellwig
// Copyright (C) 2006 Nick Piggin
// Copyright (C) 2012 Konstantin Khlebnikov
//

// Keep unconverted code working

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radix_tree_preload {
    pub lock: local_lock_t,
    pub nr: unsigned,
// nodes->parent points to next preallocated node
    pub nodes: *mut radix_tree_node,
}

//
// The bottom two bits of the slot determine how the remaining bits in the
// slot are interpreted:
//
// 00 - data pointer
// 10 - internal entry
// x1 - value entry
//
// The internal entry may be a pointer to the next level in the tree, a
// sibling entry, or an indicator that the entry in this slot has been moved
// to another location in the tree and the lookup should be restarted.  While
// NULL fits the 'data pointer' pattern, it means that there is no entry in
// the tree for this index (no matter what level of the tree it is found at).
// This means that storing a NULL entry in the tree is the same as deleting
// the entry from the tree.
//

// radix-tree API starts here

// The IDR tag is stored in the low bits of xa_flags

// The top bits of xa_flags are used to store the root tags

//
// struct radix_tree_iter - radix tree iterator state
//
// @index:	index of current slot
// @next_index:	one beyond the last index for this chunk
// @tags:	bit-mask for tag-iterating
// @node:	node that contains current slot
//
// This radix tree iterator works in terms of "chunks" of slots.  A chunk is a
// subinterval of slots contained within one radix tree leaf node.  It is
// described by a pointer to its first slot and a struct radix_tree_iter
// which holds the chunk's position in the tree and its size.  For tagged
// iteration radix_tree_iter also holds the slots' bit-mask for one chosen
// radix tree tag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radix_tree_iter {
    pub index: c_ulong,
    pub next_index: c_ulong,
    pub tags: c_ulong,
    pub node: *mut radix_tree_node,
}

//
// DOC: Radix-tree synchronization
//
// The radix-tree API requires that users provide all synchronisation (with
// specific exceptions, noted below).
//
// Synchronization of access to the data items being stored in the tree, and
// management of their lifetimes must be completely managed by API users.
//
// For API usage, in general,
// - any function _modifying_ the tree or tags (inserting or deleting
// items, setting or clearing tags) must exclude other modifications, and
// exclude any functions reading the tree.
// - any function _reading_ the tree or tags (looking up items or tags,
// gang lookups) must exclude modifications to the tree, but may occur
// concurrently with other readers.
//
// The notable exceptions to this rule are the following functions:
// __radix_tree_lookup
// radix_tree_lookup
// radix_tree_lookup_slot
// radix_tree_tag_get
// radix_tree_gang_lookup
// radix_tree_gang_lookup_tag
// radix_tree_gang_lookup_tag_slot
// radix_tree_tagged
//
// The first 7 functions are able to be called locklessly, using RCU. The
// caller must ensure calls to these functions are made within rcu_read_lock()
// regions. Other readers (lock-free or otherwise) and modifications may be
// running concurrently.
//
// It is still required that the caller manage the synchronization and lifetimes
// of the items. So if RCU lock-free lookups are used, typically this would mean
// that the items have their own locks, or are amenable to lock-free access; and
// that the items are freed by RCU (or only freed after having been deleted from
// the radix tree *and* a synchronize_rcu() grace period).
//
// (Note, rcu_assign_pointer and rcu_dereference are not needed to control
// access to data items when inserting into or looking up from the radix tree)
//
// Note that the value returned by radix_tree_tag_get() may not be relied upon
// if only the RCU read lock is held.  Functions to set/clear tags and to
// delete nodes running concurrently with it may affect its result such that
// two consecutive reads in the same locked section may return different
// values.  If reliability is required, modification functions must also be
// excluded from concurrency.
//
// radix_tree_tagged is able to be called without locking or RCU.
//
// radix_tree_deref_slot - dereference a slot
// @slot: slot pointer, returned by radix_tree_lookup_slot
//
// For use with radix_tree_lookup_slot().  Caller must hold tree at least read
// locked across slot lookup and dereference. Not required if write lock is
// held (ie. items cannot be concurrently inserted).
//
// radix_tree_deref_retry must be used to confirm validity of the pointer if
// only the read lock is held.
//
// Return: entry stored in that slot.
//
extern "C" {
    pub fn rcu_dereference(_arg: *mut slot) -> return;
}
//
// radix_tree_deref_slot_protected - dereference a slot with tree lock held
// @slot: slot pointer, returned by radix_tree_lookup_slot
// @treelock: caller must hold this spinlock
//
// Similar to radix_tree_deref_slot.  The caller does not hold the RCU read
// lock but it must hold the tree lock to prevent parallel updates.
//
// Return: entry stored in that slot.
//
extern "C" {
    pub fn rcu_dereference_protected(_arg: *mut slot, _arg: lockdep_is_held(treelock)) -> return;
}
//
// radix_tree_deref_retry	- check radix_tree_deref_slot
// @arg:	pointer returned by radix_tree_deref_slot
// Returns:	0 if retry is not required, otherwise retry is required
//
// radix_tree_deref_retry must be used with radix_tree_deref_slot.
//
extern "C" {
    pub fn unlikely(_arg: radix_tree_is_internal_node(arg)) -> return;
}
//
// radix_tree_exception	- radix_tree_deref_slot returned either exception?
// @arg:	value returned by radix_tree_deref_slot
// Returns:	0 if well-aligned pointer, non-0 if either kind of exception.
//
extern "C" {
    pub fn unlikely(RADIX_TREE_ENTRY_MASK: (unsigned long)arg &) -> return;
}
extern "C" {
    pub fn radix_tree_preload(gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn radix_tree_maybe_preload(gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn radix_tree_init();
}
extern "C" {
    pub fn radix_tree_tagged(: *const radix_tree_root, tag: c_uint) -> c_int;
}
//
// radix_tree_iter_init - initialize radix tree iterator
//
// @iter:	pointer to iterator state
// @start:	iteration starting index
// Returns:	NULL
//
// Leave iter->tags uninitialized. radix_tree_next_chunk() will fill it
// in the case of a successful tagged chunk lookup.  If the lookup was
// unsuccessful or non-tagged then nobody cares about ->tags.
//
// Set index to zero to bypass next_index overflow protection.
// See the comment in radix_tree_next_chunk() for details.
//
// radix_tree_next_chunk - find next chunk of slots for iteration
//
// @root:	radix tree root
// @iter:	iterator state
// @flags:	RADIX_TREE_ITER_* flags and tag index
// Returns:	pointer to chunk first slot, or NULL if there no more left
//
// This function looks up the next chunk in the radix tree starting from
// @iter->next_index.  It returns a pointer to the chunk's first slot.
// Also it fills @iter with data about chunk: position in the tree (index),
// its end (next_index), and constructs a bit mask for tagged iterating (tags).
//
// radix_tree_iter_lookup - look up an index in the radix tree
// @root: radix tree root
// @iter: iterator state
// @index: key to look up
//
// If @index is present in the radix tree, this function returns the slot
// containing it and updates @iter to describe the entry.  If @index is not
// present, it returns NULL.
//
extern "C" {
    pub fn radix_tree_next_chunk(_arg: root, _arg: iter, _arg: RADIX_TREE_ITER_CONTIG) -> return;
}
//
// radix_tree_iter_retry - retry this chunk of the iteration
// @iter:	iterator state
//
// If we iterate over a tree protected only by the RCU lock, a race
// against deletion or creation may result in seeing a slot for which
// radix_tree_deref_retry() returns true.  If so, call this function
// and continue the iteration.
//
// radix_tree_iter_resume - resume iterating when the chunk may be invalid
// @slot: pointer to current slot
// @iter: iterator state
// Returns: New slot pointer
//
// If the iterator needs to release then reacquire a lock, the chunk may
// have been invalidated by an insertion or deletion.  Call this function
// before releasing the lock to continue the iteration from the next index.
//
// radix_tree_chunk_size - get current chunk size
//
// @iter:	pointer to radix tree iterator
// Returns:	current chunk size
//
// radix_tree_next_slot - find next slot in chunk
//
// @slot:	pointer to current slot
// @iter:	pointer to iterator state
// @flags:	RADIX_TREE_ITER_*, should be constant
// Returns:	pointer to next slot, or NULL if there no more left
//
// This function updates @iter->index in the case of a successful lookup.
// For tagged lookup it also eats @iter->tags.
//
// There are several cases where 'slot' can be passed in as NULL to this
// function.  These cases result from the use of radix_tree_iter_resume() or
// radix_tree_iter_retry().  In these cases we don't end up dereferencing
// 'slot' because either:
// a) we are doing tagged iteration and iter->tags has been set to 0, or
// b) we are doing non-tagged iteration, and iter->index and iter->next_index
// have been set up so that radix_tree_chunk_size() returns 1 or 0.
//
// forbid switching to the next chunk
//
// radix_tree_for_each_slot - iterate over non-empty slots
//
// @slot:	the void** variable for pointer to slot
// @root:	the struct radix_tree_root pointer
// @iter:	the struct radix_tree_iter pointer
// @start:	iteration starting index
//
// @slot points to radix tree slot, @iter->index contains its index.
//

//
// radix_tree_for_each_tagged - iterate over tagged slots
//
// @slot:	the void** variable for pointer to slot
// @root:	the struct radix_tree_root pointer
// @iter:	the struct radix_tree_iter pointer
// @start:	iteration starting index
// @tag:	tag index
//
// @slot points to radix tree slot, @iter->index contains its index.
//

