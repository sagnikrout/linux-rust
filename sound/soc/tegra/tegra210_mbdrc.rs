//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_mbdrc.h
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
// tegra210_mbdrc.h - Definitions for Tegra210 MBDRC driver
//
// Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
//

// Register offsets from TEGRA210_MBDRC*_BASE
pub const TEGRA210_MBDRC_SOFT_RESET: c_uint = 0x4;
pub const TEGRA210_MBDRC_CG: c_uint = 0x8;
pub const TEGRA210_MBDRC_STATUS: c_uint = 0xc;
pub const TEGRA210_MBDRC_CFG: c_uint = 0x28;
pub const TEGRA210_MBDRC_CHANNEL_MASK: c_uint = 0x2c;
pub const TEGRA210_MBDRC_MASTER_VOL: c_uint = 0x30;
pub const TEGRA210_MBDRC_FAST_FACTOR: c_uint = 0x34;
pub const TEGRA210_MBDRC_FILTER_COUNT: c_int = 3;
pub const TEGRA210_MBDRC_FILTER_PARAM_STRIDE: c_uint = 0x4;
pub const TEGRA210_MBDRC_IIR_CFG: c_uint = 0x38;
pub const TEGRA210_MBDRC_IN_ATTACK: c_uint = 0x44;
pub const TEGRA210_MBDRC_IN_RELEASE: c_uint = 0x50;
pub const TEGRA210_MBDRC_FAST_ATTACK: c_uint = 0x5c;
pub const TEGRA210_MBDRC_IN_THRESHOLD: c_uint = 0x68;
pub const TEGRA210_MBDRC_OUT_THRESHOLD: c_uint = 0x74;
pub const TEGRA210_MBDRC_RATIO_1ST: c_uint = 0x80;
pub const TEGRA210_MBDRC_RATIO_2ND: c_uint = 0x8c;
pub const TEGRA210_MBDRC_RATIO_3RD: c_uint = 0x98;
pub const TEGRA210_MBDRC_RATIO_4TH: c_uint = 0xa4;
pub const TEGRA210_MBDRC_RATIO_5TH: c_uint = 0xb0;
pub const TEGRA210_MBDRC_MAKEUP_GAIN: c_uint = 0xbc;
pub const TEGRA210_MBDRC_INIT_GAIN: c_uint = 0xc8;
pub const TEGRA210_MBDRC_GAIN_ATTACK: c_uint = 0xd4;
pub const TEGRA210_MBDRC_GAIN_RELEASE: c_uint = 0xe0;
pub const TEGRA210_MBDRC_FAST_RELEASE: c_uint = 0xec;
pub const TEGRA210_MBDRC_CFG_RAM_CTRL: c_uint = 0xf8;
pub const TEGRA210_MBDRC_CFG_RAM_DATA: c_uint = 0x104;

// Fields for TEGRA210_MBDRC_CFG
pub const TEGRA210_MBDRC_CFG_RMS_OFFSET_SHIFT: c_int = 16;

pub const TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT: c_int = 14;

pub const TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT: c_int = 13;

pub const TEGRA210_MBDRC_CFG_SHIFT_CTRL_SHIFT: c_int = 8;

pub const TEGRA210_MBDRC_CFG_FRAME_SIZE_SHIFT: c_int = 4;

pub const TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_CHANNEL_MASK
pub const TEGRA210_MBDRC_CHANNEL_MASK_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_MASTER_VOL
pub const TEGRA210_MBDRC_MASTER_VOL_SHIFT: c_int = 23;

pub const TEGRA210_MBDRC_MASTER_VOL_MAX: c_int = 256;
// Fields for TEGRA210_MBDRC_FAST_FACTOR
pub const TEGRA210_MBDRC_FAST_FACTOR_RELEASE_SHIFT: c_int = 16;

pub const TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_IIR_CFG
pub const TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_IN_ATTACK
pub const TEGRA210_MBDRC_IN_ATTACK_TC_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_IN_RELEASE
pub const TEGRA210_MBDRC_IN_RELEASE_TC_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_FAST_ATTACK
pub const TEGRA210_MBDRC_FAST_ATTACK_TC_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_IN_THRESHOLD / TEGRA210_MBDRC_OUT_THRESHOLD
pub const TEGRA210_MBDRC_THRESH_4TH_SHIFT: c_int = 24;

pub const TEGRA210_MBDRC_THRESH_3RD_SHIFT: c_int = 16;

pub const TEGRA210_MBDRC_THRESH_2ND_SHIFT: c_int = 8;

pub const TEGRA210_MBDRC_THRESH_1ST_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_RATIO_1ST
pub const TEGRA210_MBDRC_RATIO_1ST_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_RATIO_2ND
pub const TEGRA210_MBDRC_RATIO_2ND_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_RATIO_3RD
pub const TEGRA210_MBDRC_RATIO_3RD_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_RATIO_4TH
pub const TEGRA210_MBDRC_RATIO_4TH_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_RATIO_5TH
pub const TEGRA210_MBDRC_RATIO_5TH_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_MAKEUP_GAIN
pub const TEGRA210_MBDRC_MAKEUP_GAIN_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_INIT_GAIN
pub const TEGRA210_MBDRC_INIT_GAIN_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_GAIN_ATTACK
pub const TEGRA210_MBDRC_GAIN_ATTACK_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_GAIN_RELEASE
pub const TEGRA210_MBDRC_GAIN_RELEASE_SHIFT: c_int = 0;

// Fields for TEGRA210_MBDRC_FAST_RELEASE
pub const TEGRA210_MBDRC_FAST_RELEASE_SHIFT: c_int = 0;

pub const TEGRA210_MBDRC_RAM_CTRL_RW_READ: c_int = 0;

pub const TEGRA210_MBDRC_RAM_CTRL_RAM_ADDR_MASK: c_uint = 0x1ff;
//
// Order and size of each structure element for following structures should not
// be altered size order of elements and their size are based on PEQ co-eff ram
// and shift ram layout.
//
pub const TEGRA210_MBDRC_THRESHOLD_NUM: c_int = 4;

pub const TEGRA210_MBDRC_MAX_BIQUAD_STAGES: c_int = 8;
// Order of these enums are same as the order of band specific hw registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_mbdrc_band_params {
    pub band: u32,
    pub iir_stages: u32,
    pub in_attack_tc: u32,
    pub in_release_tc: u32,
    pub fast_attack_tc: u32,
    pub in_threshold: [u32; TEGRA210_MBDRC_THRESHOLD_NUM],
    pub out_threshold: [u32; TEGRA210_MBDRC_THRESHOLD_NUM],
    pub ratio: [u32; TEGRA210_MBDRC_RATIO_NUM],
    pub makeup_gain: u32,
    pub gain_init: u32,
    pub gain_attack_tc: u32,
    pub gain_release_tc: u32,
    pub fast_release_tc: u32,
// For biquad_params[][5] order of coeff is b0, b1, a0, a1, a2
    pub 5]: *mut *mut u32 biquad_params[TEGRA210_MBDRC_MAX_BIQUAD_STAGES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_mbdrc_config {
    pub mode: c_uint,
    pub rms_off: c_uint,
    pub peak_rms_mode: c_uint,
    pub filter_structure: c_uint,
    pub shift_ctrl: c_uint,
    pub frame_size: c_uint,
    pub channel_mask: c_uint,
    pub /: *mut *mut unsigned int fa_factor; / Fast attack factor,
    pub /: *mut *mut unsigned int fr_factor; / Fast release factor,
    pub band_params: [tegra210_mbdrc_band_params; MBDRC_NUM_BAND],
}

extern "C" {
    pub fn tegra210_mbdrc_regmap_init(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn tegra210_mbdrc_component_init(cmpnt: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn tegra210_mbdrc_hw_params(cmpnt: *mut snd_soc_component) -> c_int;
}
