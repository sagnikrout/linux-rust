//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra20_i2s.h
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
// tegra20_i2s.h - Definitions for Tegra20 I2S driver
//
// Author: Stephen Warren <swarren@nvidia.com>
// Copyright (C) 2010,2012 - NVIDIA, Inc.
//
// Based on code copyright/by:
//
// Copyright (c) 2009-2010, NVIDIA Corporation.
// Scott Peterson <speterson@nvidia.com>
//
// Copyright (C) 2010 Google, Inc.
// Iliyan Malchev <malchev@google.com>
//

// Register offsets from TEGRA20_I2S1_BASE and TEGRA20_I2S2_BASE
pub const TEGRA20_I2S_CTRL: c_uint = 0x00;
pub const TEGRA20_I2S_STATUS: c_uint = 0x04;
pub const TEGRA20_I2S_TIMING: c_uint = 0x08;
pub const TEGRA20_I2S_FIFO_SCR: c_uint = 0x0c;
pub const TEGRA20_I2S_PCM_CTRL: c_uint = 0x10;
pub const TEGRA20_I2S_NW_CTRL: c_uint = 0x14;
pub const TEGRA20_I2S_TDM_CTRL: c_uint = 0x20;
pub const TEGRA20_I2S_TDM_TX_RX_CTRL: c_uint = 0x24;
pub const TEGRA20_I2S_FIFO1: c_uint = 0x40;
pub const TEGRA20_I2S_FIFO2: c_uint = 0x80;
// Fields in TEGRA20_I2S_CTRL

pub const TEGRA20_I2S_LRCK_LEFT_LOW: c_int = 0;
pub const TEGRA20_I2S_LRCK_RIGHT_LOW: c_int = 1;
pub const TEGRA20_I2S_CTRL_LRCK_SHIFT: c_int = 24;

pub const TEGRA20_I2S_BIT_FORMAT_I2S: c_int = 0;
pub const TEGRA20_I2S_BIT_FORMAT_RJM: c_int = 1;
pub const TEGRA20_I2S_BIT_FORMAT_LJM: c_int = 2;
pub const TEGRA20_I2S_BIT_FORMAT_DSP: c_int = 3;
pub const TEGRA20_I2S_CTRL_BIT_FORMAT_SHIFT: c_int = 10;

pub const TEGRA20_I2S_BIT_SIZE_16: c_int = 0;
pub const TEGRA20_I2S_BIT_SIZE_20: c_int = 1;
pub const TEGRA20_I2S_BIT_SIZE_24: c_int = 2;
pub const TEGRA20_I2S_BIT_SIZE_32: c_int = 3;
pub const TEGRA20_I2S_CTRL_BIT_SIZE_SHIFT: c_int = 8;

pub const TEGRA20_I2S_FIFO_16_LSB: c_int = 0;
pub const TEGRA20_I2S_FIFO_20_LSB: c_int = 1;
pub const TEGRA20_I2S_FIFO_24_LSB: c_int = 2;
pub const TEGRA20_I2S_FIFO_32: c_int = 3;
pub const TEGRA20_I2S_FIFO_PACKED: c_int = 7;
pub const TEGRA20_I2S_CTRL_FIFO_FORMAT_SHIFT: c_int = 4;

// Fields in TEGRA20_I2S_STATUS

// Fields in TEGRA20_I2S_TIMING

pub const TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_SHIFT: c_int = 0;
pub const TEGRA20_I2S_TIMING_CHANNEL_BIT_COUNT_MASK_US: c_uint = 0x7ff;

// Fields in TEGRA20_I2S_FIFO_SCR
pub const TEGRA20_I2S_FIFO_SCR_FIFO2_FULL_EMPTY_COUNT_SHIFT: c_int = 24;
pub const TEGRA20_I2S_FIFO_SCR_FIFO1_FULL_EMPTY_COUNT_SHIFT: c_int = 16;
pub const TEGRA20_I2S_FIFO_SCR_FIFO_FULL_EMPTY_COUNT_MASK: c_uint = 0x3f;

pub const TEGRA20_I2S_FIFO_ATN_LVL_ONE_SLOT: c_int = 0;
pub const TEGRA20_I2S_FIFO_ATN_LVL_FOUR_SLOTS: c_int = 1;
pub const TEGRA20_I2S_FIFO_ATN_LVL_EIGHT_SLOTS: c_int = 2;
pub const TEGRA20_I2S_FIFO_ATN_LVL_TWELVE_SLOTS: c_int = 3;
pub const TEGRA20_I2S_FIFO_SCR_FIFO2_ATN_LVL_SHIFT: c_int = 4;

pub const TEGRA20_I2S_FIFO_SCR_FIFO1_ATN_LVL_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra20_i2s {
    pub dai: snd_soc_dai_driver,
    pub clk_i2s: *mut clk,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub regmap: *mut regmap,
    pub reset: *mut reset_control,
}
