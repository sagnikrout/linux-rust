//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cgroup_dmem.h
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
// Copyright © 2023-2024 Intel Corporation
//

// Opaque definition of a cgroup region, used internally
//
// struct dmem_cgroup_ops - Operations for a dmem cgroup region.
// @reclaim: Optional callback invoked when dmem.max is set below the current
// usage of a pool. The driver should attempt to free at least
// @target_bytes from @pool. May be called multiple times if usage
// remains above the limit after returning.
//
// Return: 0 if some progress was made (even if less than
// @target_bytes was freed), -ENOSPC if no progress could be made
// (the caller will retry up to a bounded number of times), or
// another negative error code if a fatal error occurred (stops
// further reclaim attempts immediately).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmem_cgroup_ops {
    pub priv): *mut u64 target_bytes, void,
}

//
// struct dmem_cgroup_init - Initialization parameters for a dmem cgroup region.
// @size: Size of the region in bytes.
// @ops: Optional operations for this region. May be NULL.
// @reclaim_priv: Opaque pointer passed to @ops->reclaim. May be NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmem_cgroup_init {
    pub size: u64,
    pub ops: *const dmem_cgroup_ops,
    pub reclaim_priv: *mut c_void,
}

extern "C" {
    pub fn dmem_cgroup_unregister_region(region: *mut dmem_cgroup_region);
}
extern "C" {
    pub fn dmem_cgroup_uncharge(pool: *mut dmem_cgroup_pool_state, size: u64);
}
extern "C" {
    pub fn dmem_cgroup_pool_state_put(pool: *mut dmem_cgroup_pool_state);
}

// ret_pool = NULL;
// ret_limit_pool = NULL;

