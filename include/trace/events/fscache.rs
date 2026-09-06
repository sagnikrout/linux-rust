//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/fscache.h
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
// FS-Cache tracepoints
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Define enums for tracing information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_cache_trace {
    fscache_cache_collision,
    fscache_cache_get_acquire,
    fscache_cache_new_acquire,
    fscache_cache_put_alloc_volume,
    fscache_cache_put_cache,
    fscache_cache_put_prep_failed,
    fscache_cache_put_relinquish,
    fscache_cache_put_volume,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_volume_trace {
    fscache_volume_collision,
    fscache_volume_get_cookie,
    fscache_volume_get_create_work,
    fscache_volume_get_hash_collision,
    fscache_volume_get_withdraw,
    fscache_volume_free,
    fscache_volume_new_acquire,
    fscache_volume_put_cookie,
    fscache_volume_put_create_work,
    fscache_volume_put_hash_collision,
    fscache_volume_put_relinquish,
    fscache_volume_put_withdraw,
    fscache_volume_see_create_work,
    fscache_volume_see_hash_wake,
    fscache_volume_wait_create_work,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_cookie_trace {
    fscache_cookie_collision,
    fscache_cookie_discard,
    fscache_cookie_failed,
    fscache_cookie_get_attach_object,
    fscache_cookie_get_end_access,
    fscache_cookie_get_hash_collision,
    fscache_cookie_get_inval_work,
    fscache_cookie_get_lru,
    fscache_cookie_get_use_work,
    fscache_cookie_new_acquire,
    fscache_cookie_put_hash_collision,
    fscache_cookie_put_lru,
    fscache_cookie_put_object,
    fscache_cookie_put_over_queued,
    fscache_cookie_put_relinquish,
    fscache_cookie_put_withdrawn,
    fscache_cookie_put_work,
    fscache_cookie_see_active,
    fscache_cookie_see_lru_discard,
    fscache_cookie_see_lru_discard_clear,
    fscache_cookie_see_lru_do_one,
    fscache_cookie_see_relinquish,
    fscache_cookie_see_withdraw,
    fscache_cookie_see_work,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_active_trace {
    fscache_active_use,
    fscache_active_use_modify,
    fscache_active_unuse,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fscache_access_trace {
    fscache_access_acquire_volume,
    fscache_access_acquire_volume_end,
    fscache_access_cache_pin,
    fscache_access_cache_unpin,
    fscache_access_invalidate_cookie,
    fscache_access_invalidate_cookie_end,
    fscache_access_io_end,
    fscache_access_io_not_live,
    fscache_access_io_read,
    fscache_access_io_resize,
    fscache_access_io_wait,
    fscache_access_io_write,
    fscache_access_lookup_cookie,
    fscache_access_lookup_cookie_end,
    fscache_access_lookup_cookie_end_failed,
    fscache_access_relinquish_volume,
    fscache_access_relinquish_volume_end,
    fscache_access_unlive,
}

//
// Declare tracing information enums and their string mappings for display.
//

//
// Export enum symbols via userspace.
//

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

// This part must be outside protection
