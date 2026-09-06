//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/cache.h
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
// include/linux/sunrpc/cache.h
//
// Generic code for various authentication-related caches
// used by sunrpc clients and servers.
//
// Copyright (C) 2002 Neil Brown <neilb@cse.unsw.edu.au>
//

//
// Each cache requires:
// - A 'struct cache_detail' which contains information specific to the cache
// for common code to use.
// - An item structure that must contain a "struct cache_head"
// - A lookup function defined using DefineCacheLookup
// - A 'put' function that can release a cache item. It will only
// be called after cache_put has succeed, so there are guarantee
// to be no references.
// - A function to calculate a hash of an item's key.
//
// as well as assorted code fragments (e.g. compare keys) and numbers
// (e.g. hash size, goal_age, etc).
//
// Each cache must be registered so that it can be cleaned regularly.
// When the cache is unregistered, it is flushed completely.
//
// Entries have a ref count and a 'hashed' flag which counts the existence
// in the hash table.
// We only expire entries when refcount is zero.
// Existence in the cache is counted  the refcount.
//
// Every cache item has a common header that is used
// for expiring and refreshing entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_head {
    pub cache_list: hlist_node,
    pub use: *mut *mut time64_t expiry_time; / After time expiry_time, don't,
// the data
    pub was: *mut *mut time64_t last_refresh; / If CACHE_PENDING, this is when upcall,
// sent, else this is when update was
// received, though it is alway set to
// be *after* ->flush_time.
//
    pub ref: kref,
    pub flags: c_ulong,
}

// cache_head.flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_detail {
    pub owner: *mut *mut module,
    pub hash_size: c_int,
    pub hash_table: *mut *mut hlist_head,
    pub hash_lock: spinlock_t,
    pub name: *mut c_char,
    pub ): *mut *mut void (cache_put)(struct kref,
    pub ): *mut cache_head,
    pub h): *mut cache_head,
    pub blen): *mut *mut *mut char bpp, int,
    pub len): *mut *mut char buf, int,
    pub h): *mut cache_head,
    pub has_died): c_int,
    pub (*alloc)(void): *mut *mut cache_head,
    pub (*flush)(void): *mut c_void,
    pub new): *mut *mut *mut int (match)(struct cache_head orig, struct cache_head,
    pub new): *mut *mut *mut void (init)(struct cache_head orig, struct cache_head,
    pub new): *mut *mut *mut void (update)(struct cache_head orig, struct cache_head,
// fields below this comment are for internal use
// and should not be touched by cache owners
//
    pub with: *mut *mut time64_t flush_time; / flush all cache items,
// last_refresh at or earlier
// than this.  last_refresh
// is never set at or earlier
// than this.
//
    pub others: list_head,
    pub nextcheck: time64_t,
    pub entries: c_int,
// fields for communication over channel
    pub requests: list_head,
    pub readers: list_head,
    pub queue_lock: spinlock_t,
    pub queue_wait: wait_queue_head_t,
    pub next_seqno: u64,
    pub /: *mut *mut atomic_t writers; / how many time is /channel open,
    pub /: *mut *mut time64_t last_close; / if no writers, when did last close,
    pub /: *mut *mut time64_t last_warn; / when we last warned about no writers,
    pub procfs: *mut proc_dir_entry,
    pub pipefs: *mut dentry,
}

// this must be embedded in any request structure that
// identifies an object that will want a callback on
// a cache fill
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_req {
    pub req): *mut *mut *mut cache_deferred_req (defer)(cache_req,
    pub the: *mut *mut unsigned long thread_wait; / How long (jiffies) we can block,
// current thread to wait for updates.
//
}

// this must be embedded in a deferred_request that is being
// delayed awaiting cache-fill
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_deferred_req {
    pub /: *mut *mut hlist_node hash; / on hash chain,
    pub /: *mut *mut list_head recent; / on fifo,
    pub /: *mut *mut *mut cache_head item; / cache item we wait on,
    pub requests: *mut *mut *mut void owner; / we might need to discard all defered,
// owned by someone
    pub too_many): c_int,
}

//
// timestamps kept in the cache are expressed in seconds
// since boot.  This is the best for measuring differences in
// real time.
// This reimplemnts ktime_get_boottime_seconds() in a slightly
// faster but less accurate way. When we end up converting
// back to wallclock (CLOCK_REALTIME), that error often
// cancels out during the reverse operation.
//
extern "C" {
    pub fn cache_clean_deferred(owner: *mut c_void);
}
extern "C" {
    pub fn cache_flush();
}
extern "C" {
    pub fn cache_purge(detail: *mut cache_detail);
}

extern "C" {
    pub fn cache_initialize() -> void __init;
}
extern "C" {
    pub fn cache_register_net(cd: *mut cache_detail, net: *mut net) -> c_int;
}
extern "C" {
    pub fn cache_unregister_net(cd: *mut cache_detail, net: *mut net);
}
extern "C" {
    pub fn cache_destroy_net(cd: *mut cache_detail, net: *mut net);
}
extern "C" {
    pub fn sunrpc_init_cache_detail(cd: *mut cache_detail);
}
extern "C" {
    pub fn sunrpc_destroy_cache_detail(cd: *mut cache_detail);
}
extern "C" {
    pub fn sunrpc_cache_unregister_pipefs(: *mut cache_detail);
}
extern "C" {
    pub fn sunrpc_cache_unhash(: *mut cache_detail, : *mut cache_head);
}
extern "C" {
    pub fn sunrpc_cache_requests_count(cd: *mut cache_detail) -> c_int;
}
// Must store cache_detail in seq_file->private if using next three functions
extern "C" {
    pub fn cache_seq_stop_rcu(file: *mut seq_file, p: *mut c_void);
}
extern "C" {
    pub fn qword_add(bpp: *mut c_char, lp: *mut c_int, str: *mut c_char);
}
extern "C" {
    pub fn qword_addhex(bpp: *mut c_char, lp: *mut c_int, buf: *mut c_char, blen: c_int);
}
extern "C" {
    pub fn qword_get(bpp: *mut c_char, dest: *mut c_char, bufsize: c_int) -> c_int;
}
// anint = rv;
// time = ll;
