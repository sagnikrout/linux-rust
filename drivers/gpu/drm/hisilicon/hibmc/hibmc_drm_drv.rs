//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/hibmc_drm_drv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Hisilicon Hibmc SoC drm driver
//
// Based on the bochs drm driver.
//
// Copyright (c) 2016 Huawei Limited.
//
// Author:
// Rongrong Zou <zourongrong@huawei.com>
// Rongrong Zou <zourongrong@gmail.com>
// Jianhua Li <lijianhua@huawei.com>
//

pub const HIBMC_MIN_VECTORS: c_int = 1;
pub const HIBMC_MAX_VECTORS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_vdac {
    pub dev: *mut drm_device,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub adapter: i2c_adapter,
    pub bit_data: i2c_algo_bit_data,
    pub phys_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_drm_private {
// hw
    pub mmio: *mut void __iomem,
// vram
    pub vram: *mut void __iomem,
    pub vram_base: resource_size_t,
    pub vram_size: resource_size_t,
// drm
    pub dev: drm_device,
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub vdac: hibmc_vdac,
    pub dp: hibmc_dp,
}

extern "C" {
    pub fn container_of(_arg: connector, hibmc_vdac: struct, _arg: connector) -> return;
}
extern "C" {
    pub fn container_of(_arg: connector, hibmc_dp: struct, _arg: connector) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, hibmc_drm_private: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn hibmc_de_init(priv: *mut hibmc_drm_private) -> c_int;
}
extern "C" {
    pub fn hibmc_vdac_init(priv: *mut hibmc_drm_private) -> c_int;
}
extern "C" {
    pub fn hibmc_ddc_create(drm_dev: *mut drm_device, connector: *mut hibmc_vdac) -> c_int;
}
extern "C" {
    pub fn hibmc_ddc_del(vdac: *mut hibmc_vdac);
}
extern "C" {
    pub fn hibmc_dp_init(priv: *mut hibmc_drm_private) -> c_int;
}
extern "C" {
    pub fn hibmc_debugfs_init(connector: *mut drm_connector, root: *mut dentry);
}
extern "C" {
    pub fn hibmc_dp_hpd_isr(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
