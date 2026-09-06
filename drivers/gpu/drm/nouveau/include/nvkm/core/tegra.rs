//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/tegra.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_tegra {
    pub func: *const nvkm_device_tegra_func,
    pub device: nvkm_device,
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub rst: *mut reset_control,
    pub clk: *mut clk,
    pub clk_ref: *mut clk,
    pub clk_pwr: *mut clk,
    pub vdd: *mut regulator,
    pub pmc: *mut tegra_pmc,
//
// Protects accesses to mm from subsystems
//
    pub mutex: mutex,
    pub mm: nvkm_mm,
    pub domain: *mut iommu_domain,
    pub pgshift: c_ulong,
    pub iommu: },
    pub gpu_speedo: c_int,
    pub gpu_speedo_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_tegra_func {
//
// If an IOMMU is used, indicates which address bit will trigger a
// IOMMU translation when set (when this bit is not set, IOMMU is
// bypassed). A value of 0 means an IOMMU is never used.
//
    pub iommu_bit: u8,
//
// Whether the chip requires a reference clock
//
    pub require_ref_clk: bool,
//
// Whether the chip requires the VDD regulator
//
    pub require_vdd: bool,
}
