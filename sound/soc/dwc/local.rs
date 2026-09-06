//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/dwc/local.h
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
// Copyright (ST) 2012 Rajeev Kumar (rajeevkumar.linux@gmail.com)
//

// common register for all channel
pub const IER: c_uint = 0x000;
pub const IRER: c_uint = 0x004;
pub const ITER: c_uint = 0x008;
pub const CER: c_uint = 0x00C;
pub const CCR: c_uint = 0x010;
pub const RXFFR: c_uint = 0x014;
pub const TXFFR: c_uint = 0x018;
// Enable register fields
pub const IER_TDM_SLOTS_SHIFT: c_int = 8;
pub const IER_FRAME_OFF_SHIFT: c_int = 5;

// Interrupt status register fields

// I2STxRxRegisters for all channels

// Receive enable register fields
pub const RER_RXSLOT_SHIFT: c_int = 8;

// Transmit enable register fields
pub const TER_TXSLOT_SHIFT: c_int = 8;

// I2SCOMPRegisters
pub const I2S_COMP_PARAM_2: c_uint = 0x01F0;
pub const I2S_COMP_PARAM_1: c_uint = 0x01F4;
pub const I2S_COMP_VERSION: c_uint = 0x01F8;
pub const I2S_COMP_TYPE: c_uint = 0x01FC;
pub const I2S_RRXDMA: c_uint = 0x01C4;
pub const I2S_RTXDMA: c_uint = 0x01CC;
pub const I2S_DMACR: c_uint = 0x0200;

//
// Component parameter register fields - define the I2S block's
// configuration.
//

// Number of entries in WORDSIZE and DATA_WIDTH parameter registers

pub const MAX_CHANNEL_NUM: c_int = 8;
pub const MIN_CHANNEL_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union dw_i2s_snd_dma_data {
    pub pd: i2s_dma_data,
    pub dt: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_i2s_dev {
    pub i2s_base: *mut void __iomem,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub active: c_int,
    pub capability: c_uint,
    pub quirks: c_uint,
    pub i2s_reg_comp1: c_uint,
    pub i2s_reg_comp2: c_uint,
    pub dev: *mut device,
    pub ccr: u32,
    pub xfer_resolution: u32,
    pub fifo_th: u32,
    pub l_reg: u32,
    pub r_reg: u32,
    pub /: *mut *mut bool is_jh7110; / Flag for StarFive JH7110 SoC,
// data related to DMA transfers b/w i2s and DMAC
    pub play_dma_data: dw_i2s_snd_dma_data,
    pub capture_dma_data: dw_i2s_snd_dma_data,
    pub config: i2s_clk_config_data,
    pub config): *mut *mut int (i2s_clk_cfg)(struct i2s_clk_config_data,
// data related to PIO transfers
    pub use_pio: bool,
// data related to TDM mode
    pub tdm_slots: u32,
    pub tdm_mask: u32,
    pub frame_offset: u32,
    pub tx_substream: *mut snd_pcm_substream __rcu,
    pub rx_substream: *mut snd_pcm_substream __rcu,
    pub period_elapsed): *mut bool,
    pub period_elapsed): *mut bool,
    pub tx_ptr: c_uint,
    pub rx_ptr: c_uint,
}

extern "C" {
    pub fn dw_pcm_push_tx(dev: *mut dw_i2s_dev);
}
extern "C" {
    pub fn dw_pcm_pop_rx(dev: *mut dw_i2s_dev);
}
extern "C" {
    pub fn dw_pcm_register(pdev: *mut platform_device) -> c_int;
}

