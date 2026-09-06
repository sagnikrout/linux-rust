//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/amd/amd_iommu.h
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
// Copyright (C) 2009-2010 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <jroedel@suse.de>
//

extern "C" {
    pub fn amd_iommu_int_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn amd_iommu_int_thread_evtlog(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn amd_iommu_int_thread_pprlog(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn amd_iommu_int_thread_galog(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn amd_iommu_restart_event_logging(iommu: *mut amd_iommu);
}
extern "C" {
    pub fn amd_iommu_restart_ga_log(iommu: *mut amd_iommu);
}
extern "C" {
    pub fn amd_iommu_restart_ppr_log(iommu: *mut amd_iommu);
}
extern "C" {
    pub fn amd_iommu_set_rlookup_table(iommu: *mut amd_iommu, devid: u16);
}
extern "C" {
    pub fn iommu_feature_enable(iommu: *mut amd_iommu, bit: u8);
}

extern "C" {
    pub fn amd_iommu_debugfs_setup();
}

// Needed for interrupt remapping
extern "C" {
    pub fn amd_iommu_prepare() -> c_int;
}
extern "C" {
    pub fn amd_iommu_enable() -> c_int;
}
extern "C" {
    pub fn amd_iommu_disable();
}
extern "C" {
    pub fn amd_iommu_reenable(mode: c_int) -> c_int;
}
extern "C" {
    pub fn amd_iommu_enable_faulting(cpu: c_uint) -> c_int;
}
// Protection domain ops
extern "C" {
    pub fn amd_iommu_init_identity_domain();
}
extern "C" {
    pub fn amd_iommu_domain_free(dom: *mut iommu_domain);
}
// SVA/PASID
extern "C" {
    pub fn amd_iommu_pasid_supported() -> bool;
}
// IOPF
extern "C" {
    pub fn amd_iommu_iopf_init(iommu: *mut amd_iommu) -> c_int;
}
extern "C" {
    pub fn amd_iommu_iopf_uninit(iommu: *mut amd_iommu);
}
// GCR3 setup
extern "C" {
    pub fn amd_iommu_clear_gcr3(dev_data: *mut iommu_dev_data, pasid: ioasid_t) -> c_int;
}
// PPR
extern "C" {
    pub fn amd_iommu_alloc_ppr_log(iommu: *mut amd_iommu) -> int __init;
}
extern "C" {
    pub fn amd_iommu_free_ppr_log(iommu: *mut amd_iommu) -> void __init;
}
extern "C" {
    pub fn amd_iommu_enable_ppr_log(iommu: *mut amd_iommu);
}
extern "C" {
    pub fn amd_iommu_poll_ppr_log(iommu: *mut amd_iommu);
}
extern "C" {
    pub fn amd_iommu_complete_ppr(dev: *mut device, pasid: u32, status: c_int, tag: c_int) -> c_int;
}
//
// This function flushes all internal caches of
// the IOMMU used by this driver.
//
extern "C" {
    pub fn amd_iommu_flush_all_caches(iommu: *mut amd_iommu);
}

extern "C" {
    pub fn amd_iommu_create_irq_domain(iommu: *mut amd_iommu) -> c_int;
}

extern "C" {
    pub fn phys_to_virt(_arg: __sme_clr(paddr)) -> return;
}
extern "C" {
    pub fn PCI_SEG_DEVID_TO_SBDF(_arg: seg, _arg: devid) -> return;
}
extern "C" {
    pub fn amd_iommu_ht_range_ignore() -> bool;
}
//
// This must be called after device probe completes. During probe
// use rlookup_amd_iommu() get the iommu.
//
extern "C" {
    pub fn iommu_get_iommu_dev(_arg: dev, amd_iommu: struct, _arg: iommu) -> return;
}
// This must be called after device probe completes.
extern "C" {
    pub fn iommu_get_iommu_dev(_arg: dev_data->dev, amd_iommu: struct, _arg: iommu) -> return;
}
extern "C" {
    pub fn container_of(_arg: dom, protection_domain: struct, _arg: domain) -> return;
}
extern "C" {
    pub fn translation_pre_enabled(iommu: *mut amd_iommu) -> bool;
}
extern "C" {
    pub fn add_special_device(type: u8, id: u8, devid: *mut u32, cmd_line: bool) -> int __init;
}
extern "C" {
    pub fn amd_iommu_pdom_id_alloc() -> c_int;
}
extern "C" {
    pub fn amd_iommu_pdom_id_reserve(id: u16, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn amd_iommu_pdom_id_free(id: c_int);
}
extern "C" {
    pub fn amd_iommu_pdom_id_destroy();
}

extern "C" {
    pub fn amd_iommu_apply_ivrs_quirks();
}

// All existing DTE must have V bit set
//
// Restore cached persistent DTE bits, which can be set by information
// in IVRS table. See set_dev_entry_from_acpi().
//
// NESTED
