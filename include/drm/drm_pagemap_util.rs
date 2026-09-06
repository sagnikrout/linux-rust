//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_pagemap_util.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

//
// struct drm_pagemap_peer - Structure representing a fast interconnect peer
// @list: Pointer to a &struct drm_pagemap_owner_list used to keep track of peers
// @link: List link for @list's list of peers.
// @owner: Pointer to a &struct drm_pagemap_owner, common for a set of peers having
// fast interconnects.
// @private: Pointer private to the struct embedding this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_peer {
    pub list: *mut drm_pagemap_owner_list,
    pub link: list_head,
    pub owner: *mut drm_pagemap_owner,
    pub private: *mut c_void,
}

//
// struct drm_pagemap_owner_list - Keeping track of peers and owners
// @peer: List of peers.
//
// The owner list defines the scope where we identify peers having fast interconnects
// and a common owner. Typically a driver has a single global owner list to
// keep track of common owners for the driver's pagemaps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pagemap_owner_list {
// @lock: Mutex protecting the @peers list.
    pub lock: mutex,
// @peers: List of peers.
    pub peers: list_head,
}

//
// Convenience macro to define an owner list.
// Typically the owner list statically declared
// driver-wide.
//

extern "C" {
    pub fn drm_pagemap_shrinker_add(dpagemap: *mut drm_pagemap);
}
extern "C" {
    pub fn drm_pagemap_cache_lock_lookup(cache: *mut drm_pagemap_cache) -> c_int;
}
extern "C" {
    pub fn drm_pagemap_cache_unlock_lookup(cache: *mut drm_pagemap_cache);
}
extern "C" {
    pub fn drm_pagemap_cache_set_pagemap(cache: *mut drm_pagemap_cache, dpagemap: *mut drm_pagemap);
}

extern "C" {
    pub fn drm_pagemap_shrinker_might_lock(dpagemap: *mut drm_pagemap);
}

extern "C" {
    pub fn drm_pagemap_release_owner(peer: *mut drm_pagemap_peer);
}
