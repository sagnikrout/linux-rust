//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/raven/acp3x.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
//

pub const I2S_SP_INSTANCE: c_uint = 0x01;
pub const I2S_BT_INSTANCE: c_uint = 0x02;
pub const TDM_ENABLE: c_int = 1;
pub const TDM_DISABLE: c_int = 0;
pub const ACP3x_DEVS: c_int = 4;
pub const ACP3x_PHY_BASE_ADDRESS: c_uint = 0x1240000;
pub const ACP3x_I2S_MODE: c_int = 0;
pub const ACP3x_REG_START: c_uint = 0x1240000;
pub const ACP3x_REG_END: c_uint = 0x1250200;
pub const ACP3x_I2STDM_REG_START: c_uint = 0x1242400;
pub const ACP3x_I2STDM_REG_END: c_uint = 0x1242410;
pub const ACP3x_BT_TDM_REG_START: c_uint = 0x1242800;
pub const ACP3x_BT_TDM_REG_END: c_uint = 0x1242810;
pub const I2S_MODE: c_uint = 0x04;
pub const I2S_RX_THRESHOLD: c_int = 27;
pub const I2S_TX_THRESHOLD: c_int = 28;
pub const BT_TX_THRESHOLD: c_int = 26;
pub const BT_RX_THRESHOLD: c_int = 25;
pub const ACP_ERR_INTR_MASK: c_int = 29;
pub const ACP3x_POWER_ON: c_uint = 0x00;
pub const ACP3x_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP3x_POWER_OFF: c_uint = 0x02;
pub const ACP3x_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP3x_SOFT_RESET__SoftResetAudDone_MASK: c_uint = 0x00010001;
pub const ACP_SRAM_PTE_OFFSET: c_uint = 0x02050000;
pub const ACP_SRAM_SP_PB_PTE_OFFSET: c_uint = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: c_uint = 0x100;
pub const ACP_SRAM_BT_PB_PTE_OFFSET: c_uint = 0x200;
pub const ACP_SRAM_BT_CP_PTE_OFFSET: c_uint = 0x300;
pub const PAGE_SIZE_4K_ENABLE: c_uint = 0x2;
pub const I2S_SP_TX_MEM_WINDOW_START: c_uint = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: c_uint = 0x4020000;
pub const I2S_BT_TX_MEM_WINDOW_START: c_uint = 0x4040000;
pub const I2S_BT_RX_MEM_WINDOW_START: c_uint = 0x4060000;
pub const SP_PB_FIFO_ADDR_OFFSET: c_uint = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: c_uint = 0x700;
pub const BT_PB_FIFO_ADDR_OFFSET: c_uint = 0x900;
pub const BT_CAPT_FIFO_ADDR_OFFSET: c_uint = 0xB00;
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
pub const SLOT_WIDTH_8: c_uint = 0x08;
pub const SLOT_WIDTH_16: c_uint = 0x10;
pub const SLOT_WIDTH_24: c_uint = 0x18;
pub const SLOT_WIDTH_32: c_uint = 0x20;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: c_uint = 0x00;
pub const ACP_PGFSM_STATUS_MASK: c_uint = 0x03;
pub const ACP_POWERED_ON: c_uint = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP_POWERED_OFF: c_uint = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP3x_ITER_IRER_SAMP_LEN_MASK: c_uint = 0x38;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp3x_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
    pub capture_channel: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_dev_data {
    pub tdm_mode: bool,
    pub i2s_irq: c_int,
    pub i2s_instance: u16,
    pub tdm_fmt: u32,
    pub substream_type: u32,
    pub acp3x_base: *mut void __iomem,
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
    pub capture_channel: u16,
    pub direction: u16,
    pub channels: u16,
    pub xfer_resolution: u32,
    pub val: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp3x_base: *mut void __iomem,
}

extern "C" {
    pub fn readl(ACP3x_PHY_BASE_ADDRESS: base_addr -) -> return;
}
