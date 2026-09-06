//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_amx.h
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
// SPDX-FileCopyrightText: Copyright (c) 2021-2025 NVIDIA CORPORATION. All rights reserved.
//
// tegra210_amx.h - Definitions for Tegra210 AMX driver
//

// Register offsets from TEGRA210_AMX*_BASE
pub const TEGRA210_AMX_RX_STATUS: c_uint = 0x0c;
pub const TEGRA210_AMX_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_AMX_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_AMX_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_AMX_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_AMX_RX1_CIF_CTRL: c_uint = 0x20;
pub const TEGRA210_AMX_RX2_CIF_CTRL: c_uint = 0x24;
pub const TEGRA210_AMX_RX3_CIF_CTRL: c_uint = 0x28;
pub const TEGRA210_AMX_RX4_CIF_CTRL: c_uint = 0x2c;
pub const TEGRA210_AMX_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_AMX_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_AMX_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_AMX_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_AMX_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_AMX_TX_CIF_CTRL: c_uint = 0x60;
pub const TEGRA210_AMX_ENABLE: c_uint = 0x80;
pub const TEGRA210_AMX_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_AMX_CG: c_uint = 0x88;
pub const TEGRA210_AMX_STATUS: c_uint = 0x8c;
pub const TEGRA210_AMX_INT_STATUS: c_uint = 0x90;
pub const TEGRA210_AMX_CTRL: c_uint = 0xa4;
pub const TEGRA210_AMX_OUT_BYTE_EN0: c_uint = 0xa8;
pub const TEGRA210_AMX_CYA: c_uint = 0xb0;
pub const TEGRA210_AMX_CFG_RAM_CTRL: c_uint = 0xb8;
pub const TEGRA210_AMX_CFG_RAM_DATA: c_uint = 0xbc;
pub const TEGRA194_AMX_RX1_FRAME_PERIOD: c_uint = 0xc0;
pub const TEGRA194_AMX_RX4_FRAME_PERIOD: c_uint = 0xcc;
pub const TEGRA194_AMX_RX4_LAST_FRAME_PERIOD: c_uint = 0xdc;
pub const TEGRA264_AMX_STREAMS_AUTO_DISABLE: c_uint = 0xb8;
pub const TEGRA264_AMX_CFG_RAM_CTRL: c_uint = 0xc0;
pub const TEGRA264_AMX_CFG_RAM_DATA: c_uint = 0xc4;
pub const TEGRA264_AMX_RX1_FRAME_PERIOD: c_uint = 0xc8;
pub const TEGRA264_AMX_RX4_FRAME_PERIOD: c_uint = 0xd4;
pub const TEGRA264_AMX_RX4_LAST_FRAME_PERIOD: c_uint = 0xe4;
// Fields in TEGRA210_AMX_ENABLE
pub const TEGRA210_AMX_ENABLE_SHIFT: c_int = 0;
// Fields in TEGRA210_AMX_CTRL
pub const TEGRA210_AMX_CTRL_MSTR_RX_NUM_SHIFT: c_int = 14;

pub const TEGRA210_AMX_CTRL_RX_DEP_SHIFT: c_int = 12;

// Fields in TEGRA210_AMX_CFG_RAM_CTRL
pub const TEGRA210_AMX_CFG_RAM_CTRL_RW_SHIFT: c_int = 14;

pub const TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT: c_int = 13;

pub const TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT: c_int = 12;

pub const TEGRA210_AMX_CFG_CTRL_RAM_ADDR_SHIFT: c_int = 0;
// Fields in TEGRA210_AMX_SOFT_RESET
pub const TEGRA210_AMX_SOFT_RESET_SOFT_EN: c_int = 1;

pub const TEGRA210_AMX_AUDIOCIF_CH_STRIDE: c_int = 4;
pub const TEGRA_AMX_SLOTS_PER_WORD: c_int = 4;
pub const TEGRA210_AMX_RAM_DEPTH: c_int = 16;
pub const TEGRA210_AMX_MAP_STREAM_NUM_SHIFT: c_int = 6;
pub const TEGRA210_AMX_MAP_WORD_NUM_SHIFT: c_int = 2;
pub const TEGRA210_AMX_MAP_BYTE_NUM_SHIFT: c_int = 0;
pub const TEGRA210_AMX_BYTE_MASK_COUNT: c_int = 2;
pub const TEGRA210_AMX_MAX_CHANNEL: c_int = 16;
pub const TEGRA210_AMX_AUTO_DISABLE_OFFSET: c_int = 0;
pub const TEGRA264_AMX_RAM_DEPTH: c_int = 32;
pub const TEGRA264_AMX_BYTE_MASK_COUNT: c_int = 4;
pub const TEGRA264_AMX_MAX_CHANNEL: c_int = 32;
pub const TEGRA264_AMX_AUTO_DISABLE_OFFSET: c_int = 8;
pub const TEGRA_AMX_OUT_DAI_ID: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_amx_soc_data {
    pub regmap_conf: *const regmap_config,
    pub auto_disable: bool,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub max_ch: c_uint,
    pub ram_depth: c_uint,
    pub byte_mask_size: c_uint,
    pub reg_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_amx {
    pub soc_data: *const tegra210_amx_soc_data,
    pub byte_mask: *mut c_uint,
    pub map: *mut u16,
    pub regmap: *mut regmap,
}
