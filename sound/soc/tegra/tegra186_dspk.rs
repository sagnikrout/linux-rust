//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra186_dspk.h
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
// tegra186_dspk.h - Definitions for Tegra186 DSPK driver
//
// Copyright (c) 2020 NVIDIA CORPORATION. All rights reserved.
//
// Register offsets from DSPK BASE
pub const TEGRA186_DSPK_RX_STATUS: c_uint = 0x0c;
pub const TEGRA186_DSPK_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA186_DSPK_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA186_DSPK_RX_INT_SET: c_uint = 0x18;
pub const TEGRA186_DSPK_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA186_DSPK_RX_CIF_CTRL: c_uint = 0x20;
pub const TEGRA186_DSPK_ENABLE: c_uint = 0x40;
pub const TEGRA186_DSPK_SOFT_RESET: c_uint = 0x44;
pub const TEGRA186_DSPK_CG: c_uint = 0x48;
pub const TEGRA186_DSPK_STATUS: c_uint = 0x4c;
pub const TEGRA186_DSPK_INT_STATUS: c_uint = 0x50;
pub const TEGRA186_DSPK_CORE_CTRL: c_uint = 0x60;
pub const TEGRA186_DSPK_CODEC_CTRL: c_uint = 0x64;
// DSPK CORE CONTROL fields
pub const CH_SEL_SHIFT: c_int = 8;

pub const DSPK_OSR_SHIFT: c_int = 4;

pub const LRSEL_POL_SHIFT: c_int = 0;

pub const TEGRA186_DSPK_RX_FIFO_DEPTH: c_int = 64;
pub const DSPK_OSR_FACTOR: c_int = 32;
// DSPK interface clock ratio
pub const DSPK_CLK_RATIO: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_osr {
    DSPK_OSR_32,
    DSPK_OSR_64,
    DSPK_OSR_128,
    DSPK_OSR_256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_ch_sel {
    DSPK_CH_SELECT_LEFT,
    DSPK_CH_SELECT_RIGHT,
    DSPK_CH_SELECT_STEREO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_lrsel {
    DSPK_LRSEL_LEFT,
    DSPK_LRSEL_RIGHT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_dspk {
    pub rx_fifo_th: c_uint,
    pub osr_val: c_uint,
    pub lrsel: c_uint,
    pub ch_sel: c_uint,
    pub mono_to_stereo: c_uint,
    pub stereo_to_mono: c_uint,
    pub clk_dspk: *mut clk,
    pub regmap: *mut regmap,
}
