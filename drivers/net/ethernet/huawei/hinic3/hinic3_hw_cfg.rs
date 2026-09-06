//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_hw_cfg.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_irq {
    pub irq_id: u32,
    pub msix_entry_idx: u16,
    pub allocated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_irq_info {
    pub irq: *mut hinic3_irq,
    pub num_irq: u16,
// device max irq number
    pub num_irq_hw: u16,
// protect irq alloc and free
    pub irq_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_service_cap {
    pub max_sqs: u16,
    pub max_rqs: u16,
    pub default_num_queues: u16,
}

// Device capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dev_cap {
// Bitmasks of services supported by device
    pub supp_svcs_bitmap: u16,
// Physical port
    pub port_id: u8,
    pub cos_valid_bitmap: u8,
    pub port_cos_valid_bitmap: u8,
// max number of VFs that PF supports
    pub max_vf: u16,
    pub nic_svc_cap: hinic3_nic_service_cap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_cfg_mgmt_info {
    pub irq_info: hinic3_irq_info,
    pub cap: hinic3_dev_cap,
}

extern "C" {
    pub fn hinic3_init_cfg_mgmt(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_cfg_mgmt(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_free_irq(hwdev: *mut hinic3_hwdev, irq_id: u32);
}
extern "C" {
    pub fn hinic3_init_capability(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_support_nic(hwdev: *mut hinic3_hwdev) -> bool;
}
extern "C" {
    pub fn hinic3_func_max_qnum(hwdev: *mut hinic3_hwdev) -> u16;
}
extern "C" {
    pub fn hinic3_physical_port_id(hwdev: *mut hinic3_hwdev) -> u8;
}
