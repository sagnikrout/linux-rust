//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun8i_mixer.h
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
// Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
//

pub const SUN8I_MIXER_GLOBAL_CTL: c_uint = 0x0;
pub const SUN8I_MIXER_GLOBAL_STATUS: c_uint = 0x4;
pub const SUN8I_MIXER_GLOBAL_DBUFF: c_uint = 0x8;
pub const SUN8I_MIXER_GLOBAL_SIZE: c_uint = 0xc;
pub const SUN50I_MIXER_GLOBAL_SIZE: c_uint = 0x8;
pub const SUN50I_MIXER_GLOBAL_CLK: c_uint = 0xc;

pub const DE2_MIXER_UNIT_SIZE: c_uint = 0x6000;
pub const DE3_MIXER_UNIT_SIZE: c_uint = 0x3000;
pub const DE2_BLD_BASE: c_uint = 0x1000;
pub const DE2_CH_BASE: c_uint = 0x2000;
pub const DE2_CH_SIZE: c_uint = 0x1000;
pub const DE3_BLD_BASE: c_uint = 0x0800;
pub const DE3_CH_BASE: c_uint = 0x1000;
pub const DE3_CH_SIZE: c_uint = 0x0800;
pub const DE33_CH_BASE: c_uint = 0x1000;
pub const DE33_CH_SIZE: c_uint = 0x20000;

// colors are always in AARRGGBB format
pub const SUN8I_MIXER_BLEND_COLOR_BLACK: c_uint = 0xff000000;
// The following numbers are some still unknown magic numbers
pub const SUN8I_MIXER_BLEND_MODE_DEF: c_uint = 0x03010301;

pub const SUN8I_MIXER_FBFMT_ARGB8888: c_int = 0;
pub const SUN8I_MIXER_FBFMT_ABGR8888: c_int = 1;
pub const SUN8I_MIXER_FBFMT_RGBA8888: c_int = 2;
pub const SUN8I_MIXER_FBFMT_BGRA8888: c_int = 3;
pub const SUN8I_MIXER_FBFMT_XRGB8888: c_int = 4;
pub const SUN8I_MIXER_FBFMT_XBGR8888: c_int = 5;
pub const SUN8I_MIXER_FBFMT_RGBX8888: c_int = 6;
pub const SUN8I_MIXER_FBFMT_BGRX8888: c_int = 7;
pub const SUN8I_MIXER_FBFMT_RGB888: c_int = 8;
pub const SUN8I_MIXER_FBFMT_BGR888: c_int = 9;
pub const SUN8I_MIXER_FBFMT_RGB565: c_int = 10;
pub const SUN8I_MIXER_FBFMT_BGR565: c_int = 11;
pub const SUN8I_MIXER_FBFMT_ARGB4444: c_int = 12;
pub const SUN8I_MIXER_FBFMT_ABGR4444: c_int = 13;
pub const SUN8I_MIXER_FBFMT_RGBA4444: c_int = 14;
pub const SUN8I_MIXER_FBFMT_BGRA4444: c_int = 15;
pub const SUN8I_MIXER_FBFMT_ARGB1555: c_int = 16;
pub const SUN8I_MIXER_FBFMT_ABGR1555: c_int = 17;
pub const SUN8I_MIXER_FBFMT_RGBA5551: c_int = 18;
pub const SUN8I_MIXER_FBFMT_BGRA5551: c_int = 19;
pub const SUN8I_MIXER_FBFMT_ARGB2101010: c_int = 20;
pub const SUN8I_MIXER_FBFMT_ABGR2101010: c_int = 21;
pub const SUN8I_MIXER_FBFMT_RGBA1010102: c_int = 22;
pub const SUN8I_MIXER_FBFMT_BGRA1010102: c_int = 23;
pub const SUN8I_MIXER_FBFMT_YUYV: c_int = 0;
pub const SUN8I_MIXER_FBFMT_UYVY: c_int = 1;
pub const SUN8I_MIXER_FBFMT_YVYU: c_int = 2;
pub const SUN8I_MIXER_FBFMT_VYUY: c_int = 3;
pub const SUN8I_MIXER_FBFMT_NV16: c_int = 4;
pub const SUN8I_MIXER_FBFMT_NV61: c_int = 5;
pub const SUN8I_MIXER_FBFMT_YUV422: c_int = 6;
// format 7 doesn't exist
pub const SUN8I_MIXER_FBFMT_NV12: c_int = 8;
pub const SUN8I_MIXER_FBFMT_NV21: c_int = 9;
pub const SUN8I_MIXER_FBFMT_YUV420: c_int = 10;
// format 11 doesn't exist
// format 12 is semi-planar YUV411 UVUV
// format 13 is semi-planar YUV411 VUVU
pub const SUN8I_MIXER_FBFMT_YUV411: c_int = 14;
// format 15 doesn't exist
pub const SUN8I_MIXER_FBFMT_P010_YUV: c_int = 16;
// format 17 is P010 YVU
pub const SUN8I_MIXER_FBFMT_P210_YUV: c_int = 18;
// format 19 is P210 YVU
// format 20 is packed YVU444 10-bit
// format 21 is packed YUV444 10-bit
//
// Sub-engines listed bellow are unused for now. The EN registers are here only
// to be used to disable these sub-engines.
//
pub const SUN8I_MIXER_FCE_EN: c_uint = 0xa0000;
pub const SUN8I_MIXER_BWS_EN: c_uint = 0xa2000;
pub const SUN8I_MIXER_LTI_EN: c_uint = 0xa4000;
pub const SUN8I_MIXER_PEAK_EN: c_uint = 0xa6000;
pub const SUN8I_MIXER_ASE_EN: c_uint = 0xa8000;
pub const SUN8I_MIXER_FCC_EN: c_uint = 0xaa000;
pub const SUN8I_MIXER_DCSC_EN: c_uint = 0xb0000;
pub const SUN50I_MIXER_FCE_EN: c_uint = 0x70000;
pub const SUN50I_MIXER_PEAK_EN: c_uint = 0x70800;
pub const SUN50I_MIXER_LCTI_EN: c_uint = 0x71000;
pub const SUN50I_MIXER_BLS_EN: c_uint = 0x71800;
pub const SUN50I_MIXER_FCC_EN: c_uint = 0x72000;
pub const SUN50I_MIXER_DNS_EN: c_uint = 0x80000;
pub const SUN50I_MIXER_DRC_EN: c_uint = 0xa0000;
pub const SUN50I_MIXER_FMT_EN: c_uint = 0xa8000;
pub const SUN50I_MIXER_CDC0_EN: c_uint = 0xd0000;
pub const SUN50I_MIXER_CDC1_EN: c_uint = 0xd8000;
// First mixer or second mixer with VEP support.
// Second mixer without VEP support.
// First mixer with the MMIO layout found in the D1 SoC.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sun8i_mixer_type {
    SUN8I_MIXER_DE2,
    SUN8I_MIXER_DE3,
    SUN8I_MIXER_DE33,
}

//
// struct sun8i_layer_cfg - layer configuration
// @vi_scaler_num: Number of VI scalers. Used on DE2 and DE3.
// @scaler_mask: bitmask which tells which channel supports scaling
// First, scaler supports for VI channels is defined and after that, scaler
// support for UI channels. For example, if mixer has 2 VI channels without
// scaler and 2 UI channels with scaler, bitmask would be 0xC.
// @ccsc: select set of CCSC base addresses from the enumeration above.
// @de_type: sun8i_mixer_type enum representing the display engine generation.
// @scaline_yuv: size of a scanline for VI scaler for YUV formats.
// @de2_fcc_alpha: use FCC for missing DE2 VI alpha capability
// Most DE2 cores has FCC. If number of VI planes is one, enable this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_layer_cfg {
    pub vi_scaler_num: c_uint,
    pub scaler_mask: c_int,
    pub ccsc: c_int,
    pub de_type: c_uint,
    pub scanline_yuv: c_uint,
    pub 1: unsigned int de2_fcc_alpha :,
}

//
// struct sun8i_mixer_cfg - mixer HW configuration
// @lay_cfg: layer configuration
// @vi_num: number of VI channels
// @ui_num: number of UI channels
// @de_type: sun8i_mixer_type enum representing the display engine generation.
// @mod_rate: module clock rate that needs to be set in order to have
// a functional block.
// @map: channel map for DE variants processing YUV separately (DE33)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_mixer_cfg {
    pub lay_cfg: sun8i_layer_cfg,
    pub vi_num: c_int,
    pub ui_num: c_int,
    pub de_type: c_uint,
    pub mod_rate: c_ulong,
    pub map: [c_uint; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_mixer {
    pub engine: sunxi_engine,
    pub cfg: *const sun8i_mixer_cfg,
    pub reset: *mut reset_control,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub top_regs: *mut regmap,
    pub disp_regs: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_layer {
    pub plane: drm_plane,
    pub type: c_int,
    pub index: c_int,
    pub channel: c_int,
    pub overlay: c_int,
    pub regs: *mut regmap,
    pub cfg: *const sun8i_layer_cfg,
}

extern "C" {
    pub fn container_of(_arg: plane, sun8i_layer: struct, _arg: plane) -> return;
}
extern "C" {
    pub fn container_of(_arg: engine, sun8i_mixer: struct, _arg: engine) -> return;
}
extern "C" {
    pub fn sun8i_mixer_drm_format_to_hw(format: u32, hw_format: *mut u32) -> c_int;
}
