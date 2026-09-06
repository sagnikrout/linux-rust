//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dibs/dibs_loopback.h
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
// dibs loopback (aka loopback-ism) device structure definitions.
//
// Copyright (c) 2024, Alibaba Inc.
//
// Author: Wen Gu <guwen@linux.alibaba.com>
// Tony Lu <tonylu@linux.alibaba.com>
//

pub const DIBS_LO_DMBS_HASH_BITS: c_int = 12;
pub const DIBS_LO_MAX_DMBS: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibs_lo_dmb_node {
    pub list: hlist_node,
    pub token: u64,
    pub len: u32,
    pub sba_idx: u32,
    pub cpu_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibs_lo_dev {
    pub dibs: *mut dibs_dev,
    pub dmb_cnt: core::sync::atomic::AtomicI32,
    pub dmb_ht_lock: rwlock_t,
    pub DIBS_LO_MAX_DMBS): DECLARE_BITMAP(sba_idx_mask,,
    pub DIBS_LO_DMBS_HASH_BITS): DECLARE_HASHTABLE(dmb_ht,,
    pub ldev_release: wait_queue_head_t,
}

extern "C" {
    pub fn dibs_loopback_init() -> c_int;
}
extern "C" {
    pub fn dibs_loopback_exit();
}

