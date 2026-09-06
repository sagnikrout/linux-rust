//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/lpc3xxx-i2s.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2008 NXP Semiconductors
// Copyright 2023 Timesys Corporation <piotr.wojtaszczyk@timesys.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc3xxx_i2s_info {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub /: *mut *mut mutex lock; / To serialize user-space access,
    pub regs: *mut regmap,
    pub streams_in_use: u32,
    pub clkrate: u32,
    pub freq: c_int,
    pub playback_dma_config: snd_dmaengine_dai_dma_data,
    pub capture_dma_config: snd_dmaengine_dai_dma_data,
}

extern "C" {
    pub fn lpc3xxx_pcm_register(pdev: *mut platform_device) -> c_int;
}
// I2S controller register offsets
pub const LPC3XXX_REG_I2S_DAO: c_uint = 0x00;
pub const LPC3XXX_REG_I2S_DAI: c_uint = 0x04;
pub const LPC3XXX_REG_I2S_TX_FIFO: c_uint = 0x08;
pub const LPC3XXX_REG_I2S_RX_FIFO: c_uint = 0x0C;
pub const LPC3XXX_REG_I2S_STAT: c_uint = 0x10;
pub const LPC3XXX_REG_I2S_DMA0: c_uint = 0x14;
pub const LPC3XXX_REG_I2S_DMA1: c_uint = 0x18;
pub const LPC3XXX_REG_I2S_IRQ: c_uint = 0x1C;
pub const LPC3XXX_REG_I2S_TX_RATE: c_uint = 0x20;
pub const LPC3XXX_REG_I2S_RX_RATE: c_uint = 0x24;
// i2s_daO i2s_dai register definitions

pub const LPC3XXX_I2S_WW32_HP: c_uint = 0x1f /* Word select half period for 32bit word width */;
pub const LPC3XXX_I2S_WW16_HP: c_uint = 0x0f /* Word select half period for 16bit word width */;
pub const LPC3XXX_I2S_WW8_HP: c_uint = 0x7  /* Word select half period for 8bit word width */;
// i2s_stat register definitions

// i2s_dma0 Configuration register definitions

// i2s_dma1 Configuration register definitions

// i2s_irq register definitions

