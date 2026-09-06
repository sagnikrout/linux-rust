//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mxsfb/mxsfb_drv.h
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
//
// Copyright (C) 2016 Marek Vasut <marex@denx.de>
//
// i.MX23/i.MX28/i.MX6SX MXSFB LCD controller driver.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxsfb_devdata {
    pub transfer_count: c_uint,
    pub cur_buf: c_uint,
    pub next_buf: c_uint,
    pub hs_wdth_mask: c_uint,
    pub hs_wdth_shift: c_uint,
    pub has_overlay: bool,
    pub has_ctrl2: bool,
    pub has_crc32: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxsfb_drm_private {
    pub devdata: *const mxsfb_devdata,
    pub /: *mut *mut *mut void __iomem base; / registers,
    pub clk: *mut clk,
    pub clk_axi: *mut clk,
    pub clk_disp_axi: *mut clk,
    pub irq: c_uint,
    pub drm: *mut drm_device,
    pub primary: drm_plane,
    pub overlay: drm_plane,
    pub planes: },
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: *mut drm_connector,
    pub bridge: *mut drm_bridge,
    pub crc_active: bool,
}

extern "C" {
    pub fn mxsfb_enable_axi_clk(mxsfb: *mut mxsfb_drm_private);
}
extern "C" {
    pub fn mxsfb_disable_axi_clk(mxsfb: *mut mxsfb_drm_private);
}
extern "C" {
    pub fn mxsfb_kms_init(mxsfb: *mut mxsfb_drm_private) -> c_int;
}
