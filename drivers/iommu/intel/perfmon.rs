//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/intel/perfmon.h
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
//
// PERFCFGOFF_REG, PERFFRZOFF_REG
// PERFOVFOFF_REG, PERFCNTROFF_REG
//
pub const IOMMU_PMU_NUM_OFF_REGS: c_int = 4;
pub const IOMMU_PMU_OFF_REGS_STEP: c_int = 4;
pub const IOMMU_PMU_FILTER_REQUESTER_ID: c_uint = 0x01;
pub const IOMMU_PMU_FILTER_DOMAIN: c_uint = 0x02;
pub const IOMMU_PMU_FILTER_PASID: c_uint = 0x04;
pub const IOMMU_PMU_FILTER_ATS: c_uint = 0x08;
pub const IOMMU_PMU_FILTER_PAGE_TABLE: c_uint = 0x10;

pub const IOMMU_PMU_CFG_OFFSET: c_uint = 0x100;
pub const IOMMU_PMU_CFG_CNTRCAP_OFFSET: c_uint = 0x80;
pub const IOMMU_PMU_CFG_CNTREVCAP_OFFSET: c_uint = 0x84;
pub const IOMMU_PMU_CFG_SIZE: c_uint = 0x8;
pub const IOMMU_PMU_CFG_FILTERS_OFFSET: c_uint = 0x4;
pub const IOMMU_PMU_CAP_REGS_STEP: c_int = 8;

pub const IOMMU_EVENT_CFG_EGI_SHIFT: c_int = 8;
pub const IOMMU_EVENT_CFG_ES_SHIFT: c_int = 32;

extern "C" {
    pub fn alloc_iommu_pmu(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn free_iommu_pmu(iommu: *mut intel_iommu);
}
extern "C" {
    pub fn iommu_pmu_register(iommu: *mut intel_iommu);
}
extern "C" {
    pub fn iommu_pmu_unregister(iommu: *mut intel_iommu);
}

