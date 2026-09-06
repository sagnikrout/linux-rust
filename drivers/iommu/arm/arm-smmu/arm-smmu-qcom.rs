//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/arm/arm-smmu/arm-smmu-qcom.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_smmu {
    pub smmu: arm_smmu_device,
    pub data: *const qcom_smmu_match_data,
    pub bypass_quirk: bool,
    pub bypass_cbndx: u8,
    pub stall_enabled: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_smmu_impl_reg_offset {
    QCOM_SMMU_TBU_PWR_STATUS,
    QCOM_SMMU_STATS_SYNC_INV_TBU_ACK,
    QCOM_SMMU_MMU2QSS_AND_SAFE_WAIT_CNTR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_smmu_config {
    pub reg_offset: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_smmu_match_data {
    pub cfg: *const qcom_smmu_config,
    pub impl: *const arm_smmu_impl,
    pub adreno_impl: *const arm_smmu_impl,
    pub client_match: *const *const of_device_id,
}

extern "C" {
    pub fn qcom_smmu_context_fault(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn qcom_smmu_tlb_sync_debug(smmu: *mut arm_smmu_device);
}
extern "C" {
    pub fn qcom_tbu_probe(pdev: *mut platform_device) -> c_int;
}

