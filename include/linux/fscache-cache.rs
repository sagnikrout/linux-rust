//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fscache-cache.h
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
// General filesystem caching backing cache interface
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// NOTE!!! See:
//
// Documentation/filesystems/caching/backend-api.rst
//
// for a description of the cache backend interface declared here.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_cache_state {
    FSCACHE_CACHE_IS_NOT_PRESENT,	/* No cache is present for this name */
    FSCACHE_CACHE_IS_PREPARING,	/* A cache is preparing to come live */
    FSCACHE_CACHE_IS_ACTIVE,	/* Attached cache is active and can be used */
    FSCACHE_CACHE_GOT_IOERROR,	/* Attached cache stopped on I/O error */
    FSCACHE_CACHE_IS_WITHDRAWN,	/* Attached cache is being withdrawn */

}

//
// Cache cookie.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscache_cache {
    pub ops: *const fscache_cache_ops,
    pub /: *mut *mut list_head cache_link; / Link in cache list,
    pub /: *mut *mut *mut void cache_priv; / Private cache data (or NULL),
    pub ref: refcount_t,
    pub /: *mut *mut atomic_t n_volumes; / Number of active volumes;,
    pub /: *mut *mut atomic_t n_accesses; / Number of in-progress accesses on the cache,
    pub /: *mut *mut atomic_t object_count; / no. of live objects in this cache,
    pub debug_id: c_uint,
    pub state: fscache_cache_state,
    pub name: *mut c_char,
}

//
// cache operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fscache_cache_ops {
// name of cache provider
    pub name: *const c_char,
// Acquire a volume
    pub volume): *mut *mut void (acquire_volume)(struct fscache_volume,
// Free the cache's data attached to a volume
    pub volume): *mut *mut void (free_volume)(struct fscache_volume,
// Look up a cookie in the cache
    pub cookie): *mut *mut bool (lookup_cookie)(struct fscache_cookie,
// Withdraw an object without any cookie access counts held
    pub cookie): *mut *mut void (withdraw_cookie)(struct fscache_cookie,
// Change the size of a data object
    pub new_size): loff_t,
// Invalidate an object
    pub cookie): *mut *mut bool (invalidate_cookie)(struct fscache_cookie,
// Begin an operation for the netfs lib
    pub want_state): fscache_want_state,
// Prepare to write to a live cache object
    pub cookie): *mut *mut void (prepare_to_write)(struct fscache_cookie,
}

//
// out-of-line cache backend functions
//
extern "C" {
    pub fn fscache_relinquish_cache(cache: *mut fscache_cache);
}
extern "C" {
    pub fn fscache_withdraw_cache(cache: *mut fscache_cache);
}
extern "C" {
    pub fn fscache_withdraw_volume(volume: *mut fscache_volume);
}
extern "C" {
    pub fn fscache_withdraw_cookie(cookie: *mut fscache_cookie);
}
extern "C" {
    pub fn fscache_io_error(cache: *mut fscache_cache);
}
extern "C" {
    pub fn fscache_cookie_lookup_negative(cookie: *mut fscache_cookie);
}
extern "C" {
    pub fn fscache_resume_after_invalidation(cookie: *mut fscache_cookie);
}
extern "C" {
    pub fn fscache_caching_failed(cookie: *mut fscache_cookie);
}
//
// fscache_cookie_state - Read the state of a cookie
// @cookie: The cookie to query
//
// Get the state of a cookie, imposing an ordering between the cookie contents
// and the state value.  Paired with fscache_set_cookie_state().
//
extern "C" {
    pub fn smp_load_acquire(_arg: &cookie->state) -> return;
}
//
// fscache_get_key - Get a pointer to the cookie key
// @cookie: The cookie to query
//
// Return a pointer to the where a cookie's key is stored.
//
// fscache_count_object - Tell fscache that an object has been added
// @cache: The cache to account to
//
// Tell fscache that an object has been added to the cache.  This prevents the
// cache from tearing down the cache structure until the object is uncounted.
//
// fscache_uncount_object - Tell fscache that an object has been removed
// @cache: The cache to account to
//
// Tell fscache that an object has been removed from the cache and will no
// longer be accessed.  After this point, the cache cookie may be destroyed.
//
// fscache_wait_for_objects - Wait for all objects to be withdrawn
// @cache: The cache to query
//
// Wait for all extant objects in a cache to finish being withdrawn
// and go away.
//

