//! Automatically rewritten from C to Rust
//! Source: scripts/gendwarfksyms/cache.c
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
// Copyright (C) 2024 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_item {
    pub key: c_ulong,
    pub value: c_int,
    pub hash: hlist_node,
}

#[no_mangle]
pub unsafe extern "C" fn cache_set(cache: *mut cache, key: c_ulong, value: c_int) {
    void cache_set(struct cache *cache, unsigned long key, int value)
    {
    struct cache_item *ci;
    ci = xmalloc(sizeof(*ci));
    ci.key = key;
    ci.value = value;
    hash_add(cache.cache, &ci.hash, hash_32(key));
    }
#[no_mangle]
pub unsafe extern "C" fn cache_get(cache: *mut cache, key: c_ulong) -> c_int {
    int cache_get(struct cache *cache, unsigned long key)
    {
    struct cache_item *ci;
    hash_for_each_possible(cache.cache, ci, hash, hash_32(key)) {
    if (ci.key == key)
    return ci.value;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cache_init(cache: *mut cache) {
    void cache_init(struct cache *cache)
    {
    hash_init(cache.cache);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_free(cache: *mut cache) {
    void cache_free(struct cache *cache)
    {
    struct hlist_node *tmp;
    struct cache_item *ci;
    hash_for_each_safe(cache.cache, ci, tmp, hash) {
    free(ci);
    }
    hash_init(cache.cache);
    }
