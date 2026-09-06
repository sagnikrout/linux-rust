//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_mixer.h
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
// tegra210_mixer.h - Definitions for Tegra210 MIXER driver
//
// Copyright (c) 2021, NVIDIA CORPORATION.  All rights reserved.
//
// XBAR_RX related MIXER offsets
pub const TEGRA210_MIXER_RX1_SOFT_RESET: c_uint = 0x04;
pub const TEGRA210_MIXER_RX1_STATUS: c_uint = 0x10;
pub const TEGRA210_MIXER_RX1_CIF_CTRL: c_uint = 0x24;
pub const TEGRA210_MIXER_RX1_CTRL: c_uint = 0x28;
pub const TEGRA210_MIXER_RX1_PEAK_CTRL: c_uint = 0x2c;
pub const TEGRA210_MIXER_RX1_SAMPLE_COUNT: c_uint = 0x30;
// XBAR_TX related MIXER offsets
pub const TEGRA210_MIXER_TX1_ENABLE: c_uint = 0x280;
pub const TEGRA210_MIXER_TX1_SOFT_RESET: c_uint = 0x284;
pub const TEGRA210_MIXER_TX1_STATUS: c_uint = 0x290;
pub const TEGRA210_MIXER_TX1_INT_STATUS: c_uint = 0x294;
pub const TEGRA210_MIXER_TX1_INT_MASK: c_uint = 0x298;
pub const TEGRA210_MIXER_TX1_INT_SET: c_uint = 0x29c;
pub const TEGRA210_MIXER_TX1_INT_CLEAR: c_uint = 0x2a0;
pub const TEGRA210_MIXER_TX1_CIF_CTRL: c_uint = 0x2a4;
pub const TEGRA210_MIXER_TX1_ADDER_CONFIG: c_uint = 0x2a8;
// MIXER related offsets
pub const TEGRA210_MIXER_ENABLE: c_uint = 0x400;
pub const TEGRA210_MIXER_SOFT_RESET: c_uint = 0x404;
pub const TEGRA210_MIXER_CG: c_uint = 0x408;
pub const TEGRA210_MIXER_STATUS: c_uint = 0x410;
pub const TEGRA210_MIXER_INT_STATUS: c_uint = 0x414;
pub const TEGRA210_MIXER_GAIN_CFG_RAM_CTRL: c_uint = 0x42c;
pub const TEGRA210_MIXER_GAIN_CFG_RAM_DATA: c_uint = 0x430;
pub const TEGRA210_MIXER_PEAKM_RAM_CTRL: c_uint = 0x434;
pub const TEGRA210_MIXER_PEAKM_RAM_DATA: c_uint = 0x438;
pub const TEGRA210_MIXER_CTRL: c_uint = 0x43c;

// Fields in TEGRA210_MIXER_ENABLE
pub const TEGRA210_MIXER_ENABLE_SHIFT: c_int = 0;

// Fields in TEGRA210_MIXER_GAIN_CFG_RAM_CTRL
pub const TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_0: c_uint = 0x0;
pub const TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_STRIDE: c_uint = 0x10;
pub const TEGRA210_MIXER_GAIN_CFG_RAM_RW_SHIFT: c_int = 14;

pub const TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_INIT_EN_SHIFT: c_int = 13;

pub const TEGRA210_MIXER_GAIN_CFG_RAM_SEQ_ACCESS_EN_SHIFT: c_int = 12;

pub const TEGRA210_MIXER_GAIN_CFG_RAM_ADDR_SHIFT: c_int = 0;

pub const TEGRA210_MIXER_REG_STRIDE: c_uint = 0x40;
pub const TEGRA210_MIXER_RX_MAX: c_int = 10;

pub const TEGRA210_MIXER_TX_MAX: c_int = 5;

pub const TEGRA210_MIXER_SAMPLE_COUNT_SHIFT: c_int = 24;

pub const REG_CFG_DONE_TRIGGER: c_uint = 0xf;
pub const VAL_CFG_DONE_TRIGGER: c_uint = 0x1;
pub const NUM_GAIN_POLY_COEFFS: c_int = 9;
pub const TEGRA210_MIXER_GAIN_MAX: c_uint = 0x20000;
pub const TEGRA210_MIXER_PRESCALAR: c_int = 6;

pub const TEGRA210_MIXER_FADE_DURATION_MAX: c_uint = 0x7fffffff;
pub const TEGRA210_MIXER_FADE_IDLE: c_int = 0;
pub const TEGRA210_MIXER_FADE_ACTIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_mixer_gain_params {
    pub poly_coeff: [c_int; NUM_GAIN_POLY_COEFFS],
    pub gain_value: c_int,
    pub duration: [c_int; NUM_DURATION_PARMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_mixer {
    pub gain_value: [c_int; TEGRA210_MIXER_RX_MAX],
    pub fade_gain: [c_int; TEGRA210_MIXER_RX_MAX],
    pub duration: [u32; TEGRA210_MIXER_RX_MAX],
    pub in_fade: [bool; TEGRA210_MIXER_RX_MAX],
    pub fade_pending: [bool; TEGRA210_MIXER_RX_MAX],
    pub regmap: *mut regmap,
}
