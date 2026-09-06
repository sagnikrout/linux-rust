//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra30_i2s.h
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
// tegra30_i2s.h - Definitions for Tegra30 I2S driver
//
// Copyright (c) 2011,2012, NVIDIA CORPORATION.  All rights reserved.
//

// Register offsets from TEGRA30_I2S*_BASE
pub const TEGRA30_I2S_CTRL: c_uint = 0x0;
pub const TEGRA30_I2S_TIMING: c_uint = 0x4;
pub const TEGRA30_I2S_OFFSET: c_uint = 0x08;
pub const TEGRA30_I2S_CH_CTRL: c_uint = 0x0c;
pub const TEGRA30_I2S_SLOT_CTRL: c_uint = 0x10;
pub const TEGRA30_I2S_CIF_RX_CTRL: c_uint = 0x14;
pub const TEGRA30_I2S_CIF_TX_CTRL: c_uint = 0x18;
pub const TEGRA30_I2S_FLOWCTL: c_uint = 0x1c;
pub const TEGRA30_I2S_TX_STEP: c_uint = 0x20;
pub const TEGRA30_I2S_FLOW_STATUS: c_uint = 0x24;
pub const TEGRA30_I2S_FLOW_TOTAL: c_uint = 0x28;
pub const TEGRA30_I2S_FLOW_OVER: c_uint = 0x2c;
pub const TEGRA30_I2S_FLOW_UNDER: c_uint = 0x30;
pub const TEGRA30_I2S_LCOEF_1_4_0: c_uint = 0x34;
pub const TEGRA30_I2S_LCOEF_1_4_1: c_uint = 0x38;
pub const TEGRA30_I2S_LCOEF_1_4_2: c_uint = 0x3c;
pub const TEGRA30_I2S_LCOEF_1_4_3: c_uint = 0x40;
pub const TEGRA30_I2S_LCOEF_1_4_4: c_uint = 0x44;
pub const TEGRA30_I2S_LCOEF_1_4_5: c_uint = 0x48;
pub const TEGRA30_I2S_LCOEF_2_4_0: c_uint = 0x4c;
pub const TEGRA30_I2S_LCOEF_2_4_1: c_uint = 0x50;
pub const TEGRA30_I2S_LCOEF_2_4_2: c_uint = 0x54;
// Fields in TEGRA30_I2S_CTRL

pub const TEGRA30_I2S_CTRL_OBS_SEL_SHIFT: c_int = 24;

pub const TEGRA30_I2S_FRAME_FORMAT_LRCK: c_int = 0;
pub const TEGRA30_I2S_FRAME_FORMAT_FSYNC: c_int = 1;
pub const TEGRA30_I2S_CTRL_FRAME_FORMAT_SHIFT: c_int = 12;

pub const TEGRA30_I2S_LRCK_LEFT_LOW: c_int = 0;
pub const TEGRA30_I2S_LRCK_RIGHT_LOW: c_int = 1;
pub const TEGRA30_I2S_CTRL_LRCK_SHIFT: c_int = 9;

pub const TEGRA30_I2S_BIT_CODE_LINEAR: c_int = 0;
pub const TEGRA30_I2S_BIT_CODE_ULAW: c_int = 1;
pub const TEGRA30_I2S_BIT_CODE_ALAW: c_int = 2;
pub const TEGRA30_I2S_CTRL_BIT_CODE_SHIFT: c_int = 4;

pub const TEGRA30_I2S_BITS_8: c_int = 1;
pub const TEGRA30_I2S_BITS_12: c_int = 2;
pub const TEGRA30_I2S_BITS_16: c_int = 3;
pub const TEGRA30_I2S_BITS_20: c_int = 4;
pub const TEGRA30_I2S_BITS_24: c_int = 5;
pub const TEGRA30_I2S_BITS_28: c_int = 6;
pub const TEGRA30_I2S_BITS_32: c_int = 7;
// Sample container size; see {RX,TX}_MASK field in CH_CTRL below
pub const TEGRA30_I2S_CTRL_BIT_SIZE_SHIFT: c_int = 0;

// Fields in TEGRA30_I2S_TIMING

pub const TEGRA30_I2S_TIMING_CHANNEL_BIT_COUNT_SHIFT: c_int = 0;
pub const TEGRA30_I2S_TIMING_CHANNEL_BIT_COUNT_MASK_US: c_uint = 0x7ff;

// Fields in TEGRA30_I2S_OFFSET
pub const TEGRA30_I2S_OFFSET_RX_DATA_OFFSET_SHIFT: c_int = 16;
pub const TEGRA30_I2S_OFFSET_RX_DATA_OFFSET_MASK_US: c_uint = 0x7ff;

pub const TEGRA30_I2S_OFFSET_TX_DATA_OFFSET_SHIFT: c_int = 0;
pub const TEGRA30_I2S_OFFSET_TX_DATA_OFFSET_MASK_US: c_uint = 0x7ff;

// Fields in TEGRA30_I2S_CH_CTRL
// (FSYNC width - 1) in bit clocks
pub const TEGRA30_I2S_CH_CTRL_FSYNC_WIDTH_SHIFT: c_int = 24;
pub const TEGRA30_I2S_CH_CTRL_FSYNC_WIDTH_MASK_US: c_uint = 0xff;

pub const TEGRA30_I2S_HIGHZ_NO: c_int = 0;
pub const TEGRA30_I2S_HIGHZ_YES: c_int = 1;
pub const TEGRA30_I2S_HIGHZ_ON_HALF_BIT_CLK: c_int = 2;
pub const TEGRA30_I2S_CH_CTRL_HIGHZ_CTRL_SHIFT: c_int = 12;

pub const TEGRA30_I2S_MSB_FIRST: c_int = 0;
pub const TEGRA30_I2S_LSB_FIRST: c_int = 1;
pub const TEGRA30_I2S_CH_CTRL_RX_BIT_ORDER_SHIFT: c_int = 10;

pub const TEGRA30_I2S_CH_CTRL_TX_BIT_ORDER_SHIFT: c_int = 9;

pub const TEGRA30_I2S_POS_EDGE: c_int = 0;
pub const TEGRA30_I2S_NEG_EDGE: c_int = 1;
pub const TEGRA30_I2S_CH_CTRL_EGDE_CTRL_SHIFT: c_int = 8;

// Sample size is # bits from BIT_SIZE minus this field
pub const TEGRA30_I2S_CH_CTRL_RX_MASK_BITS_SHIFT: c_int = 4;
pub const TEGRA30_I2S_CH_CTRL_RX_MASK_BITS_MASK_US: c_int = 7;

pub const TEGRA30_I2S_CH_CTRL_TX_MASK_BITS_SHIFT: c_int = 0;
pub const TEGRA30_I2S_CH_CTRL_TX_MASK_BITS_MASK_US: c_int = 7;

// Fields in TEGRA30_I2S_SLOT_CTRL
// Number of slots in frame, minus 1
pub const TEGRA30_I2S_SLOT_CTRL_TOTAL_SLOTS_SHIFT: c_int = 16;
pub const TEGRA30_I2S_SLOT_CTRL_TOTAL_SLOTS_MASK_US: c_int = 7;

// TDM mode slot enable bitmask
pub const TEGRA30_I2S_SLOT_CTRL_RX_SLOT_ENABLES_SHIFT: c_int = 8;

pub const TEGRA30_I2S_SLOT_CTRL_TX_SLOT_ENABLES_SHIFT: c_int = 0;

// Fields in TEGRA30_I2S_CIF_RX_CTRL
// Uses field from TEGRA30_AUDIOCIF_CTRL_* in tegra30_ahub.h
// Fields in TEGRA30_I2S_CIF_TX_CTRL
// Uses field from TEGRA30_AUDIOCIF_CTRL_* in tegra30_ahub.h
// Fields in TEGRA30_I2S_FLOWCTL
pub const TEGRA30_I2S_FILTER_LINEAR: c_int = 0;
pub const TEGRA30_I2S_FILTER_QUAD: c_int = 1;
pub const TEGRA30_I2S_FLOWCTL_FILTER_SHIFT: c_int = 31;

// Fields in TEGRA30_I2S_TX_STEP
pub const TEGRA30_I2S_TX_STEP_SHIFT: c_int = 0;
pub const TEGRA30_I2S_TX_STEP_MASK_US: c_uint = 0xffff;

// Fields in TEGRA30_I2S_FLOW_STATUS

//
// There are no fields in TEGRA30_I2S_FLOW_TOTAL, TEGRA30_I2S_FLOW_OVER,
// TEGRA30_I2S_FLOW_UNDER; they are counters taking the whole register.
//
// Fields in TEGRA30_I2S_LCOEF_*
pub const TEGRA30_I2S_LCOEF_COEF_SHIFT: c_int = 0;
pub const TEGRA30_I2S_LCOEF_COEF_MASK_US: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra30_i2s_soc_data {
    pub conf): *mut tegra30_ahub_cif_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra30_i2s {
    pub soc_data: *const tegra30_i2s_soc_data,
    pub dai: snd_soc_dai_driver,
    pub cif_id: c_int,
    pub clk_i2s: *mut clk,
    pub capture_i2s_cif: tegra30_ahub_txcif,
    pub capture_fifo_cif: tegra30_ahub_rxcif,
    pub capture_dma_chan: [c_char; 8],
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_i2s_cif: tegra30_ahub_rxcif,
    pub playback_fifo_cif: tegra30_ahub_txcif,
    pub playback_dma_chan: [c_char; 8],
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub regmap: *mut regmap,
    pub dma_config: snd_dmaengine_pcm_config,
}
