//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_ahub.h
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
// tegra210_ahub.h - TEGRA210 AHUB
//
// Copyright (c) 2020-2025, NVIDIA CORPORATION.  All rights reserved.
//
// Tegra210 specific
pub const TEGRA210_XBAR_PART1_RX: c_uint = 0x200;
pub const TEGRA210_XBAR_PART2_RX: c_uint = 0x400;
pub const TEGRA210_XBAR_RX_STRIDE: c_uint = 0x4;
pub const TEGRA210_XBAR_AUDIO_RX_COUNT: c_int = 90;
pub const TEGRA210_XBAR_REG_MASK_0: c_uint = 0xf1f03ff;
pub const TEGRA210_XBAR_REG_MASK_1: c_uint = 0x3f30031f;
pub const TEGRA210_XBAR_REG_MASK_2: c_uint = 0xff1cf313;
pub const TEGRA210_XBAR_REG_MASK_3: c_uint = 0x0;
pub const TEGRA210_XBAR_UPDATE_MAX_REG: c_int = 3;
// Tegra186 specific
pub const TEGRA186_XBAR_PART3_RX: c_uint = 0x600;
pub const TEGRA186_XBAR_AUDIO_RX_COUNT: c_int = 115;
pub const TEGRA186_XBAR_REG_MASK_0: c_uint = 0xf3fffff;
pub const TEGRA186_XBAR_REG_MASK_1: c_uint = 0x3f310f1f;
pub const TEGRA186_XBAR_REG_MASK_2: c_uint = 0xff3cf311;
pub const TEGRA186_XBAR_REG_MASK_3: c_uint = 0x3f0f00ff;
pub const TEGRA186_XBAR_UPDATE_MAX_REG: c_int = 4;
// Tegra264 specific
pub const TEGRA264_XBAR_PART1_RX: c_uint = 0x1000;
pub const TEGRA264_XBAR_PART2_RX: c_uint = 0x2000;
pub const TEGRA264_XBAR_PART3_RX: c_uint = 0x3000;
pub const TEGRA264_XBAR_PART4_RX: c_uint = 0x4000;
pub const TEGRA264_XBAR_PART0_ADX6_RX1: c_uint = 0x224;

pub const TEGRA264_XBAR_REG_MASK_0: c_uint = 0xfffffff;
pub const TEGRA264_XBAR_REG_MASK_1: c_uint = 0x3f013f1f;
pub const TEGRA264_XBAR_REG_MASK_2: c_uint = 0xff3c0301;
pub const TEGRA264_XBAR_REG_MASK_3: c_uint = 0x3f00ffff;
pub const TEGRA264_XBAR_REG_MASK_4: c_uint = 0x7fff9f;
pub const TEGRA264_XBAR_UPDATE_MAX_REG: c_int = 5;
pub const TEGRA264_AXBAR_ADMAIF_RX1: c_uint = 0x0;
pub const TEGRA264_AXBAR_SFC4_RX1: c_uint = 0x6c;
pub const TEGRA264_AXBAR_MIXER1_RX1: c_uint = 0x80;
pub const TEGRA264_AXBAR_MIXER1_RX10: c_uint = 0xa4;
pub const TEGRA264_AXBAR_DSPK1_RX1: c_uint = 0xc0;
pub const TEGRA264_AXBAR_OPE1_RX1: c_uint = 0x100;
pub const TEGRA264_AXBAR_MVC1_RX1: c_uint = 0x110;
pub const TEGRA264_AXBAR_MVC2_RX1: c_uint = 0x114;
pub const TEGRA264_AXBAR_AMX1_RX1: c_uint = 0x120;
pub const TEGRA264_AXBAR_AMX3_RX4: c_uint = 0x14c;
pub const TEGRA264_AXBAR_ADX1_RX1: c_uint = 0x160;
pub const TEGRA264_AXBAR_ASRC1_RX7: c_uint = 0x1a8;
pub const TEGRA264_AXBAR_ADMAIF_RX21: c_uint = 0x1d0;
pub const TEGRA264_AXBAR_ADX6_RX1: c_uint = 0x224;

// AXBAR register offsets
pub const TEGRA186_AXBAR_PART_0_AMX1_RX1_0: c_uint = 0x120;
pub const TEGRA186_AXBAR_PART_0_AMX3_RX4_0: c_uint = 0x14c;
pub const TEGRA186_AXBAR_PART_0_ASRC1_RX7_0: c_uint = 0x1a8;
pub const TEGRA186_AXBAR_PART_0_DSPK1_RX1_0: c_uint = 0xc0;
pub const TEGRA186_AXBAR_PART_0_DSPK2_RX1_0: c_uint = 0xc4;
pub const TEGRA186_AXBAR_PART_0_I2S6_RX1_0: c_uint = 0x54;
pub const TEGRA186_AXBAR_PART_0_MVC1_RX1_0: c_uint = 0x110;
pub const TEGRA186_AXBAR_PART_0_MVC2_RX1_0: c_uint = 0x114;
pub const TEGRA210_AXBAR_PART_0_ADMAIF_RX10_0: c_uint = 0x24;
pub const TEGRA210_AXBAR_PART_0_ADMAIF_RX1_0: c_uint = 0x0;
pub const TEGRA210_AXBAR_PART_0_ADX1_RX1_0: c_uint = 0x160;
pub const TEGRA210_AXBAR_PART_0_ADX2_RX1_0: c_uint = 0x164;
pub const TEGRA210_AXBAR_PART_0_AFC1_RX1_0: c_uint = 0xd0;
pub const TEGRA210_AXBAR_PART_0_AFC6_RX1_0: c_uint = 0xe4;
pub const TEGRA210_AXBAR_PART_0_AMX1_RX1_0: c_uint = 0x140;
pub const TEGRA210_AXBAR_PART_0_I2S1_RX1_0: c_uint = 0x40;
pub const TEGRA210_AXBAR_PART_0_I2S5_RX1_0: c_uint = 0x50;
pub const TEGRA210_AXBAR_PART_0_MIXER1_RX10_0: c_uint = 0xa4;
pub const TEGRA210_AXBAR_PART_0_MIXER1_RX1_0: c_uint = 0x80;
pub const TEGRA210_AXBAR_PART_0_MVC1_RX1_0: c_uint = 0x120;
pub const TEGRA210_AXBAR_PART_0_MVC2_RX1_0: c_uint = 0x124;
pub const TEGRA210_AXBAR_PART_0_OPE1_RX1_0: c_uint = 0x100;
pub const TEGRA210_AXBAR_PART_0_OPE2_RX1_0: c_uint = 0x104;
pub const TEGRA210_AXBAR_PART_0_SFC1_RX1_0: c_uint = 0x60;
pub const TEGRA210_AXBAR_PART_0_SFC4_RX1_0: c_uint = 0x6c;
pub const TEGRA210_AXBAR_PART_0_SPDIF1_RX1_0: c_uint = 0xc0;
pub const TEGRA210_AXBAR_PART_0_SPDIF1_RX2_0: c_uint = 0xc4;
pub const TEGRA210_AXBAR_PART_0_SPKPROT1_RX1_0: c_uint = 0x110;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahub_soc_data {
    pub regmap_config: *const regmap_config,
    pub cmpnt_drv: *const snd_soc_component_driver,
    pub dai_drv: *mut snd_soc_dai_driver,
    pub mask: [c_uint; TEGRA_XBAR_UPDATE_MAX_REG],
    pub reg_count: c_uint,
    pub num_dais: c_uint,
    pub xbar_part_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ahub {
    pub soc_data: *const tegra_ahub_soc_data,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
}
