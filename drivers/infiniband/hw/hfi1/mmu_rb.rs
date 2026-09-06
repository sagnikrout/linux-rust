//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/mmu_rb.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2020 Cornelis Networks, Inc.
// Copyright(c) 2016 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_rb_node {
    pub addr: c_ulong,
    pub len: c_ulong,
    pub __last: c_ulong,
    pub node: rb_node,
    pub handler: *mut mmu_rb_handler,
    pub list: list_head,
    pub refcount: kref,
}

// filter and evict must not sleep. Only remove is allowed to sleep.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_rb_ops {
    pub len): c_ulong,
    pub mnode): *mut *mut *mut void (remove)(void ops_arg, struct mmu_rb_node,
    pub stop): *mut *mut void evict_arg, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_rb_handler {
//
// struct mmu_notifier is 56 bytes, and spinlock_t is 4 bytes, so
// they fit together in one cache line.  mn is relatively rarely
// accessed, so co-locating the spinlock with it achieves much of
// the cacheline contention reduction of giving the spinlock its own
// cacheline without the overhead of doing so.
//
    pub mn: mmu_notifier,
    pub /: *mut *mut spinlock_t lock; / protect the RB tree,
// Begin on a new cachline boundary here
    pub ____cacheline_aligned_in_smp: rb_root_cached root,
    pub ops_arg: *mut c_void,
    pub ops: *const mmu_rb_ops,
    pub lru_list: list_head,
    pub del_work: work_struct,
    pub del_list: list_head,
    pub wq: *mut workqueue_struct,
    pub free_ptr: *mut c_void,
}

extern "C" {
    pub fn hfi1_mmu_rb_unregister(handler: *mut mmu_rb_handler);
}
extern "C" {
    pub fn hfi1_mmu_rb_release(refcount: *mut kref);
}
extern "C" {
    pub fn hfi1_mmu_rb_evict(handler: *mut mmu_rb_handler, evict_arg: *mut c_void);
}
