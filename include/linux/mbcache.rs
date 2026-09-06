//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mbcache.h
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

// Cache entry flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb_cache_entry {
// List of entries in cache - protected by cache->c_list_lock
    pub e_list: list_head,
//
// Hash table list - protected by hash chain bitlock. The entry is
// guaranteed to be hashed while e_refcnt > 0.
//
    pub e_hash_list: hlist_bl_node,
//
// Entry refcount. Once it reaches zero, entry is unhashed and freed.
// While refcount > 0, the entry is guaranteed to stay in the hash and
// e.g. mb_cache_entry_try_delete() will fail.
//
    pub e_refcnt: core::sync::atomic::AtomicI32,
// Key in hash - stable during lifetime of the entry
    pub e_key: u32,
    pub e_flags: c_ulong,
// User provided value - stable during lifetime of the entry
    pub e_value: u64,
}

extern "C" {
    pub fn mb_cache_destroy(cache: *mut mb_cache);
}
extern "C" {
    pub fn mb_cache_entry_wait_unused(entry: *mut mb_cache_entry);
}
