//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/designware_i2s.h
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
// Copyright (ST) 2012 Rajeev Kumar (rajeevkumar.linux@gmail.com)
//

//
// struct i2s_clk_config_data - represent i2s clk configuration data
// @chan_nr: number of channel
// @data_width: number of bits per sample (8/16/24/32 bit)
// @sample_rate: sampling frequency (8Khz, 16Khz, 32Khz, 44Khz, 48Khz)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_clk_config_data {
    pub chan_nr: c_int,
    pub data_width: u32,
    pub sample_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_platform_data {

    pub cap: c_uint,
    pub channel: c_int,
    pub snd_fmts: u32,
    pub snd_rates: u32,

    pub quirks: c_uint,
    pub i2s_reg_comp1: c_uint,
    pub i2s_reg_comp2: c_uint,
    pub play_dma_data: *mut c_void,
    pub capture_dma_data: *mut c_void,
    pub slave): *mut *mut *mut bool (filter)(struct dma_chan chan, void,
    pub config): *mut *mut int (i2s_clk_cfg)(struct i2s_clk_config_data,
    pub dev): *mut *mut int (i2s_pd_init)(struct dw_i2s_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_dma_data {
    pub data: *mut c_void,
    pub addr: dma_addr_t,
    pub max_burst: u32,
    pub addr_width: dma_slave_buswidth,
    pub slave): *mut *mut *mut bool (filter)(struct dma_chan chan, void,
}

// I2S DMA registers
pub const I2S_RXDMA: c_uint = 0x01C0;
pub const I2S_TXDMA: c_uint = 0x01C8;

