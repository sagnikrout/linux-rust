//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_peq.h
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
// tegra210_peq.h - Definitions for Tegra210 PEQ driver
//
// Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
//

// Register offsets from PEQ base
pub const TEGRA210_PEQ_SOFT_RESET: c_uint = 0x0;
pub const TEGRA210_PEQ_CG: c_uint = 0x4;
pub const TEGRA210_PEQ_STATUS: c_uint = 0x8;
pub const TEGRA210_PEQ_CFG: c_uint = 0xc;
pub const TEGRA210_PEQ_CFG_RAM_CTRL: c_uint = 0x10;
pub const TEGRA210_PEQ_CFG_RAM_DATA: c_uint = 0x14;
pub const TEGRA210_PEQ_CFG_RAM_SHIFT_CTRL: c_uint = 0x18;
pub const TEGRA210_PEQ_CFG_RAM_SHIFT_DATA: c_uint = 0x1c;
// Fields in TEGRA210_PEQ_CFG
pub const TEGRA210_PEQ_CFG_BIQUAD_STAGES_SHIFT: c_int = 2;

pub const TEGRA210_PEQ_CFG_MODE_SHIFT: c_int = 0;

pub const TEGRA210_PEQ_RAM_CTRL_RW_READ: c_int = 0;

pub const TEGRA210_PEQ_RAM_CTRL_RAM_ADDR_MASK: c_uint = 0x1ff;
// PEQ register definition ends here
pub const TEGRA210_PEQ_MAX_BIQUAD_STAGES: c_int = 12;
pub const TEGRA210_PEQ_MAX_CHANNELS: c_int = 8;
pub const TEGRA210_PEQ_BIQUAD_INIT_STAGE: c_int = 5;

extern "C" {
    pub fn tegra210_peq_regmap_init(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn tegra210_peq_component_init(cmpnt: *mut snd_soc_component) -> c_int;
}
