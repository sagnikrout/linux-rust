//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/loongson/loongson_i2s.h
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


// SPDX-License-Identifier: GPL-2.0
//
// ALSA I2S interface for the Loongson platform
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//

// I2S Common Registers
pub const LS_I2S_VER: c_uint = 0x00 /* I2S Version */;
pub const LS_I2S_CFG: c_uint = 0x04 /* I2S Config */;
pub const LS_I2S_CTRL: c_uint = 0x08 /* I2S Control */;
pub const LS_I2S_RX_DATA: c_uint = 0x0C /* I2S DMA RX Address */;
pub const LS_I2S_TX_DATA: c_uint = 0x10 /* I2S DMA TX Address */;
// 2K2000 I2S Specify Registers
pub const LS_I2S_CFG1: c_uint = 0x14 /* I2S Config1 */;
// 7A2000 I2S Specify Registers
pub const LS_I2S_TX_ORDER: c_uint = 0x100 /* TX DMA Order */;
pub const LS_I2S_RX_ORDER: c_uint = 0x110 /* RX DMA Order */;
// Loongson I2S Control Register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_idma_data {
    pub /: *mut *mut dma_addr_t dev_addr; / device physical address for DMA,
    pub /: *mut *mut *mut void __iomem order_addr; / DMA order register,
    pub /: *mut *mut int irq; / DMA irq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_i2s {
    pub dev: *mut device,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub tx_dma_data: loongson_idma_data,
}
