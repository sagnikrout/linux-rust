//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_dmic.h
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
// tegra210_dmic.h - Definitions for Tegra210 DMIC driver
//
// Copyright (c) 2020 NVIDIA CORPORATION.  All rights reserved.
//
// Register offsets from DMIC BASE
pub const TEGRA210_DMIC_TX_STATUS: c_uint = 0x0c;
pub const TEGRA210_DMIC_TX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_DMIC_TX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_DMIC_TX_INT_SET: c_uint = 0x18;
pub const TEGRA210_DMIC_TX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_DMIC_TX_CIF_CTRL: c_uint = 0x20;
pub const TEGRA210_DMIC_ENABLE: c_uint = 0x40;
pub const TEGRA210_DMIC_SOFT_RESET: c_uint = 0x44;
pub const TEGRA210_DMIC_CG: c_uint = 0x48;
pub const TEGRA210_DMIC_STATUS: c_uint = 0x4c;
pub const TEGRA210_DMIC_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_DMIC_CTRL: c_uint = 0x64;
pub const TEGRA210_DMIC_DBG_CTRL: c_uint = 0x70;
pub const TEGRA210_DMIC_DCR_BIQUAD_0_COEF_4: c_uint = 0x88;
pub const TEGRA210_DMIC_LP_FILTER_GAIN: c_uint = 0x8c;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_0: c_uint = 0x90;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_1: c_uint = 0x94;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_2: c_uint = 0x98;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_3: c_uint = 0x9c;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_4: c_uint = 0xa0;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_0: c_uint = 0xa4;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_1: c_uint = 0xa8;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_2: c_uint = 0xac;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_3: c_uint = 0xb0;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_4: c_uint = 0xb4;
// Fields in TEGRA210_DMIC_CTRL
pub const CH_SEL_SHIFT: c_int = 8;

pub const LRSEL_POL_SHIFT: c_int = 4;

pub const OSR_SHIFT: c_int = 0;

pub const DMIC_OSR_FACTOR: c_int = 64;
pub const DEFAULT_GAIN_Q23: c_uint = 0x800000;
// Max boost gain factor used for mixer control
pub const MAX_BOOST_GAIN: c_int = 25599;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_ch_select {
    DMIC_CH_SELECT_LEFT,
    DMIC_CH_SELECT_RIGHT,
    DMIC_CH_SELECT_STEREO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_osr {
    DMIC_OSR_64,
    DMIC_OSR_128,
    DMIC_OSR_256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_lrsel {
    DMIC_LRSEL_LEFT,
    DMIC_LRSEL_RIGHT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_dmic {
    pub clk_dmic: *mut clk,
    pub regmap: *mut regmap,
    pub mono_to_stereo: c_uint,
    pub stereo_to_mono: c_uint,
    pub boost_gain: c_uint,
    pub ch_select: c_uint,
    pub osr_val: c_uint,
    pub lrsel: c_uint,
}
