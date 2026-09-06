//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_dev.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2019 Netronome Systems, Inc.

pub const PCI_VENDOR_ID_CORIGINE: c_uint = 0x1da8;
pub const PCI_DEVICE_ID_NFP3800: c_uint = 0x3800;
pub const PCI_DEVICE_ID_NFP4000: c_uint = 0x4000;
pub const PCI_DEVICE_ID_NFP5000: c_uint = 0x5000;
pub const PCI_DEVICE_ID_NFP6000: c_uint = 0x6000;
pub const PCI_DEVICE_ID_NFP3800_VF: c_uint = 0x3803;
pub const PCI_DEVICE_ID_NFP6000_VF: c_uint = 0x6003;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_dev_id {
    NFP_DEV_NFP3800,
    NFP_DEV_NFP3800_VF,
    NFP_DEV_NFP6000,
    NFP_DEV_NFP6000_VF,
    NFP_DEV_CNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_dev_info {
// Required fields
    pub dma_mask: u64,
    pub qc_idx_mask: u32,
    pub qc_addr_offset: u32,
    pub min_qc_size: u32,
    pub max_qc_size: u32,
// PF-only fields
    pub chip_names: *const c_char,
    pub pcie_cfg_expbar_offset: u32,
    pub pcie_expl_offset: u32,
    pub qc_area_sz: u32,
}
