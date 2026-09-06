//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra20_ac97.h
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
// tegra20_ac97.h - Definitions for the Tegra20 AC97 controller driver
//
// Copyright (c) 2012 Lucas Stach <dev@lynxeye.de>
//
// Partly based on code copyright/by:
//
// Copyright (c) 2011,2012 Toradex Inc.
//

pub const TEGRA20_AC97_CTRL: c_uint = 0x00;
pub const TEGRA20_AC97_CMD: c_uint = 0x04;
pub const TEGRA20_AC97_STATUS1: c_uint = 0x08;
// ...
pub const TEGRA20_AC97_FIFO1_SCR: c_uint = 0x1c;
// ...
pub const TEGRA20_AC97_FIFO_TX1: c_uint = 0x40;
pub const TEGRA20_AC97_FIFO_RX1: c_uint = 0x80;
// TEGRA20_AC97_CTRL

// TEGRA20_AC97_CMD
pub const TEGRA20_AC97_CMD_CMD_ADDR_SHIFT: c_int = 24;

pub const TEGRA20_AC97_CMD_CMD_DATA_SHIFT: c_int = 8;

pub const TEGRA20_AC97_CMD_CMD_ID_SHIFT: c_int = 2;

// TEGRA20_AC97_STATUS1
pub const TEGRA20_AC97_STATUS1_STA_ADDR1_SHIFT: c_int = 24;

pub const TEGRA20_AC97_STATUS1_STA_DATA1_SHIFT: c_int = 8;

// TEGRA20_AC97_FIFO1_SCR
pub const TEGRA20_AC97_FIFO_SCR_REC_MT_CNT_SHIFT: c_int = 27;

pub const TEGRA20_AC97_FIFO_SCR_PB_MT_CNT_SHIFT: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra20_ac97 {
    pub clk_ac97: *mut clk,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub reset: *mut reset_control,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub sync_gpio: *mut gpio_desc,
}
