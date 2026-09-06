//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/kmb/kmb_drv.h
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
// Copyright © 2018-2020 Intel Corporation
//

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 1;
// Platform definitions
pub const KMB_CRTC_MIN_VFP: c_int = 4;

pub const KMB_CRTC_MIN_WIDTH: c_int = 1920;
pub const KMB_CRTC_MIN_HEIGHT: c_int = 1080;
pub const KMB_FB_MAX_WIDTH: c_int = 1920;
pub const KMB_FB_MAX_HEIGHT: c_int = 1080;
pub const KMB_FB_MIN_WIDTH: c_int = 1;
pub const KMB_FB_MIN_HEIGHT: c_int = 1;

pub const KMB_LCD_DEFAULT_CLK: c_int = 200000000;
pub const KMB_SYS_CLK_MHZ: c_int = 500;
pub const ICAM_MMIO: c_uint = 0x3b100000;
pub const ICAM_LCD_OFFSET: c_uint = 0x1080;
pub const ICAM_MMIO_SIZE: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmb_clock {
    pub clk_lcd: *mut clk,
    pub clk_pll0: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmb_drm_private {
    pub drm: drm_device,
    pub kmb_dsi: *mut kmb_dsi,
    pub lcd_mmio: *mut void __iomem,
    pub kmb_clk: kmb_clock,
    pub crtc: drm_crtc,
    pub plane: *mut kmb_plane,
    pub state: *mut drm_atomic_commit,
    pub irq_lock: spinlock_t,
    pub irq_lcd: c_int,
    pub sys_clk_mhz: c_int,
    pub init_disp_cfg: [disp_cfg; KMB_MAX_PLANES],
    pub plane_status: [layer_status; KMB_MAX_PLANES],
    pub kmb_under_flow: c_int,
    pub kmb_flush_done: c_int,
    pub layer_no: c_int,
}

extern "C" {
    pub fn container_of(_arg: dev, kmb_drm_private: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn container_of(_arg: x, kmb_drm_private: struct, _arg: crtc) -> return;
}
extern "C" {
    pub fn readl(reg: dev_p->lcd_mmio +) -> return;
}
extern "C" {
    pub fn kmb_setup_crtc(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn kmb_set_scanout(lcd: *mut kmb_drm_private);
}
