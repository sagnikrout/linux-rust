//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/btree.h
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
// THE BTREE:
//
// At a high level, bcache's btree is relatively standard b+ tree. All keys and
// pointers are in the leaves; interior nodes only have pointers to the child
// nodes.
//
// In the interior nodes, a struct bkey always points to a child btree node, and
// the key is the highest key in the child node - except that the highest key in
// an interior node is always MAX_KEY. The size field refers to the size on disk
// of the child node - this would allow us to have variable sized btree nodes
// (handy for keeping the depth of the btree 1 by expanding just the root).
//
// Btree nodes are themselves log structured, but this is hidden fairly
// thoroughly. Btree nodes on disk will in practice have extents that overlap
// (because they were written at different times), but in memory we never have
// overlapping extents - when we read in a btree node from disk, the first thing
// we do is resort all the sets of keys with a mergesort, and in the same pass
// we check for overlapping extents and adjust them appropriately.
//
// struct btree_op is a central interface to the btree code. It's used for
// specifying read vs. write locking, and the embedded closure is used for
// waiting on IO or reserve memory.
//
// BTREE CACHE:
//
// Btree nodes are cached in memory; traversing the btree might require reading
// in btree nodes which is handled mostly transparently.
//
// bch_btree_node_get() looks up a btree node in the cache and reads it in from
// disk if necessary. This function is almost never called directly though - the
// btree() macro is used to get a btree node, call some function on it, and
// unlock the node after the function returns.
//
// The root is special cased - it's taken out of the cache's lru (thus pinning
// it in memory), so we can find the root of the btree by just dereferencing a
// pointer instead of looking it up in the cache. This makes locking a bit
// tricky, since the root pointer is protected by the lock in the btree node it
// points to - the btree_root() macro handles this.
//
// In various places we must be able to allocate memory for multiple btree nodes
// in order to make forward progress. To do this we use the btree cache itself
// as a reserve; if __get_free_pages() fails, we'll find a node in the btree
// cache we can reuse. We can't allow more than one thread to be doing this at a
// time, so there's a lock, implemented by a pointer to the btree_op closure -
// this allows the btree_root() macro to implicitly release this lock.
//
// BTREE IO:
//
// Btree nodes never have to be explicitly read in; bch_btree_node_get() handles
// this.
//
// For writing, we have two btree_write structs embeddded in struct btree - one
// write in flight, and one being set up, and we toggle between them.
//
// Writing is done with a single function -  bch_btree_write() really serves two
// different purposes and should be broken up into two different functions. When
// passing now = false, it merely indicates that the node is now dirty - calling
// it ensures that the dirty keys will be written at some point in the future.
//
// When passing now = true, bch_btree_write() causes a write to happen
// "immediately" (if there was already a write in flight, it'll cause the write
// to happen as soon as the previous write completes). It returns immediately
// though - but it takes a refcount on the closure in struct btree_op you passed
// to it, so a closure_sync() later can be used to wait for the write to
// complete.
//
// This is handy because btree_split() and garbage collection can issue writes
// in parallel, reducing the amount of time they have to hold write locks.
//
// LOCKING:
//
// When traversing the btree, we may need write locks starting at some level -
// inserting a key into the btree will typically only require a write lock on
// the leaf node.
//
// This is specified with the lock field in struct btree_op; lock = 0 means we
// take write locks at level <= 0, i.e. only leaf nodes. bch_btree_node_get()
// checks this field and returns the node with the appropriate lock held.
//
// If, after traversing the btree, the insertion code discovers it has to split
// then it must restart from the root and take new locks - to do this it changes
// the lock field and returns -EINTR, which causes the btree_root() macro to
// loop.
//
// Handling cache misses require a different mechanism for upgrading to a write
// lock. We do cache lookups with only a read lock held, but if we get a cache
// miss and we wish to insert this data into the cache, we have to insert a
// placeholder key to detect races - otherwise, we could race with a write and
// overwrite the data that was just written to the cache with stale data from
// the backing device.
//
// For this we use a sequence number that write locks and unlocks increment - to
// insert the check key it unlocks the btree node and then takes a write lock,
// and fails if the sequence number doesn't match.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_write {
    pub journal: *mut core::sync::atomic::AtomicI32,
// If btree_split() frees a btree node, it writes a new pointer to that
// btree node indicating it was freed; it takes a refcount on
// c->prio_blocked because we can't write the gens until the new
// pointer is on disk. This allows btree_write_endio() to release the
// refcount that btree_split() took.
//
    pub prio_blocked: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree {
// Hottest entries first
    pub hash: hlist_node,
// Key/pointer for this btree node
    pub seq: c_ulong,
    pub lock: rw_semaphore,
    pub c: *mut cache_set,
    pub parent: *mut btree,
    pub write_lock: mutex,
    pub flags: c_ulong,
    pub /: *mut *mut uint16_t written; / would be nice to kill,
    pub level: u8,
    pub keys: btree_keys,
// For outstanding btree writes, used as a lock - protects write_idx
    pub io: closure,
    pub io_mutex: semaphore,
    pub list: list_head,
    pub work: delayed_work,
    pub writes: [btree_write; 2],
    pub bio: *mut bio,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btree_flags {
    BTREE_NODE_io_error,
    BTREE_NODE_dirty,
    BTREE_NODE_write_idx,
    BTREE_NODE_journal_flush,
}

extern "C" {
    pub fn bkey_put(c: *mut cache_set, k: *mut bkey);
}
// Looping macros

// Recursing down the btree
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_op {
// for waiting on btree reserve in btree_split()
    pub wait: wait_queue_entry_t,
// Btree level at which we start taking write locks
    pub lock: c_short,
    pub insert_collision:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_check_info {
    pub state: *mut btree_check_state,
    pub thread: *mut task_struct,
    pub result: c_int,
}

pub const BCH_BTR_CHKTHREAD_MAX: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_check_state {
    pub c: *mut cache_set,
    pub total_threads: c_int,
    pub key_idx: c_int,
    pub idx_lock: spinlock_t,
    pub started: core::sync::atomic::AtomicI32,
    pub enough: core::sync::atomic::AtomicI32,
    pub wait: wait_queue_head_t,
    pub infos: [btree_check_info; BCH_BTR_CHKTHREAD_MAX],
}

extern "C" {
    pub fn bch_btree_node_read_done(b: *mut btree);
}
extern "C" {
    pub fn __bch_btree_node_write(b: *mut btree, parent: *mut closure);
}
extern "C" {
    pub fn bch_btree_node_write(b: *mut btree, parent: *mut closure);
}
extern "C" {
    pub fn bch_btree_set_root(b: *mut btree);
}
extern "C" {
    pub fn bch_gc_thread_start(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bch_initial_gc_finish(c: *mut cache_set);
}
extern "C" {
    pub fn bch_moving_gc(c: *mut cache_set);
}
extern "C" {
    pub fn bch_btree_check(c: *mut cache_set) -> c_int;
}
extern "C" {
    pub fn bch_initial_mark_key(c: *mut cache_set, level: c_int, k: *mut bkey);
}
extern "C" {
    pub fn bch_cannibalize_unlock(c: *mut cache_set);
}
//
// Garbage collection thread only works when sectors_to_gc < 0,
// calling wake_up_gc() won't start gc thread if sectors_to_gc is
// not a nagetive value.
// Therefore sectors_to_gc is set to -1 here, before waking up
// gc thread by calling wake_up_gc(). Then gc_should_run() will
// give a chance to permit gc thread to run. "Give a chance" means
// before going into gc_should_run(), there is still possibility
// that c->sectors_to_gc being set to other positive value. So
// this routine won't 100% make sure gc thread will be woken up
// to run.
//
// These macros are for recursing down the btree - they handle the details of
// locking and looking up nodes in the cache for you. They're best treated as
// mere syntax when reading code that uses them.
//
// op->lock determines whether we take a read or a write lock at a given depth.
// If you've got a read lock and find that you need a write lock (i.e. you're
// going to have to split), set op->lock and return -EINTR; btree_root() will
// call you again and you'll have the correct lock.
//
// btree - recurse down the btree on a specified key
// @fn:		function to call, which will be passed the child node
// @key:	key to recurse on
// @b:		parent btree node
// @op:		pointer to struct btree_op
//

//
// btree_root - call a function on the root of the btree
// @fn:		function to call, which will be passed the child node
// @c:		cache set
// @op:		pointer to struct btree_op
//

pub const MAP_DONE: c_int = 0;
pub const MAP_CONTINUE: c_int = 1;
pub const MAP_ALL_NODES: c_int = 0;
pub const MAP_LEAF_NODES: c_int = 1;
pub const MAP_END_KEY: c_int = 1;
extern "C" {
    pub fn int(b_op: *mut btree_map_nodes_fn)(struct btree_op, b: *mut btree) -> typedef;
}
extern "C" {
    pub fn __bch_btree_map_nodes(_arg: op, _arg: c, _arg: from, _arg: fn, _arg: MAP_ALL_NODES) -> return;
}
extern "C" {
    pub fn __bch_btree_map_nodes(_arg: op, _arg: c, _arg: from, _arg: fn, _arg: MAP_LEAF_NODES) -> return;
}
extern "C" {
    pub fn bool(buf: *mut keybuf_pred_fn)(struct keybuf, k: *mut bkey) -> typedef;
}
extern "C" {
    pub fn bch_keybuf_init(buf: *mut keybuf);
}
extern "C" {
    pub fn bch_keybuf_del(buf: *mut keybuf, w: *mut keybuf_key);
}
extern "C" {
    pub fn bch_update_bucket_in_use(c: *mut cache_set, stats: *mut gc_stat);
}
