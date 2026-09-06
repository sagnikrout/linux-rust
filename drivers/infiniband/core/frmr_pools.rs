//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/frmr_pools.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frmr_handles_page {
    pub list: list_head,
    pub handles: [u32; NUM_HANDLES_PER_PAGE],
}

// FRMR queue holds a list of frmr_handles_page.
// num_pages: number of pages in the queue.
// ci: current index in the handles array across all pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frmr_queue {
    pub pages_list: list_head,
    pub num_pages: u32,
    pub ci: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_frmr_pool {
    pub node: rb_node,
    pub /: *mut *mut ib_frmr_key key; / Pool key,
// Protect access to the queue
    pub lock: spinlock_t,
    pub queue: frmr_queue,
    pub inactive_queue: frmr_queue,
    pub aging_work: delayed_work,
    pub device: *mut ib_device,
    pub max_in_use: u32,
    pub in_use: u32,
    pub pinned_handles: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_frmr_pools {
    pub rb_root: rb_root,
    pub rb_lock: rwlock_t,
    pub pool_ops: *const ib_frmr_pool_ops,
    pub aging_wq: *mut workqueue_struct,
    pub aging_period_sec: u32,
}

extern "C" {
    pub fn ib_frmr_pools_set_aging_period(device: *mut ib_device, period_sec: u32) -> c_int;
}
