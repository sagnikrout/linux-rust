//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/vangogh/acp5x.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PCM Driver
//
// Copyright (C) 2021 Advanced Micro Devices, Inc. All rights reserved.
//

pub const ACP5x_PHY_BASE_ADDRESS: c_uint = 0x1240000;
pub const ACP_DEVICE_ID: c_uint = 0x15E2;
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: c_uint = 0x00010001;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: c_uint = 0x00;
pub const ACP_PGFSM_STATUS_MASK: c_uint = 0x03;
pub const ACP_POWERED_ON: c_uint = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP_POWERED_OFF: c_uint = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP_ERR_INTR_MASK: c_uint = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: c_uint = 0xFFFFFFFF;
pub const ACP5x_DEVS: c_int = 4;
pub const ACP5x_REG_START: c_uint = 0x1240000;
pub const ACP5x_REG_END: c_uint = 0x1250200;
pub const ACP5x_I2STDM_REG_START: c_uint = 0x1242400;
pub const ACP5x_I2STDM_REG_END: c_uint = 0x1242410;
pub const ACP5x_HS_TDM_REG_START: c_uint = 0x1242814;
pub const ACP5x_HS_TDM_REG_END: c_uint = 0x1242824;
pub const I2S_MODE: c_int = 0;
pub const ACP5x_I2S_MODE: c_int = 1;
pub const ACP5x_RES: c_int = 4;
pub const I2S_RX_THRESHOLD: c_int = 27;
pub const I2S_TX_THRESHOLD: c_int = 28;
pub const HS_TX_THRESHOLD: c_int = 24;
pub const HS_RX_THRESHOLD: c_int = 23;
pub const I2S_SP_INSTANCE: c_int = 1;
pub const I2S_HS_INSTANCE: c_int = 2;
pub const ACP_SRAM_PTE_OFFSET: c_uint = 0x02050000;
pub const ACP_SRAM_SP_PB_PTE_OFFSET: c_uint = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: c_uint = 0x100;
pub const ACP_SRAM_HS_PB_PTE_OFFSET: c_uint = 0x200;
pub const ACP_SRAM_HS_CP_PTE_OFFSET: c_uint = 0x300;
pub const PAGE_SIZE_4K_ENABLE: c_uint = 0x2;
pub const I2S_SP_TX_MEM_WINDOW_START: c_uint = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: c_uint = 0x4020000;
pub const I2S_HS_TX_MEM_WINDOW_START: c_uint = 0x4040000;
pub const I2S_HS_RX_MEM_WINDOW_START: c_uint = 0x4060000;
pub const SP_PB_FIFO_ADDR_OFFSET: c_uint = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: c_uint = 0x700;
pub const HS_PB_FIFO_ADDR_OFFSET: c_uint = 0x900;
pub const HS_CAPT_FIFO_ADDR_OFFSET: c_uint = 0xB00;
pub const PLAYBACK_MIN_NUM_PERIODS: c_int = 2;
pub const PLAYBACK_MAX_NUM_PERIODS: c_int = 8;
pub const PLAYBACK_MAX_PERIOD_SIZE: c_int = 8192;
pub const PLAYBACK_MIN_PERIOD_SIZE: c_int = 1024;
pub const CAPTURE_MIN_NUM_PERIODS: c_int = 2;
pub const CAPTURE_MAX_NUM_PERIODS: c_int = 8;
pub const CAPTURE_MAX_PERIOD_SIZE: c_int = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: c_int = 1024;

pub const FIFO_SIZE: c_uint = 0x100;
pub const DMA_SIZE: c_uint = 0x40;
pub const FRM_LEN: c_uint = 0x100;
pub const I2S_MASTER_MODE_ENABLE: c_int = 1;
pub const I2S_MASTER_MODE_DISABLE: c_int = 0;
pub const SLOT_WIDTH_8: c_int = 8;
pub const SLOT_WIDTH_16: c_int = 16;
pub const SLOT_WIDTH_24: c_int = 24;
pub const SLOT_WIDTH_32: c_int = 32;
pub const TDM_ENABLE: c_int = 1;
pub const TDM_DISABLE: c_int = 0;
pub const ACP5x_ITER_IRER_SAMP_LEN_MASK: c_uint = 0x38;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_dev_data {
    pub tdm_mode: bool,
    pub master_mode: bool,
    pub i2s_irq: c_int,
    pub i2s_instance: u16,
    pub tdm_fmt: u32,
    pub acp5x_base: *mut void __iomem,
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
    pub i2ssp_play_stream: *mut snd_pcm_substream,
    pub i2ssp_capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_stream_instance {
    pub num_pages: u16,
    pub i2s_instance: u16,
    pub direction: u16,
    pub channels: u16,
    pub xfer_resolution: u32,
    pub val: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp5x_base: *mut void __iomem,
    pub lrclk_div: u32,
    pub bclk_div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acp_dma_count {
    pub low: u32,
    pub high: u32,
    pub bcount: },
    pub bytescount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp5x_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acp_i2stdm_mstrclkgen {
    pub 1: u32 i2stdm_master_mode :,
    pub 1: u32 i2stdm_format_mode :,
    pub 9: u32 i2stdm_lrclk_div_val :,
    pub 11: u32 i2stdm_bclk_div_val :,
    pub bits: } bitfields,,
    pub u32_all: u32,
}

// common header file uses exact offset rather than relative
// offset which requires subtraction logic from base_addr
// for accessing ACP5x MMIO space registers
//
extern "C" {
    pub fn readl(ACP5x_PHY_BASE_ADDRESS: base_addr -) -> return;
}
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
