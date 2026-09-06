//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/dbring.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2019-2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbring_element {
    pub paddr: dma_addr_t,
    pub payload: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbring_data {
    pub data: *mut c_void,
    pub data_sz: u32,
    pub meta: wmi_dma_buf_release_meta_data,
    pub buff: *mut ath11k_dbring_element,
    pub buf_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbring_buf_release_event {
    pub fixed: ath11k_wmi_dma_buf_release_fixed_param,
    pub buf_entry: *mut wmi_dma_buf_release_entry,
    pub meta_data: *mut wmi_dma_buf_release_meta_data,
    pub num_buf_entry: u32,
    pub num_meta: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbring_cap {
    pub pdev_id: u32,
    pub id: wmi_direct_buffer_module,
    pub min_elem: u32,
    pub min_buf_sz: u32,
    pub min_buf_align: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_dbring {
    pub refill_srng: dp_srng,
    pub bufs_idr: idr,
// Protects bufs_idr
    pub idr_lock: spinlock_t,
    pub tp_addr: dma_addr_t,
    pub hp_addr: dma_addr_t,
    pub bufs_max: c_int,
    pub pdev_id: u32,
    pub buf_sz: u32,
    pub buf_align: u32,
    pub num_resp_per_event: u32,
    pub event_timeout_ms: u32,
    pub ): *mut *mut *mut int (handler)(struct ath11k , struct ath11k_dbring_data,
}

extern "C" {
    pub fn ath11k_dbring_srng_cleanup(ar: *mut ath11k, ring: *mut ath11k_dbring);
}
extern "C" {
    pub fn ath11k_dbring_buf_cleanup(ar: *mut ath11k, ring: *mut ath11k_dbring);
}
extern "C" {
    pub fn ath11k_dbring_validate_buffer(ar: *mut ath11k, data: *mut c_void, size: u32) -> c_int;
}
