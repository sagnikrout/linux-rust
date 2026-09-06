//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-cache-policy-internal.h
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
// Copyright (C) 2012 Red Hat. All rights reserved.
//
// This file is released under the GPL.
//

// ----------------------------------------------------------------
// work = NULL;
// sz_ptr = sz;
// ----------------------------------------------------------------
//
// Some utility functions commonly used by policies and the core target.
//
extern "C" {
    pub fn sizeof(dm_div_up(nr_entries: *mut *mut unsigned long), _arg: BITS_PER_LONG) -> return;
}
extern "C" {
    pub fn vzalloc(_arg: s) -> return;
}
// ----------------------------------------------------------------
//
// Creates a new cache policy given a policy name, a cache size, an origin size and the block size.
//
// Destroys the policy.  This drops references to the policy module as well
// as calling it's destroy method.  So always use this rather than calling
// the policy->destroy method directly.
//
extern "C" {
    pub fn dm_cache_policy_destroy(p: *mut dm_cache_policy);
}
//
// In case we've forgotten.
//
extern "C" {
    pub fn dm_cache_policy_get_hint_size(p: *mut dm_cache_policy) -> usize;
}
// ----------------------------------------------------------------
