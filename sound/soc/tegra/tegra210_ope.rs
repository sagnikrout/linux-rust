//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_ope.h
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
// tegra210_ope.h - Definitions for Tegra210 OPE driver
//
// Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
//

//
// OPE_RX registers are with respect to XBAR.
// The data comes from XBAR to OPE
//
pub const TEGRA210_OPE_RX_STATUS: c_uint = 0xc;
pub const TEGRA210_OPE_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_OPE_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_OPE_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_OPE_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_OPE_RX_CIF_CTRL: c_uint = 0x20;
//
// OPE_TX registers are with respect to XBAR.
// The data goes out from OPE to XBAR
//
pub const TEGRA210_OPE_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_OPE_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_OPE_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_OPE_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_OPE_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_OPE_TX_CIF_CTRL: c_uint = 0x60;
// OPE Gloabal registers
pub const TEGRA210_OPE_ENABLE: c_uint = 0x80;
pub const TEGRA210_OPE_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_OPE_CG: c_uint = 0x88;
pub const TEGRA210_OPE_STATUS: c_uint = 0x8c;
pub const TEGRA210_OPE_INT_STATUS: c_uint = 0x90;
pub const TEGRA210_OPE_DIR: c_uint = 0x94;
// Fields for TEGRA210_OPE_ENABLE
pub const TEGRA210_OPE_EN_SHIFT: c_int = 0;

// Fields for TEGRA210_OPE_SOFT_RESET
pub const TEGRA210_OPE_SOFT_RESET_SHIFT: c_int = 0;

pub const TEGRA210_OPE_DIR_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_ope {
    pub regmap: *mut regmap,
    pub peq_regmap: *mut regmap,
    pub mbdrc_regmap: *mut regmap,
    pub peq_biquad_gains: [u32; TEGRA210_PEQ_GAIN_PARAM_SIZE_PER_CH],
    pub peq_biquad_shifts: [u32; TEGRA210_PEQ_SHIFT_PARAM_SIZE_PER_CH],
    pub data_dir: c_uint,
}

// Extension of soc_bytes structure defined in sound/soc.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_soc_bytes {
    pub soc: soc_bytes,
    pub /: *mut *mut u32 shift; / Used as offset for AHUB RAM related programing,
}

// Utility structures for using mixer control of type snd_soc_bytes

