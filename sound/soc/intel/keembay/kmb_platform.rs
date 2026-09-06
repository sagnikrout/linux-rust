//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/keembay/kmb_platform.h
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
// Intel KeemBay Platform driver
//
// Copyright (C) 2020 Intel Corporation.
//

// Register values with reference to KMB databook v1.1
// common register for all channel
pub const IER: c_uint = 0x000;
pub const IRER: c_uint = 0x004;
pub const ITER: c_uint = 0x008;
pub const CER: c_uint = 0x00C;
pub const CCR: c_uint = 0x010;
pub const RXFFR: c_uint = 0x014;
pub const TXFFR: c_uint = 0x018;
// Interrupt status register fields

// I2S Tx Rx Registers for all channels

// I2S COMP Registers
pub const I2S_COMP_PARAM_2: c_uint = 0x01F0;
pub const I2S_COMP_PARAM_1: c_uint = 0x01F4;
pub const I2S_COMP_VERSION: c_uint = 0x01F8;
pub const I2S_COMP_TYPE: c_uint = 0x01FC;
// PSS_GEN_CTRL_I2S_GEN_CFG_0 Registers
pub const I2S_GEN_CFG_0: c_uint = 0x000;
pub const PSS_CPR_RST_EN: c_uint = 0x010;
pub const PSS_CPR_RST_SET: c_uint = 0x014;
pub const PSS_CPR_CLK_CLR: c_uint = 0x000;
pub const PSS_CPR_AUX_RST_EN: c_uint = 0x070;

// Interrupt Flag

//
// Component parameter register fields - define the I2S block's
// configuration.
//

// Add 1 to the below registers to indicate the actual size

// Number of entries in WORDSIZE and DATA_WIDTH parameter registers

pub const MAX_CHANNEL_NUM: c_int = 8;
pub const MIN_CHANNEL_NUM: c_int = 2;
pub const MAX_ISR: c_int = 4;

pub const I2S_RXDMA: c_uint = 0x01C0;
pub const I2S_RRXDMA: c_uint = 0x01C4;
pub const I2S_TXDMA: c_uint = 0x01C8;
pub const I2S_RTXDMA: c_uint = 0x01CC;
pub const I2S_DMACR: c_uint = 0x0200;

//
// struct i2s_clk_config_data - represent i2s clk configuration data
// @chan_nr: number of channel
// @data_width: number of bits per sample (8/16/24/32 bit)
// @sample_rate: sampling frequency (8Khz, 16Khz, 48Khz)
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
pub struct kmb_i2s_info {
    pub i2s_base: *mut void __iomem,
    pub pss_base: *mut void __iomem,
    pub clk_i2s: *mut clk,
    pub clk_apb: *mut clk,
    pub active: c_int,
    pub capability: c_uint,
    pub i2s_reg_comp1: c_uint,
    pub i2s_reg_comp2: c_uint,
    pub dev: *mut device,
    pub ccr: u32,
    pub xfer_resolution: u32,
    pub fifo_th: u32,
    pub clock_provider: bool,
// data related to DMA transfers b/w i2s and DMAC
    pub play_dma_data: snd_dmaengine_dai_dma_data,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub config: i2s_clk_config_data,
    pub config): *mut *mut int (i2s_clk_cfg)(struct i2s_clk_config_data,
// data related to PIO transfers
    pub use_pio: bool,
    pub tx_substream: *mut snd_pcm_substream,
    pub rx_substream: *mut snd_pcm_substream,
    pub tx_ptr: c_uint,
    pub rx_ptr: c_uint,
    pub iec958_fmt: bool,
}
