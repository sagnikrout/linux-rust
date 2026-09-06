//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra186_asrc.h
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
// SPDX-FileCopyrightText: Copyright (c) 2022-2024 NVIDIA CORPORATION. All rights reserved.
// tegra186_asrc.h - Definitions for Tegra186 ASRC driver
//
// ASRC stream related offset
pub const TEGRA186_ASRC_CFG: c_uint = 0x0;
pub const TEGRA186_ASRC_RATIO_INT_PART: c_uint = 0x4;
pub const TEGRA186_ASRC_RATIO_FRAC_PART: c_uint = 0x8;
pub const TEGRA186_ASRC_RATIO_LOCK_STATUS: c_uint = 0xc;
pub const TEGRA186_ASRC_MUTE_UNMUTE_DURATION: c_uint = 0x10;
pub const TEGRA186_ASRC_TX_THRESHOLD: c_uint = 0x14;
pub const TEGRA186_ASRC_RX_THRESHOLD: c_uint = 0x18;
pub const TEGRA186_ASRC_RATIO_COMP: c_uint = 0x1c;
pub const TEGRA186_ASRC_RX_STATUS: c_uint = 0x20;
pub const TEGRA186_ASRC_RX_CIF_CTRL: c_uint = 0x24;
pub const TEGRA186_ASRC_TX_STATUS: c_uint = 0x2c;
pub const TEGRA186_ASRC_TX_CIF_CTRL: c_uint = 0x30;
pub const TEGRA186_ASRC_ENABLE: c_uint = 0x38;
pub const TEGRA186_ASRC_SOFT_RESET: c_uint = 0x3c;
pub const TEGRA186_ASRC_STATUS: c_uint = 0x4c;
pub const TEGRA186_ASRC_STATEBUF_ADDR: c_uint = 0x5c;
pub const TEGRA186_ASRC_STATEBUF_CFG: c_uint = 0x60;
pub const TEGRA186_ASRC_INSAMPLEBUF_ADDR: c_uint = 0x64;
pub const TEGRA186_ASRC_INSAMPLEBUF_CFG: c_uint = 0x68;
pub const TEGRA186_ASRC_OUTSAMPLEBUF_ADDR: c_uint = 0x6c;
pub const TEGRA186_ASRC_OUTSAMPLEBUF_CFG: c_uint = 0x70;
// ASRC Global registers offset
pub const TEGRA186_ASRC_GLOBAL_ENB: c_uint = 0x2f4;
pub const TEGRA186_ASRC_GLOBAL_SOFT_RESET: c_uint = 0x2f8;
pub const TEGRA186_ASRC_GLOBAL_CG: c_uint = 0x2fc;
pub const TEGRA186_ASRC_GLOBAL_CFG: c_uint = 0x300;
pub const TEGRA186_ASRC_GLOBAL_SCRATCH_ADDR: c_uint = 0x304;
pub const TEGRA186_ASRC_GLOBAL_SCRATCH_CFG: c_uint = 0x308;
pub const TEGRA186_ASRC_RATIO_UPD_RX_CIF_CTRL: c_uint = 0x30c;
pub const TEGRA186_ASRC_RATIO_UPD_RX_STATUS: c_uint = 0x310;
pub const TEGRA186_ASRC_GLOBAL_STATUS: c_uint = 0x314;
pub const TEGRA186_ASRC_GLOBAL_STREAM_ENABLE_STATUS: c_uint = 0x318;
pub const TEGRA186_ASRC_GLOBAL_INT_STATUS: c_uint = 0x324;
pub const TEGRA186_ASRC_GLOBAL_INT_MASK: c_uint = 0x328;
pub const TEGRA186_ASRC_GLOBAL_INT_SET: c_uint = 0x32c;
pub const TEGRA186_ASRC_GLOBAL_INT_CLEAR: c_uint = 0x330;
pub const TEGRA186_ASRC_GLOBAL_TRANSFER_ERROR_LOG: c_uint = 0x334;
pub const TEGRA186_ASRC_GLOBAL_APR_CTRL: c_uint = 0x1000;
pub const TEGRA186_ASRC_GLOBAL_APR_CTRL_ACCESS_CTRL: c_uint = 0x1004;
pub const TEGRA186_ASRC_GLOBAL_DISARM_APR: c_uint = 0x1008;
pub const TEGRA186_ASRC_GLOBAL_DISARM_APR_ACCESS_CTRL: c_uint = 0x100c;
pub const TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS: c_uint = 0x1010;
pub const TEGRA186_ASRC_GLOBAL_RATIO_WR_ACCESS_CTRL: c_uint = 0x1014;
pub const TEGRA186_ASRC_CYA: c_uint = 0x1018;
pub const TEGRA186_ASRC_STREAM_DEFAULT_HW_COMP_BIAS_VALUE: c_uint = 0xaaaa;
pub const TEGRA186_ASRC_STREAM_DEFAULT_INPUT_HW_COMP_THRESH_CFG: c_uint = 0x00201002;
pub const TEGRA186_ASRC_STREAM_DEFAULT_OUTPUT_HW_COMP_THRESH_CFG: c_uint = 0x00201002;
pub const TEGRA186_ASRC_GLOBAL_CFG_FRAC_28BIT_PRECISION: c_int = 0;
pub const TEGRA186_ASRC_GLOBAL_CFG_FRAC_32BIT_PRECISION: c_int = 1;
pub const TEGRA186_ASRC_STREAM_ENABLE_HW_RATIO_COMP_SHIFT: c_int = 31;

pub const TEGRA186_ASRC_STREAM_RATIO_TYPE_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_STREAM_EN_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_GLOBAL_EN_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_STREAM_STATEBUF_CFG_SIZE_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_STREAM_INSAMPLEBUF_CFG_SIZE_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_STREAM_OUTSAMPLEBUF_CFG_SIZE_SHIFT: c_int = 0;

pub const TEGRA186_ASRC_STREAM_RATIO_INT_PART_MASK: c_uint = 0x1f;
pub const TEGRA186_ASRC_STREAM_RATIO_FRAC_PART_MASK: c_uint = 0xffffffff;
pub const TEGRA186_ASRC_STREAM_STRIDE: c_uint = 0x80;
pub const TEGRA186_ASRC_STREAM_MAX: c_uint = 0x6;
pub const TEGRA186_ASRC_STREAM_LIMIT: c_uint = 0x2f0;
pub const TEGRA186_ASRC_RATIO_SOURCE_ARAD: c_uint = 0x0;
pub const TEGRA186_ASRC_RATIO_SOURCE_SW: c_uint = 0x1;
pub const TEGRA186_ASRC_ARAM_START_ADDR: c_uint = 0x3f800000;
pub const TEGRA264_ASRC_ARAM_START_ADDR: c_uint = 0x8a080000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_asrc_lane {
    pub int_part: c_uint,
    pub frac_part: c_uint,
    pub ratio_source: c_uint,
    pub hwcomp_disable: c_uint,
    pub input_thresh: c_uint,
    pub output_thresh: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_asrc_soc_data {
    pub aram_start_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_asrc {
    pub soc_data: *const tegra_asrc_soc_data,
    pub lane: [tegra186_asrc_lane; TEGRA186_ASRC_STREAM_MAX],
    pub regmap: *mut regmap,
}
