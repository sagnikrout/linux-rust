//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/shmobile/shmob_drm_drv.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// shmob_drm.h  --  SH Mobile DRM driver
//
// Copyright (C) 2012 Renesas Electronics Corporation
//
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_config {
    pub clk_source: shmob_drm_clk_source,
    pub clk_div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_device {
    pub dev: *mut device,
    pub pdata: *const shmob_drm_platform_data,
    pub config: shmob_drm_config,
    pub mmio: *mut void __iomem,
    pub clock: *mut clk,
    pub lddckr: u32,
    pub irq: c_uint,
    pub /: *mut *mut spinlock_t irq_lock; / Protects hardware LDINTR register,
    pub ddev: drm_device,
    pub crtc: shmob_drm_crtc,
    pub encoder: drm_encoder,
    pub connector: *mut drm_connector,
}

extern "C" {
    pub fn container_of(_arg: dev, shmob_drm_device: struct, _arg: ddev) -> return;
}
