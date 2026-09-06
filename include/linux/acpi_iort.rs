//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acpi_iort.h
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
// Copyright (C) 2016, Semihalf
// Author: Tomasz Nowicki <tn@semihalf.com>
//

//
// PMCG model identifiers for use in smmu pmu driver. Please note
// that this is purely for the use of software and has nothing to
// do with hardware or with IORT specification.
//
pub const IORT_SMMU_V3_PMCG_GENERIC: c_uint = 0x00000000 /* Generic SMMUv3 PMCG */;
pub const IORT_SMMU_V3_PMCG_HISI_HIP08: c_uint = 0x00000001 /* HiSilicon HIP08 PMCG */;
pub const IORT_SMMU_V3_PMCG_HISI_HIP09: c_uint = 0x00000002 /* HiSilicon HIP09 PMCG */;
extern "C" {
    pub fn iort_deregister_domain_token(trans_id: c_int);
}
extern "C" {
    pub fn iort_iwb_handle(iwb_id: u32) -> acpi_handle;
}

extern "C" {
    pub fn iort_msi_map_id(dev: *mut device, id: u32) -> u32;
}
extern "C" {
    pub fn iort_msi_xlate(dev: *mut device, id: u32, node: *mut fwnode_handle) -> u32;
}
extern "C" {
    pub fn iort_its_translate_pa(node: *mut fwnode_handle, base: *mut phys_addr_t) -> c_int;
}
extern "C" {
    pub fn iort_pmsi_get_msi_info(dev: *mut device, dev_id: *mut u32, pa: *mut phys_addr_t) -> c_int;
}
extern "C" {
    pub fn acpi_configure_pmsi_domain(dev: *mut device);
}
// IOMMU interface
extern "C" {
    pub fn iort_dma_get_ranges(dev: *mut device, limit: *mut u64) -> c_int;
}
extern "C" {
    pub fn iort_iommu_configure_id(dev: *mut device, id_in: *const u32) -> c_int;
}
extern "C" {
    pub fn iort_iommu_get_resv_regions(dev: *mut device, head: *mut list_head);
}
extern "C" {
    pub fn acpi_iort_dma_get_max_cpu_address() -> phys_addr_t;
}

// IOMMU interface

