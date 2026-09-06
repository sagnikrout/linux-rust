//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/snoc.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2018 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_snoc_drv_priv {
    pub hw_rev: ath10k_hw_rev,
    pub dma_mask: u64,
    pub msa_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snoc_state {
    pub pipe_cfg_addr: u32,
    pub svc_to_pipe_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_snoc_pipe {
    pub ce_hdl: *mut ath10k_ce_pipe,
    pub pipe_num: u8,
    pub hif_ce_state: *mut ath10k,
    pub buf_sz: usize,
// protect ce info
    pub pipe_lock: spinlock_t,
    pub ar_snoc: *mut ath10k_snoc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_snoc_target_info {
    pub target_version: u32,
    pub target_type: u32,
    pub target_revision: u32,
    pub soc_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_snoc_ce_irq {
    pub irq_line: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_snoc_flags {
    ATH10K_SNOC_FLAG_REGISTERED,
    ATH10K_SNOC_FLAG_UNREGISTERING,
    ATH10K_SNOC_FLAG_MODEM_STOPPED,
    ATH10K_SNOC_FLAG_RECOVERY,
    ATH10K_SNOC_FLAG_8BIT_HOST_CAP_QUIRK,
    ATH10K_SNOC_FLAG_SKIP_HOST_CAP_QUIRK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_snoc {
    pub dev: *mut platform_device,
    pub ar: *mut ath10k,
    pub use_tz: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_firmware {
    pub dev: *mut device,
    pub fw_start_addr: dma_addr_t,
    pub iommu_domain: *mut iommu_domain,
    pub mapped_mem_size: usize,
    pub fw: },
    pub mem: *mut void __iomem,
    pub mem_pa: dma_addr_t,
    pub target_info: ath10k_snoc_target_info,
    pub mem_len: usize,
    pub pipe_info: [ath10k_snoc_pipe; CE_COUNT_MAX],
    pub ce_irqs: [ath10k_snoc_ce_irq; CE_COUNT_MAX],
    pub ce: ath10k_ce,
    pub rx_post_retry: timer_list,
    pub pwrseq: *mut pwrseq_desc,
    pub vregs: *mut regulator_bulk_data,
    pub num_vregs: usize,
    pub clks: *mut clk_bulk_data,
    pub num_clks: usize,
    pub qmi: *mut ath10k_qmi,
    pub nb: notifier_block,
    pub notifier: *mut c_void,
    pub flags: c_ulong,
    pub xo_cal_supported: bool,
    pub xo_cal_data: u32,
    pub CE_COUNT_MAX): DECLARE_BITMAP(pending_ce_irqs,,
}

extern "C" {
    pub fn ath10k_snoc_fw_indication(ar: *mut ath10k, type: u64) -> c_int;
}
extern "C" {
    pub fn ath10k_snoc_fw_crashed_dump(ar: *mut ath10k);
}
