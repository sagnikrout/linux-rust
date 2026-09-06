//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/bcm/bcm63xx-i2s.h
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
// linux/sound/soc/bcm/bcm63xx-i2s.h
// Copyright (c) 2020 Broadcom Corporation
// Author: Kevin-Ke Li <kevin-ke.li@broadcom.com>
pub const I2S_DESC_FIFO_DEPTH: c_int = 8;

pub const I2S_TX_DESC_OFF_LEVEL_SHIFT: c_int = 12;

pub const I2S_TX_DESC_IFF_LEVEL_SHIFT: c_int = 8;

pub const I2S_TX_SLAVE_MODE_SHIFT: c_int = 13;

pub const I2S_TX_MASTER_MODE: c_int = 0;
pub const I2S_TX_INTR_MASK: c_uint = 0x0F;

pub const I2S_RX_DESC_OFF_LEVEL_SHIFT: c_int = 12;

pub const I2S_RX_DESC_IFF_LEVEL_SHIFT: c_int = 8;

pub const I2S_RX_SLAVE_MODE_SHIFT: c_int = 13;

pub const I2S_RX_MASTER_MODE: c_int = 0;
pub const I2S_RX_INTR_MASK: c_uint = 0x0F;
pub const I2S_REG_MAX: c_uint = 0x007C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_i2s_priv {
    pub dev: *mut device,
    pub regmap_i2s: *mut regmap,
    pub i2s_clk: *mut clk,
    pub play_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub play_dma_desc: *mut i2s_dma_desc,
    pub capture_dma_desc: *mut i2s_dma_desc,
}

extern "C" {
    pub fn bcm63xx_soc_platform_remove(pdev: *mut platform_device) -> c_int;
}
