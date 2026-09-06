//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lru_cache.h
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

//
// this defines an element in a tracked set
// .collision is for hash table lookup.
// When we process a new IO request, we know its sector, thus can deduce the
// region number (label) easily.  To do the label -> object lookup without a
// full list walk, we use a simple hash table.
//
// .list is on one of three lists:
// in_use: currently in use (refcnt > 0, lc_number != LC_FREE)
// lru: unused but ready to be reused or recycled
// (lc_refcnt == 0, lc_number != LC_FREE),
// free: unused but ready to be recycled
// (lc_refcnt == 0, lc_number == LC_FREE),
//
// an element is said to be "in the active set",
// if either on "in_use" or "lru", i.e. lc_number != LC_FREE.
//
// DRBD currently (May 2009) only uses 61 elements on the resync lru_cache
// (total memory usage 2 pages), and up to 3833 elements on the act_log
// lru_cache, totalling ~215 kB for 64bit architecture, ~53 pages.
//
// We usually do not actually free these objects again, but only "recycle"
// them, as the change "index: -old_label, +LC_FREE" would need a transaction
// as well.  Which also means that using a kmem_cache to allocate the objects
// from wastes some resources.
// But it avoids high order page allocations in kmalloc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lc_element {
    pub collision: hlist_node,
    pub /: *mut *mut list_head list; / LRU list or free list,
    pub refcnt: unsigned,
// back "pointer" into lc_cache->element[index],
// for paranoia, and for "lc_element_to_index"
    pub lc_index: unsigned,
// if we want to track a larger set of objects,
// it needs to become an architecture independent u64
    pub lc_number: unsigned,
// special label when on free list

// for pending changes
    pub lc_new_number: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lru_cache {
// the least recently used item is kept at lru->prev
    pub lru: list_head,
    pub free: list_head,
    pub in_use: list_head,
    pub to_be_changed: list_head,
// the pre-created kmem cache to allocate the objects from
    pub lc_cache: *mut kmem_cache,
// size of tracked objects, used to memset(,0,) them in lc_reset
    pub element_size: usize,
// offset of struct lc_element member in the tracked object
    pub element_off: usize,
// number of elements (indices)
    pub nr_elements: c_uint,
// Arbitrary limit on maximum tracked objects. Practical limit is much
// lower due to allocation failures, probably. For typical use cases,
// nr_elements should be a few thousand at most.
// This also limits the maximum value of lc_element.lc_index, allowing the
// 8 high bits of .lc_index to be overloaded with flags in the future.

// allow to accumulate a few (index:label) changes,
// but no more than max_pending_changes
    pub max_pending_changes: c_uint,
// number of elements currently on to_be_changed list
    pub pending_changes: c_uint,
// statistics
    pub /: *mut *mut unsigned used; / number of elements currently on in_use list,
    pub changed: unsigned long hits, misses, starving, locked,,
// see below: flag-bits for lru_cache
    pub flags: c_ulong,
    pub name: *const c_char,
// nr_elements there
    pub lc_slot: *mut hlist_head,
    pub lc_element: *mut lc_element,
}

// flag-bits for lru_cache
// debugging aid, to catch concurrent access early.
// user needs to guarantee exclusive access by proper locking!
// annotate that the set is "dirty", possibly accumulating further
// changes, until a transaction is finally triggered
// Locked, no further changes allowed.
// Also used to serialize changing transactions.
// if we need to change the set, but currently there is no free nor
// unused element available, we are "starving", and must not give out
// further references, to guarantee that eventually some refcnt will
// drop to zero and we will be able to make progress again, changing
// the set, writing the transaction.
// if the statistics say we are frequently starving,
// nr_elements is too small.

extern "C" {
    pub fn lc_reset(lc: *mut lru_cache);
}
extern "C" {
    pub fn lc_destroy(lc: *mut lru_cache);
}
extern "C" {
    pub fn lc_del(lc: *mut lru_cache, element: *mut lc_element);
}
extern "C" {
    pub fn lc_put(lc: *mut lru_cache, e: *mut lc_element) -> c_uint;
}
extern "C" {
    pub fn lc_committed(lc: *mut lru_cache);
}
extern "C" {
    pub fn lc_seq_printf_stats(seq: *mut seq_file, lc: *mut lru_cache);
}
//
// lc_try_lock_for_transaction - can be used to stop lc_get() from changing the tracked set
// @lc: the lru cache to operate on
//
// Allows (expects) the set to be "dirty".  Note that the reference counts and
// order on the active and lru lists may still change.  Used to serialize
// changing transactions.  Returns true if we acquired the lock.
//
// lc_try_lock - variant to stop lc_get() from changing the tracked set
// @lc: the lru cache to operate on
//
// Note that the reference counts and order on the active and lru lists may
// still change.  Only works on a "clean" set.  Returns true if we acquired the
// lock, which means there are no pending changes, and any further attempt to
// change the set will not succeed until the next lc_unlock().
//
extern "C" {
    pub fn lc_try_lock(lc: *mut lru_cache) -> c_int;
}
//
// lc_unlock - unlock @lc, allow lc_get() to change the set again
// @lc: the lru cache to operate on
//
extern "C" {
    pub fn lc_is_used(lc: *mut lru_cache, enr: c_uint) -> bool;
}

