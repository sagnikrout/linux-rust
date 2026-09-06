//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/user_exp_rcv.h
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
// Copyright(c) 2020 - Cornelis Networks, Inc.
// Copyright(c) 2015 - 2017 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_pageset {
    pub idx: u16,
    pub count: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_user_buf {
    pub notifier: mmu_interval_notifier,
    pub cover_mutex: mutex,
    pub vaddr: c_ulong,
    pub length: c_ulong,
    pub npages: c_uint,
    pub pages: *mut page,
    pub n_psets: c_uint,
    pub psets: [tid_pageset; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rb_node {
    pub notifier: mmu_interval_notifier,
    pub fdata: *mut hfi1_filedata,
    pub /: *mut *mut mutex invalidate_mutex; / covers hw removal,
    pub phys: c_ulong,
    pub grp: *mut tid_group,
    pub rcventry: u32,
    pub dma_addr: dma_addr_t,
    pub freed: bool,
    pub npages: c_uint,
    pub __counted_by(npages): *mut *mut page pages[],
}

extern "C" {
    pub fn hfi1_user_exp_rcv_free(fd: *mut hfi1_filedata);
}
