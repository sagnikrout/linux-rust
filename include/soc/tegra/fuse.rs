//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/fuse.h
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
// Copyright (c) 2012-2023, NVIDIA CORPORATION.  All rights reserved.
//

pub const TEGRA20: c_uint = 0x20;
pub const TEGRA30: c_uint = 0x30;
pub const TEGRA114: c_uint = 0x35;
pub const TEGRA124: c_uint = 0x40;
pub const TEGRA132: c_uint = 0x13;
pub const TEGRA210: c_uint = 0x21;
pub const TEGRA186: c_uint = 0x18;
pub const TEGRA194: c_uint = 0x19;
pub const TEGRA234: c_uint = 0x23;
pub const TEGRA241: c_uint = 0x24;
pub const TEGRA264: c_uint = 0x26;
pub const TEGRA_FUSE_SKU_CALIB_0: c_uint = 0xf0;
pub const TEGRA30_FUSE_SATA_CALIB: c_uint = 0x124;
pub const TEGRA_FUSE_USB_CALIB_EXT_0: c_uint = 0x250;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_revision {
    TEGRA_REVISION_UNKNOWN = 0,
    TEGRA_REVISION_A01,
    TEGRA_REVISION_A02,
    TEGRA_REVISION_A03,
    TEGRA_REVISION_A03p,
    TEGRA_REVISION_A04,
    TEGRA_REVISION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_platform {
    TEGRA_PLATFORM_SILICON = 0,
    TEGRA_PLATFORM_QT,
    TEGRA_PLATFORM_SYSTEM_FPGA,
    TEGRA_PLATFORM_UNIT_FPGA,
    TEGRA_PLATFORM_ASIM_QT,
    TEGRA_PLATFORM_ASIM_LINSIM,
    TEGRA_PLATFORM_DSIM_ASIM_LINSIM,
    TEGRA_PLATFORM_VERIFICATION_SIMULATION,
    TEGRA_PLATFORM_VDK,
    TEGRA_PLATFORM_VSP,
    TEGRA_PLATFORM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_sku_info {
    pub sku_id: c_int,
    pub cpu_process_id: c_int,
    pub cpu_speedo_id: c_int,
    pub cpu_speedo_value: c_int,
    pub cpu_iddq_value: c_int,
    pub soc_process_id: c_int,
    pub soc_speedo_id: c_int,
    pub soc_speedo_value: c_int,
    pub gpu_process_id: c_int,
    pub gpu_speedo_id: c_int,
    pub gpu_speedo_value: c_int,
    pub revision: tegra_revision,
    pub platform: tegra_platform,
}

extern "C" {
    pub fn tegra_read_straps() -> u32;
}
extern "C" {
    pub fn tegra_read_ram_code() -> u32;
}
extern "C" {
    pub fn tegra_fuse_readl(offset: c_ulong, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn tegra_read_chipid() -> u32;
}
extern "C" {
    pub fn tegra_get_chip_id() -> u8;
}
extern "C" {
    pub fn tegra_get_platform() -> u8;
}
extern "C" {
    pub fn tegra_is_silicon() -> bool;
}
extern "C" {
    pub fn tegra194_miscreg_mask_serror() -> c_int;
}

