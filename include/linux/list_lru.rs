//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/list_lru.h
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
// Copyright (c) 2013 Red Hat, Inc. and Parallels Inc. All rights reserved.
// Authors: David Chinner and Glauber Costa
//
// Generic LRU infrastructure
//

// list_lru_walk_cb has to always return one of those
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lru_status {
    LRU_REMOVED,		/* item removed from list */
    LRU_REMOVED_RETRY,	/* item removed, but lock has been
    dropped and reacquired */
    LRU_ROTATE,		/* item referenced, give another pass */
    LRU_SKIP,		/* item cannot be locked, skip */
    LRU_RETRY,		/* item not freeable. May drop the lock
    internally, but has to return locked. */
    LRU_STOP,		/* stop lru list walking. May drop the lock
    internally, but has to return locked. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_lru_one {
    pub list: list_head,
// may become negative during memcg reparenting
    pub nr_items: c_long,
// protects all fields above
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_lru_memcg {
    pub rcu: rcu_head,
// array of per cgroup per node lists, indexed by node id
    pub node: [list_lru_one; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_lru_node {
// global list, used for the root cgroup in cgroup aware lrus
    pub lru: list_lru_one,
    pub nr_items: atomic_long_t,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_lru {
    pub node: *mut list_lru_node,

    pub list: list_head,
    pub shrinker_id: c_int,
    pub memcg_aware: bool,
    pub xa: xarray,

    pub key: *mut lock_class_key,

}

extern "C" {
    pub fn list_lru_destroy(lru: *mut list_lru);
}

extern "C" {
    pub fn list_lru_init_memcg(_arg: lru, _arg: shrinker) -> return;
}

//
// folio_memcg_list_lru_alloc - allocate list_lru heads for shrinkable folio
// @folio: the newly allocated & charged folio
// @lru: the list_lru this might be queued on
// @gfp: gfp mask
//
// Allocate list_lru heads (per-memcg, per-node) needed to queue this
// particular folio down the line.
//
// This does memcg_list_lru_alloc(), but on the memcg that @folio is
// associated with. Handles folio_memcg() access rules in the fast
// path (list_lru heads allocated) and the allocation slowpath.
//
// Returns 0 on success, a negative error value otherwise.
//

extern "C" {
    pub fn memcg_reparent_list_lrus(memcg: *mut mem_cgroup, parent: *mut mem_cgroup);
}
//
// list_lru_lock: lock the sublist for the given node and memcg
// @lru: the lru pointer
// @nid: the node id of the sublist to lock.
// @memcg: pointer to the cgroup of the sublist to lock. On return,
// updated to the cgroup whose sublist was actually locked,
// which may be an ancestor if the original memcg was dying.
//
// Returns the locked list_lru_one sublist. The caller must call
// list_lru_unlock() when done.
//
// You must ensure that the memcg is not freed during this call (e.g., with
// rcu or by taking a css refcnt).
//
// Return: the locked list_lru_one, or NULL on failure
//
// list_lru_unlock: unlock a sublist locked by list_lru_lock()
// @l: the list_lru_one to unlock
//
extern "C" {
    pub fn list_lru_unlock(l: *mut list_lru_one);
}
extern "C" {
    pub fn list_lru_unlock_irq(l: *mut list_lru_one);
}
// Caller-locked variants, see list_lru_add() etc for documentation
//
// list_lru_add: add an element to the lru list's tail
// @lru: the lru pointer
// @item: the item to be added.
// @nid: the node id of the sublist to add the item to.
// @memcg: the cgroup of the sublist to add the item to.
//
// If the element is already part of a list, this function returns doing
// nothing. This means that it is not necessary to keep state about whether or
// not the element already belongs in the list. That said, this logic only
// works if the item is in *this* list. If the item might be in some other
// list, then you cannot rely on this check and you must remove it from the
// other list before trying to insert it.
//
// The lru list consists of many sublists internally; the @nid and @memcg
// parameters are used to determine which sublist to insert the item into.
// It's important to use the right value of @nid and @memcg when deleting the
// item, since it might otherwise get deleted from the wrong sublist.
//
// This also applies when attempting to insert the item multiple times - if
// the item is currently in one sublist and you call list_lru_add() again, you
// must pass the right @nid and @memcg parameters so that the same sublist is
// used.
//
// You must ensure that the memcg is not freed during this call (e.g., with
// rcu or by taking a css refcnt).
//
// Return: true if the list was updated, false otherwise
//
// list_lru_add_obj: add an element to the lru list's tail
// @lru: the lru pointer
// @item: the item to be added.
//
// This function is similar to list_lru_add(), but the NUMA node and the
// memcg of the sublist is determined by @item list_head. This assumption is
// valid for slab objects LRU such as dentries, inodes, etc.
//
// Return: true if the list was updated, false otherwise
//
extern "C" {
    pub fn list_lru_add_obj(lru: *mut list_lru, item: *mut list_head) -> bool;
}
//
// list_lru_del: delete an element from the lru list
// @lru: the lru pointer
// @item: the item to be deleted.
// @nid: the node id of the sublist to delete the item from.
// @memcg: the cgroup of the sublist to delete the item from.
//
// This function works analogously as list_lru_add() in terms of list
// manipulation.
//
// The comments in list_lru_add() about an element already being in a list are
// also valid for list_lru_del(), that is, you can delete an item that has
// already been removed or never been added. However, if the item is in a
// list, it must be in *this* list, and you must pass the right value of @nid
// and @memcg so that the right sublist is used.
//
// You must ensure that the memcg is not freed during this call (e.g., with
// rcu or by taking a css refcnt). When a memcg is deleted, list_lru entries
// are automatically moved to the parent memcg. This is done in a race-free
// way, so during deletion of an memcg both the old and new memcg will resolve
// to the same sublist internally.
//
// Return: true if the list was updated, false otherwise
//
// list_lru_del_obj: delete an element from the lru list
// @lru: the lru pointer
// @item: the item to be deleted.
//
// This function is similar to list_lru_del(), but the NUMA node and the
// memcg of the sublist is determined by @item list_head. This assumption is
// valid for slab objects LRU such as dentries, inodes, etc.
//
// Return: true if the list was updated, false otherwise.
//
extern "C" {
    pub fn list_lru_del_obj(lru: *mut list_lru, item: *mut list_head) -> bool;
}
//
// list_lru_count_one: return the number of objects currently held by @lru
// @lru: the lru pointer.
// @nid: the node id to count from.
// @memcg: the cgroup to count from.
//
// There is no guarantee that the list is not updated while the count is being
// computed. Callers that want such a guarantee need to provide an outer lock.
//
// Return: 0 for empty lists, otherwise the number of objects
// currently held by @lru.
//
extern "C" {
    pub fn list_lru_count_node(lru: *mut list_lru, nid: c_int) -> c_ulong;
}
extern "C" {
    pub fn list_lru_count_one(_arg: lru, _arg: sc->nid, _arg: sc->memcg) -> return;
}
extern "C" {
    pub fn list_lru_isolate(list: *mut list_lru_one, item: *mut list_head);
}
//
// list_lru_walk_one: walk a @lru, isolating and disposing freeable items.
// @lru: the lru pointer.
// @nid: the node id to scan from.
// @memcg: the cgroup to scan from.
// @isolate: callback function that is responsible for deciding what to do with
// the item currently being scanned
// @cb_arg: opaque type that will be passed to @isolate
// @nr_to_walk: how many items to scan.
//
// This function will scan all elements in a particular @lru, calling the
// @isolate callback for each of those items, along with the current list
// spinlock and a caller-provided opaque. The @isolate callback can choose to
// drop the lock internally, but *must* return with the lock held. The callback
// will return an enum lru_status telling the @lru infrastructure what to
// do with the object being scanned.
//
// Please note that @nr_to_walk does not mean how many objects will be freed,
// just how many objects will be scanned.
//
// Return: the number of objects effectively removed from the LRU.
//
// list_lru_walk_one_irq: walk a @lru, isolating and disposing freeable items.
// @lru: the lru pointer.
// @nid: the node id to scan from.
// @memcg: the cgroup to scan from.
// @isolate: callback function that is responsible for deciding what to do with
// the item currently being scanned
// @cb_arg: opaque type that will be passed to @isolate
// @nr_to_walk: how many items to scan.
//
// Same as list_lru_walk_one() except that the spinlock is acquired with
// spin_lock_irq().
//
