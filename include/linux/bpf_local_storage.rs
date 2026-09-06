//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_local_storage.h
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
// Copyright (c) 2019 Facebook
// Copyright 2020 Google LLC.
//

pub const BPF_LOCAL_STORAGE_CACHE_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_map_bucket {
    pub list: hlist_head,
    pub lock: rqspinlock_t,
}

// Thp map is not the primary owner of a bpf_local_storage_elem.
// Instead, the container object (eg. sk->sk_bpf_storage) is.
//
// The map (bpf_local_storage_map) is for two purposes
// 1. Define the size of the "local storage".  It is
// the map's value_size.
//
// 2. Maintain a list to keep track of all elems such
// that they can be cleaned up during the map destruction.
//
// When a bpf local storage is being looked up for a
// particular object,  the "bpf_map" pointer is actually used
// as the "key" to search in the list of elem in
// the respective bpf_local_storage owned by the object.
//
// e.g. sk->sk_bpf_storage is the mini-map with the "bpf_map" pointer
// as the searching key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_map {
    pub map: bpf_map,
// Lookup elem does not require accessing the map.
//
// Updating/Deleting requires a bucket lock to
// link/unlink the elem from the map.  Having
// multiple buckets to improve contention.
//
    pub buckets: *mut bpf_local_storage_map_bucket,
    pub bucket_log: u32,
    pub elem_size: u16,
    pub cache_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_data {
// smap is used as the searching key when looking up
// from the object's bpf_local_storage.
//
// Put it in the same cacheline as the data to minimize
// the number of cachelines accessed during the cache hit case.
//
    pub smap: *mut bpf_local_storage_map __rcu,
    pub __aligned(8): u8 data[],
}

// Linked to bpf_local_storage and bpf_local_storage_map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_elem {
    pub /: *mut *mut hlist_node map_node; / Linked to bpf_local_storage_map,
    pub /: *mut *mut hlist_node snode; / Linked to bpf_local_storage,
    pub local_storage: *mut bpf_local_storage __rcu,
    pub rcu: rcu_head,
    pub postpone: *mut *mut hlist_node free_node; / used to,
// bpf_selem_free
// after raw_spin_unlock
//
}

// 4 bytes hole
// The data is stored in another cacheline to minimize
// the number of cachelines access during a cache hit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage {
    pub cache: [*mut bpf_local_storage_data __rcu; BPF_LOCAL_STORAGE_CACHE_SIZE],
    pub /: *mut *mut hlist_head list; / List of bpf_local_storage_elem,
    pub of: *mut *mut *mut void owner; / The object that owns the above "list",
// bpf_local_storage_elem.
//
    pub rcu: rcu_head,
    pub /: *mut *mut rqspinlock_t lock; / Protect adding/removing from the "list",
    pub /: *mut *mut u64 mem_charge; / Copy of mem charged to owner. Protected by "lock",
    pub /: *mut *mut refcount_t owner_refcnt;/ Used to pin owner when map_free is uncharging,
}

// U16_MAX is much more than enough for sk local storage
// considering a tcp_sock is ~2k.
//

pub const BPF_LOCAL_STORAGE_CACHE_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_local_storage_cache {
    pub idx_lock: spinlock_t,
    pub idx_usage_counts: [u64; BPF_LOCAL_STORAGE_CACHE_SIZE],
}

// Helper functions for bpf_local_storage
extern "C" {
    pub fn bpf_local_storage_map_alloc_check(attr: *mut bpf_attr) -> c_int;
}
// If cacheit_lockit is false, this lookup function is lockless
// Fast path (cache hit)
// Slow path (cache miss)
extern "C" {
    pub fn SDATA(_arg: selem) -> return;
}
extern "C" {
    pub fn bpf_local_storage_destroy(local_storage: *mut bpf_local_storage) -> u32;
}
extern "C" {
    pub fn bpf_selem_unlink(selem: *mut bpf_local_storage_elem) -> c_int;
}
extern "C" {
    pub fn bpf_local_storage_map_mem_usage(map: *const bpf_map) -> u64;
}
