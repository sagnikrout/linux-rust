//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/ahb.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH11K_AHB_SMP2P_SMEM_VALUE_MASK: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_ahb_smp2p_msg_id {
    ATH11K_AHB_POWER_SAVE_ENTER = 1,
    ATH11K_AHB_POWER_SAVE_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_ahb {
    pub tgt_rproc: *mut rproc,
    pub dev: *mut device,
    pub iommu_domain: *mut iommu_domain,
    pub msa_paddr: dma_addr_t,
    pub msa_size: u32,
    pub ce_paddr: dma_addr_t,
    pub ce_size: u32,
    pub use_tz: bool,
    pub fw: },
    pub seq_no: c_ushort,
    pub smem_bit: c_uint,
    pub smem_state: *mut qcom_smem_state,
    pub smp2p_info: },
}
