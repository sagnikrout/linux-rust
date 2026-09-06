//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/intel_pmc_bxt.h
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
// GCR reg offsets from GCR base
pub const PMC_GCR_PMC_CFG_REG: c_uint = 0x08;
pub const PMC_GCR_TELEM_DEEP_S0IX_REG: c_uint = 0x78;
pub const PMC_GCR_TELEM_SHLW_S0IX_REG: c_uint = 0x80;
// PMC_CFG_REG bit masks

//
// struct intel_pmc_dev - Intel PMC device structure
// @dev: Pointer to the parent PMC device
// @scu: Pointer to the SCU IPC device data structure
// @gcr_mem_base: Virtual base address of GCR (Global Configuration Registers)
// @gcr_lock: Lock used to serialize access to GCR registers
// @telem_base: Pointer to telemetry SSRAM base resource or %NULL if not
// available
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pmc_dev {
    pub dev: *mut device,
    pub scu: *mut intel_scu_ipc_dev,
    pub gcr_mem_base: *mut void __iomem,
    pub gcr_lock: spinlock_t,
    pub telem_base: *mut resource,
}

extern "C" {
    pub fn intel_pmc_gcr_read64(pmc: *mut intel_pmc_dev, offset: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn intel_pmc_gcr_update(pmc: *mut intel_pmc_dev, offset: u32, mask: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_pmc_s0ix_counter_read(pmc: *mut intel_pmc_dev, data: *mut u64) -> c_int;
}

