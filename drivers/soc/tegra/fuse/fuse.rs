//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/tegra/fuse/fuse.h
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
// Copyright (C) 2010 Google, Inc.
// Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
//
// Author:
// Colin Cross <ccross@android.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_fuse_info {
    pub offset): *mut *mut *mut u32 (read)(struct tegra_fuse fuse, unsigned int,
    pub size: c_uint,
    pub spare: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_fuse_soc {
    pub fuse): *mut *mut void (init)(struct tegra_fuse,
    pub info): *mut *mut void (speedo_init)(struct tegra_sku_info,
    pub fuse): *mut *mut int (probe)(struct tegra_fuse,
    pub info: *const tegra_fuse_info,
    pub lookups: *const nvmem_cell_lookup,
    pub num_lookups: c_uint,
    pub cells: *const nvmem_cell_info,
    pub num_cells: c_uint,
    pub keepouts: *const nvmem_keepout,
    pub num_keepouts: c_uint,
    pub soc_attr_group: *const attribute_group,
    pub clk_suspend_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_fuse {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub phys: phys_addr_t,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub offset): *mut *mut *mut u32 (read_early)(struct tegra_fuse fuse, unsigned int,
    pub offset): *mut *mut *mut u32 (read)(struct tegra_fuse fuse, unsigned int,
    pub soc: *const tegra_fuse_soc,
// APBDMA on Tegra20
    pub lock: mutex,
    pub wait: completion,
    pub chan: *mut dma_chan,
    pub config: dma_slave_config,
    pub phys: dma_addr_t,
    pub virt: *mut u32,
    pub apbdma: },
    pub nvmem: *mut nvmem_device,
    pub lookups: *mut nvmem_cell_lookup,
}

extern "C" {
    pub fn tegra_init_revision();
}
extern "C" {
    pub fn tegra_init_apbmisc();
}
extern "C" {
    pub fn tegra_acpi_init_apbmisc();
}
extern "C" {
    pub fn tegra_fuse_read_spare(spare: c_uint) -> u32 __init;
}
extern "C" {
    pub fn tegra_fuse_read_early(offset: c_uint) -> u32 __init;
}
extern "C" {
    pub fn tegra_get_major_rev() -> u8;
}
extern "C" {
    pub fn tegra_get_minor_rev() -> u8;
}

extern "C" {
    pub fn tegra20_init_speedo_data(sku_info: *mut tegra_sku_info);
}

extern "C" {
    pub fn tegra30_init_speedo_data(sku_info: *mut tegra_sku_info);
}

extern "C" {
    pub fn tegra114_init_speedo_data(sku_info: *mut tegra_sku_info);
}

extern "C" {
    pub fn tegra124_init_speedo_data(sku_info: *mut tegra_sku_info);
}

extern "C" {
    pub fn tegra210_init_speedo_data(sku_info: *mut tegra_sku_info);
}

