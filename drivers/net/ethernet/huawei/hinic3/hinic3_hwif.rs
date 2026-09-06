//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_hwif.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_func_type {
    HINIC3_FUNC_TYPE_PF = 0,
    HINIC3_FUNC_TYPE_VF = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_db_area {
    pub db_bitmap_array: *mut c_ulong,
    pub db_max_areas: u32,
// protect doorbell area alloc and free
    pub idx_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_func_attr {
    pub func_type: hinic3_func_type,
    pub func_global_idx: u16,
    pub global_vf_id_of_pf: u16,
    pub num_irqs: u16,
    pub num_sq: u16,
    pub port_to_port_idx: u8,
    pub pci_intf_idx: u8,
    pub ppf_idx: u8,
    pub num_aeqs: u8,
    pub num_ceqs: u8,
    pub msix_flex_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_hwif {
    pub cfg_regs_base: *mut u8 __iomem,
    pub intr_regs_base: *mut u8 __iomem,
    pub mgmt_regs_base: *mut u8 __iomem,
    pub db_base_phy: u64,
    pub db_dwqe_len: u64,
    pub db_base: *mut u8 __iomem,
    pub db_area: hinic3_db_area,
    pub attr: hinic3_func_attr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_outbound_ctrl {
    ENABLE_OUTBOUND  = 0x0,
    DISABLE_OUTBOUND = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_pf_status {
    HINIC3_PF_STATUS_INIT            = 0x0,
    HINIC3_PF_STATUS_ACTIVE_FLAG     = 0x11,
    HINIC3_PF_STATUS_FLR_START_FLAG  = 0x12,
    HINIC3_PF_STATUS_FLR_FINISH_FLAG = 0x13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_doorbell_ctrl {
    ENABLE_DOORBELL  = 0,
    DISABLE_DOORBELL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_msix_state {
    HINIC3_MSIX_ENABLE,
    HINIC3_MSIX_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_msix_auto_mask {
    HINIC3_CLR_MSIX_AUTO_MASK,
    HINIC3_SET_MSIX_AUTO_MASK,
}

extern "C" {
    pub fn hinic3_hwif_read_reg(hwif: *mut hinic3_hwif, reg: u32) -> u32;
}
extern "C" {
    pub fn hinic3_hwif_write_reg(hwif: *mut hinic3_hwif, reg: u32, val: u32);
}
extern "C" {
    pub fn hinic3_free_db_addr(hwdev: *mut hinic3_hwdev, db_base: *const u8 __iomem);
}
extern "C" {
    pub fn hinic3_get_pf_status(hwif: *mut hinic3_hwif) -> hinic3_pf_status;
}
extern "C" {
    pub fn hinic3_init_hwif(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_hwif(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_global_func_id(hwdev: *mut hinic3_hwdev) -> u16;
}
extern "C" {
    pub fn hinic3_pf_id_of_vf(hwdev: *mut hinic3_hwdev) -> u8;
}
extern "C" {
    pub fn hinic3_glb_pf_vf_offset(hwdev: *mut hinic3_hwdev) -> u16;
}
extern "C" {
    pub fn hinic3_ppf_idx(hwdev: *mut hinic3_hwdev) -> u8;
}
