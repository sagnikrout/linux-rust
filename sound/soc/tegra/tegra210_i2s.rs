//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_i2s.h
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
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_i2s.h - Definitions for Tegra210 I2S driver
//
// Register offsets from I2S*_BASE
pub const TEGRA210_I2S_RX_ENABLE: c_uint = 0x0;
pub const TEGRA210_I2S_RX_SOFT_RESET: c_uint = 0x4;
pub const TEGRA210_I2S_RX_STATUS: c_uint = 0x0c;
pub const TEGRA210_I2S_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_I2S_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_I2S_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_I2S_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_I2S_RX_CIF_CTRL: c_uint = 0x20;
pub const TEGRA210_I2S_RX_CTRL: c_uint = 0x24;
pub const TEGRA210_I2S_RX_SLOT_CTRL: c_uint = 0x28;
pub const TEGRA210_I2S_RX_CLK_TRIM: c_uint = 0x2c;
pub const TEGRA210_I2S_RX_CYA: c_uint = 0x30;
pub const TEGRA210_I2S_RX_CIF_FIFO_STATUS: c_uint = 0x34;
pub const TEGRA210_I2S_TX_ENABLE: c_uint = 0x40;
pub const TEGRA210_I2S_TX_SOFT_RESET: c_uint = 0x44;
pub const TEGRA210_I2S_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_I2S_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_I2S_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_I2S_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_I2S_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_I2S_TX_CIF_CTRL: c_uint = 0x60;
pub const TEGRA210_I2S_TX_CTRL: c_uint = 0x64;
pub const TEGRA210_I2S_TX_SLOT_CTRL: c_uint = 0x68;
pub const TEGRA210_I2S_TX_CLK_TRIM: c_uint = 0x6c;
pub const TEGRA210_I2S_TX_CYA: c_uint = 0x70;
pub const TEGRA210_I2S_TX_CIF_FIFO_STATUS: c_uint = 0x74;
pub const TEGRA210_I2S_ENABLE: c_uint = 0x80;
pub const TEGRA210_I2S_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_I2S_CG: c_uint = 0x88;
pub const TEGRA210_I2S_STATUS: c_uint = 0x8c;
pub const TEGRA210_I2S_INT_STATUS: c_uint = 0x90;
pub const TEGRA210_I2S_CTRL: c_uint = 0xa0;
pub const TEGRA210_I2S_TIMING: c_uint = 0xa4;
pub const TEGRA210_I2S_SLOT_CTRL: c_uint = 0xa8;
pub const TEGRA210_I2S_CLK_TRIM: c_uint = 0xac;
pub const TEGRA210_I2S_CYA: c_uint = 0xb0;
// T264 specific registers
pub const TEGRA264_I2S_RX_FIFO_WR_ACCESS_MODE: c_uint = 0x30;
pub const TEGRA264_I2S_RX_CYA: c_uint = 0x3c;
pub const TEGRA264_I2S_RX_CIF_FIFO_STATUS: c_uint = 0x40;
pub const TEGRA264_I2S_TX_ENABLE: c_uint = 0x80;
pub const TEGRA264_I2S_TX_SOFT_RESET: c_uint = 0x84;
pub const TEGRA264_I2S_TX_STATUS: c_uint = 0x8c;
pub const TEGRA264_I2S_TX_INT_STATUS: c_uint = 0x90;
pub const TEGRA264_I2S_TX_INT_MASK: c_uint = 0x94;
pub const TEGRA264_I2S_TX_CIF_CTRL: c_uint = 0xa0;
pub const TEGRA264_I2S_TX_FIFO_RD_ACCESS_MODE: c_uint = 0xb0;
pub const TEGRA264_I2S_TX_FIFO_RD_DATA: c_uint = 0xb4;
pub const TEGRA264_I2S_TX_FIFO_THRESHOLD: c_uint = 0xb8;
pub const TEGRA264_I2S_TX_CYA: c_uint = 0xbc;
pub const TEGRA264_I2S_TX_CIF_FIFO_STATUS: c_uint = 0xc0;
pub const TEGRA264_I2S_ENABLE: c_uint = 0x100;
pub const TEGRA264_I2S_CG: c_uint = 0x108;
pub const TEGRA264_I2S_STATUS: c_uint = 0x10c;
pub const TEGRA264_I2S_INT_STATUS: c_uint = 0x110;
pub const TEGRA264_I2S_INT_SET: c_uint = 0x114;
pub const TEGRA264_I2S_INT_MASK: c_uint = 0x11c;
pub const TEGRA264_I2S_CTRL: c_uint = 0x12c;
pub const TEGRA264_I2S_TIMING: c_uint = 0x130;
pub const TEGRA264_I2S_CYA: c_uint = 0x13c;
pub const TEGRA264_I2S_PIO_MODE_ENABLE: c_uint = 0x140;
pub const TEGRA264_I2S_PAD_MACRO_STATUS: c_uint = 0x144;
// Bit fields, shifts and masks
pub const I2S_DATA_SHIFT: c_int = 8;

pub const TEGRA264_I2S_FSYNC_WIDTH_SHIFT: c_int = 23;

pub const I2S_EN_SHIFT: c_int = 0;

pub const I2S_FSYNC_WIDTH_SHIFT: c_int = 24;

pub const I2S_POS_EDGE: c_int = 0;
pub const I2S_NEG_EDGE: c_int = 1;
pub const I2S_EDGE_SHIFT: c_int = 20;

pub const I2S_FMT_LRCK: c_int = 0;
pub const I2S_FMT_FSYNC: c_int = 1;
pub const I2S_FMT_SHIFT: c_int = 12;

pub const I2S_CTRL_MASTER_EN_SHIFT: c_int = 10;

pub const I2S_CTRL_LRCK_POL_SHIFT: c_int = 9;

pub const I2S_CTRL_LPBK_SHIFT: c_int = 8;

pub const I2S_BITS_8: c_int = 1;
pub const I2S_BITS_16: c_int = 3;
pub const I2S_BITS_24: c_int = 5;
pub const I2S_BITS_32: c_int = 7;
pub const I2S_CTRL_BIT_SIZE_MASK: c_uint = 0x7;
pub const I2S_TIMING_CH_BIT_CNT_MASK: c_uint = 0x7ff;
pub const I2S_TIMING_CH_BIT_CNT_SHIFT: c_int = 0;
pub const I2S_SOFT_RESET_SHIFT: c_int = 0;

pub const I2S_RX_FIFO_DEPTH: c_int = 64;
pub const DEFAULT_I2S_RX_FIFO_THRESHOLD: c_int = 3;
pub const DEFAULT_I2S_SLOT_MASK: c_uint = 0xffff;
pub const TEGRA210_I2S_TX_OFFSET: c_int = 0;
pub const TEGRA210_I2S_CTRL_OFFSET: c_int = 0;
pub const TEGRA210_I2S_MAX_CHANNEL: c_int = 16;
pub const TEGRA264_DEFAULT_I2S_SLOT_MASK: c_uint = 0xffffffff;
pub const TEGRA264_I2S_TX_OFFSET: c_uint = 0x40;
pub const TEGRA264_I2S_CTRL_OFFSET: c_uint = 0x8c;
pub const TEGRA264_I2S_MAX_CHANNEL: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra210_i2s_path {
    I2S_RX_PATH,
    I2S_TX_PATH,
    I2S_PATHS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_i2s_soc_data {
    pub regmap_conf: *const regmap_config,
    pub i2s_cmpnt: *const snd_soc_component_driver,
    pub max_ch: c_uint,
    pub enable_reg: c_uint,
    pub tx_offset: c_uint,
    pub i2s_ctrl_offset: c_uint,
    pub fsync_width_mask: c_uint,
    pub fsync_width_shift: c_uint,
    pub slot_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_i2s {
    pub soc_data: *const tegra_i2s_soc_data,
    pub clk_i2s: *mut clk,
    pub clk_sync_input: *mut clk,
    pub regmap: *mut regmap,
    pub client_sample_format: c_int,
    pub client_channels: c_uint,
    pub stereo_to_mono: [c_uint; I2S_PATHS],
    pub mono_to_stereo: [c_uint; I2S_PATHS],
    pub dai_fmt: c_uint,
    pub fsync_width: c_uint,
    pub bclk_ratio: c_uint,
    pub tx_mask: c_uint,
    pub rx_mask: c_uint,
    pub rx_fifo_th: c_uint,
    pub loopback: bool,
}
