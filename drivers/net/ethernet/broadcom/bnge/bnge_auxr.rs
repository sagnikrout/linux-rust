//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_auxr.h
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
// Copyright (c) 2025 Broadcom

pub const BNGE_MIN_ROCE_CP_RINGS: c_int = 2;
pub const BNGE_MIN_ROCE_STAT_CTXS: c_int = 1;
pub const BNGE_MAX_ROCE_MSIX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_msix_info {
    pub vector: u32,
    pub ring_idx: u32,
    pub db_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_fw_msg {
    pub msg: *mut c_void,
    pub msg_len: c_int,
    pub resp: *mut c_void,
    pub resp_max_len: c_int,
    pub timeout: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_auxr_info {
    pub handle: *mut c_void,
    pub msix_requested: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_auxr_dev {
    pub net: *mut net_device,
    pub pdev: *mut pci_dev,
    pub bar0: *mut void __iomem,
    pub msix_info: [bnge_msix_info; BNGE_MAX_ROCE_MSIX],
    pub flags: u32,
    pub auxr_info: *mut bnge_auxr_info,
// Doorbell BAR size in bytes mapped by L2 driver.
    pub l2_db_size: c_int,
// Doorbell BAR size in bytes mapped as non-cacheable.
    pub l2_db_size_nc: c_int,
// Doorbell offset in bytes within l2_db_size_nc.
    pub l2_db_offset: c_int,
    pub chip_num: u16,
    pub hw_ring_stats_size: u16,
    pub pf_port_id: u16,
    pub en_state: c_ulong,
    pub auxr_num_msix_vec: u16,
    pub auxr_num_ctxs: u16,
// serialize auxr operations
    pub auxr_dev_lock: mutex,
}

extern "C" {
    pub fn bnge_rdma_aux_device_uninit(bdev: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_rdma_aux_device_del(bdev: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_rdma_aux_device_add(bdev: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_rdma_aux_device_init(bdev: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_unregister_dev(adev: *mut bnge_auxr_dev);
}
extern "C" {
    pub fn bnge_send_msg(adev: *mut bnge_auxr_dev, fw_msg: *mut bnge_fw_msg) -> c_int;
}
