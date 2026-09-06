//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/stm/ltdc.h
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
//
// Copyright (C) STMicroelectronics SA 2017
//
// Authors: Philippe Cornu <philippe.cornu@st.com>
// Yannick Fertre <yannick.fertre@st.com>
// Fabien Dessenne <fabien.dessenne@st.com>
// Mickael Reulier <mickael.reulier@st.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltdc_caps {
    pub /: *mut *mut u32 hw_version; / hardware version,
    pub /: *mut *mut u32 nb_layers; / number of supported layers,
    pub /: *mut *mut u32 layer_ofs; / layer offset for applicable regs,
    pub /: *const *const *const u32 layer_regs; / layer register offset,
    pub /: *mut *mut u32 bus_width; / bus width (32 or 64 bits),
    pub /: *const *const *const u32 pix_fmt_hw; / supported hw pixel formats,
    pub /: *const *const *const u32 pix_fmt_drm; / supported drm pixel formats,
    pub /: *mut *mut int pix_fmt_nb; / number of pixel format,
    pub /: *mut *mut bool pix_fmt_flex; / pixel format flexibility supported,
    pub /: *mut *mut bool non_alpha_only_l1; / non-native no-alpha formats on layer 1,
    pub /: *mut *mut int pad_max_freq_hz; / max frequency supported by pad,
    pub /: *mut *mut int nb_irq; / number of hardware interrupts,
    pub /: *mut *mut bool ycbcr_input; / ycbcr input converter supported,
    pub /: *mut *mut bool ycbcr_output; / ycbcr output converter supported,
    pub /: *mut *mut bool plane_reg_shadow; / plane shadow registers ability,
    pub /: *mut *mut bool crc; / cyclic redundancy check supported,
    pub /: *mut *mut bool dynamic_zorder; / dynamic z-order,
    pub /: *mut *mut bool plane_rotation; / plane rotation,
    pub /: *mut *mut bool fifo_threshold; / fifo underrun threshold supported,
}

pub const LTDC_MAX_LAYER: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fps_info {
    pub counter: c_uint,
    pub last_timestamp: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltdc_plat_data {
    pub /: *mut *mut int pad_max_freq_hz; / max frequency supported by pad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltdc_device {
    pub regs: *mut void __iomem,
    pub regmap: *mut regmap,
    pub /: *mut *mut *mut clk pixel_clk; / lcd pixel clock,
    pub /: *mut *mut *mut clk lvds_clk; / lvds pixel clock,
    pub /: *mut *mut *mut clk bus_clk; / bus clock,
    pub /: *mut *mut mutex err_lock; / protecting error_status,
    pub caps: ltdc_caps,
    pub irq_status: u32,
    pub /: *mut *mut u32 fifo_err; / fifo underrun error counter,
    pub /: *mut *mut u32 fifo_warn; / fifo underrun warning counter,
    pub /: *mut *mut u32 fifo_threshold; / fifo underrun threshold,
    pub /: *mut *mut u32 transfer_err; / transfer error counter,
    pub plane_fpsi: [fps_info; LTDC_MAX_LAYER],
    pub suspend_state: *mut drm_atomic_commit,
    pub crc_skip_count: c_int,
    pub crc_active: bool,
}

extern "C" {
    pub fn ltdc_load(ddev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn ltdc_unload(ddev: *mut drm_device);
}
extern "C" {
    pub fn ltdc_suspend(ddev: *mut drm_device);
}
extern "C" {
    pub fn ltdc_resume(ddev: *mut drm_device) -> c_int;
}
