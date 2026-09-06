//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_ah_cache.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const EFA_AH_GID_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ah_cache_key {
    pub gid: [u8; EFA_AH_GID_SIZE],
    pub pd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ah_cache_entry {
    pub key: efa_ah_cache_key,
    pub ah: u16,
    pub usecnt: c_uint,
    pub refcount: refcount_t,
    pub linkage: rhash_head,
    pub /: *mut *mut mutex lock; / Serializes device commands per cache entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_ah_cache {
    pub hashtable: rhashtable,
    pub /: *mut *mut mutex lock; / Protects AH cache hashtable,
}

extern "C" {
    pub fn efa_ah_cache_init(ah_cache: *mut efa_ah_cache) -> c_int;
}
extern "C" {
    pub fn efa_ah_cache_destroy(ah_cache: *mut efa_ah_cache);
}
extern "C" {
    pub fn efa_ah_cache_put(ah_cache: *mut efa_ah_cache, entry: *mut efa_ah_cache_entry);
}
