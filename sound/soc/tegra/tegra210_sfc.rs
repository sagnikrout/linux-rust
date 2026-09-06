//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_sfc.h
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
// tegra210_sfc.h - Definitions for Tegra210 SFC driver
//
// Copyright (c) 2021-2023 NVIDIA CORPORATION.  All rights reserved.
//
// SFC_RX registers are with respect to XBAR.
// The data comes from XBAR to SFC.
//
pub const TEGRA210_SFC_RX_STATUS: c_uint = 0x0c;
pub const TEGRA210_SFC_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_SFC_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_SFC_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_SFC_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_SFC_RX_CIF_CTRL: c_uint = 0x20;
pub const TEGRA210_SFC_RX_FREQ: c_uint = 0x24;
//
// SFC_TX registers are with respect to XBAR.
// The data goes out of SFC.
//
pub const TEGRA210_SFC_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_SFC_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_SFC_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_SFC_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_SFC_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_SFC_TX_CIF_CTRL: c_uint = 0x60;
pub const TEGRA210_SFC_TX_FREQ: c_uint = 0x64;
// Register offsets from TEGRA210_SFC*_BASE
pub const TEGRA210_SFC_ENABLE: c_uint = 0x80;
pub const TEGRA210_SFC_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_SFC_CG: c_uint = 0x88;
pub const TEGRA210_SFC_STATUS: c_uint = 0x8c;
pub const TEGRA210_SFC_INT_STATUS: c_uint = 0x90;
pub const TEGRA210_SFC_COEF_RAM: c_uint = 0xbc;
pub const TEGRA210_SFC_CFG_RAM_CTRL: c_uint = 0xc0;
pub const TEGRA210_SFC_CFG_RAM_DATA: c_uint = 0xc4;
// Fields in TEGRA210_SFC_ENABLE
pub const TEGRA210_SFC_EN_SHIFT: c_int = 0;

pub const TEGRA210_SFC_NUM_RATES: c_int = 13;
// Fields in TEGRA210_SFC_COEF_RAM

// Coefficients
pub const TEGRA210_SFC_COEF_RAM_DEPTH: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra210_sfc_path {
    SFC_RX_PATH,
    SFC_TX_PATH,
    SFC_PATHS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_sfc {
    pub mono_to_stereo: [c_uint; SFC_PATHS],
    pub stereo_to_mono: [c_uint; SFC_PATHS],
    pub srate_out: c_uint,
    pub srate_in: c_uint,
    pub regmap: *mut regmap,
}
