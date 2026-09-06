//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw_loader_handle.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_loader_ae_data {
    pub state: c_uint,
    pub ustore_size: c_uint,
    pub free_addr: c_uint,
    pub free_size: c_uint,
    pub live_ctx_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_loader_hal_handle {
    pub aes: [icp_qat_fw_loader_ae_data; ICP_QAT_UCLO_MAX_AE],
    pub ae_mask: c_uint,
    pub admin_ae_mask: c_uint,
    pub slice_mask: c_uint,
    pub revision_id: c_uint,
    pub ae_max_num: c_uint,
    pub upc_mask: c_uint,
    pub max_ustore: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_loader_chip_info {
    pub mmp_sram_size: c_int,
    pub nn: bool,
    pub lm2lm3: bool,
    pub reset_delay_us: u16,
    pub lm_size: u32,
    pub icp_rst_csr: u32,
    pub icp_rst_mask: u32,
    pub glb_clk_enable_csr: u32,
    pub misc_ctl_csr: u32,
    pub wakeup_event_val: u32,
    pub fw_auth: bool,
    pub css_3k: bool,
    pub dual_sign: bool,
    pub tgroup_share_ustore: bool,
    pub fcu_ctl_csr: u32,
    pub fcu_sts_csr: u32,
    pub fcu_dram_addr_hi: u32,
    pub fcu_dram_addr_lo: u32,
    pub fcu_loaded_ae_csr: u32,
    pub fcu_loaded_ae_pos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_loader_handle {
    pub hal_handle: *mut icp_qat_fw_loader_hal_handle,
    pub chip_info: *mut icp_qat_fw_loader_chip_info,
    pub pci_dev: *mut pci_dev,
    pub obj_handle: *mut c_void,
    pub sobj_handle: *mut c_void,
    pub mobj_handle: *mut c_void,
    pub cfg_ae_mask: c_uint,
    pub hal_sram_addr_v: *mut void __iomem,
    pub hal_cap_g_ctl_csr_addr_v: *mut void __iomem,
    pub hal_cap_ae_xfer_csr_addr_v: *mut void __iomem,
    pub hal_cap_ae_local_csr_addr_v: *mut void __iomem,
    pub hal_ep_csr_addr_v: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_firml_dram_desc {
    pub dram_base_addr: *mut void __iomem,
    pub dram_base_addr_v: *mut c_void,
    pub dram_bus_addr: dma_addr_t,
    pub dram_size: u64,
}
