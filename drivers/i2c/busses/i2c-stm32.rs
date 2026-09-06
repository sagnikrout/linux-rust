//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-stm32.h
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
// i2c-stm32.h
//
// Copyright (C) M'boumba Cedric Madianga 2017
// Copyright (C) STMicroelectronics 2017
// Author: M'boumba Cedric Madianga <cedric.madianga@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_i2c_speed {
    STM32_I2C_SPEED_STANDARD, /* 100 kHz */
    STM32_I2C_SPEED_FAST, /* 400 kHz */
    STM32_I2C_SPEED_FAST_PLUS, /* 1 MHz */
    STM32_I2C_SPEED_END,
}

//
// struct stm32_i2c_dma - DMA specific data
// @chan_tx: dma channel for TX transfer
// @chan_rx: dma channel for RX transfer
// @chan_using: dma channel used for the current transfer (TX or RX)
// @dma_buf: dma buffer
// @dma_len: dma buffer len
// @dma_transfer_dir: dma transfer direction indicator
// @dma_data_dir: dma transfer mode indicator
// @dma_complete: dma transfer completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_i2c_dma {
    pub chan_tx: *mut dma_chan,
    pub chan_rx: *mut dma_chan,
    pub chan_using: *mut dma_chan,
    pub dma_buf: dma_addr_t,
    pub dma_len: c_uint,
    pub dma_transfer_dir: dma_transfer_direction,
    pub dma_data_dir: dma_data_direction,
    pub dma_complete: completion,
}

extern "C" {
    pub fn stm32_i2c_dma_free(dma: *mut stm32_i2c_dma);
}
