//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp.h
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

pub const ACP_PAGE_SIZE_4K_ENABLE: c_uint = 0x02;
pub const ACP_PLAYBACK_PTE_OFFSET: c_int = 10;
pub const ACP_CAPTURE_PTE_OFFSET: c_int = 0;
// Playback and Capture Offset for Stoney
pub const ACP_ST_PLAYBACK_PTE_OFFSET: c_uint = 0x04;
pub const ACP_ST_CAPTURE_PTE_OFFSET: c_uint = 0x00;
pub const ACP_ST_BT_PLAYBACK_PTE_OFFSET: c_uint = 0x08;
pub const ACP_ST_BT_CAPTURE_PTE_OFFSET: c_uint = 0x0c;
pub const ACP_GARLIC_CNTL_DEFAULT: c_uint = 0x00000FB4;
pub const ACP_ONION_CNTL_DEFAULT: c_uint = 0x00000FB4;
pub const ACP_PHYSICAL_BASE: c_uint = 0x14000;
//
// In case of I2S SP controller instance, Stoney uses SRAM bank 1 for
// playback and SRAM Bank 2 for capture where as in case of BT I2S
// Instance, Stoney uses SRAM Bank 3 for playback & SRAM Bank 4 will
// be used for capture. Carrizo uses I2S SP controller instance. SRAM Banks
// 1, 2, 3, 4 will be used for playback & SRAM Banks 5, 6, 7, 8 will be used
// for capture scenario.
//
pub const ACP_SRAM_BANK_1_ADDRESS: c_uint = 0x4002000;
pub const ACP_SRAM_BANK_2_ADDRESS: c_uint = 0x4004000;
pub const ACP_SRAM_BANK_3_ADDRESS: c_uint = 0x4006000;
pub const ACP_SRAM_BANK_4_ADDRESS: c_uint = 0x4008000;
pub const ACP_SRAM_BANK_5_ADDRESS: c_uint = 0x400A000;
pub const ACP_DMA_RESET_TIME: c_int = 10000;
pub const ACP_CLOCK_EN_TIME_OUT_VALUE: c_uint = 0x000000FF;
pub const ACP_SOFT_RESET_DONE_TIME_OUT_VALUE: c_uint = 0x000000FF;
pub const ACP_DMA_COMPLETE_TIME_OUT_VALUE: c_uint = 0x000000FF;
pub const ACP_SRAM_BASE_ADDRESS: c_uint = 0x4000000;
pub const ACP_DAGB_GRP_SRAM_BASE_ADDRESS: c_uint = 0x4001000;
pub const ACP_DAGB_GRP_SRBM_SRAM_BASE_OFFSET: c_uint = 0x1000;
pub const ACP_INTERNAL_APERTURE_WINDOW_0_ADDRESS: c_uint = 0x00000000;
pub const ACP_INTERNAL_APERTURE_WINDOW_4_ADDRESS: c_uint = 0x01800000;
pub const TO_ACP_I2S_1: c_uint = 0x2;
pub const TO_ACP_I2S_2: c_uint = 0x4;
pub const TO_BLUETOOTH: c_uint = 0x3;
pub const FROM_ACP_I2S_1: c_uint = 0xa;
pub const FROM_ACP_I2S_2: c_uint = 0xb;
pub const FROM_BLUETOOTH: c_uint = 0xb;
pub const I2S_SP_INSTANCE: c_uint = 0x01;
pub const I2S_BT_INSTANCE: c_uint = 0x02;
pub const I2S_MICSP_INSTANCE: c_uint = 0x03;
pub const CAP_CHANNEL0: c_uint = 0x00;
pub const CAP_CHANNEL1: c_uint = 0x01;
pub const ACP_TILE_ON_MASK: c_uint = 0x03;
pub const ACP_TILE_OFF_MASK: c_uint = 0x02;
pub const ACP_TILE_ON_RETAIN_REG_MASK: c_uint = 0x1f;
pub const ACP_TILE_OFF_RETAIN_REG_MASK: c_uint = 0x20;
pub const ACP_TILE_P1_MASK: c_uint = 0x3e;
pub const ACP_TILE_P2_MASK: c_uint = 0x3d;
pub const ACP_TILE_DSP0_MASK: c_uint = 0x3b;
pub const ACP_TILE_DSP1_MASK: c_uint = 0x37;
pub const ACP_TILE_DSP2_MASK: c_uint = 0x2f;
// Playback DMA channels
pub const SYSRAM_TO_ACP_CH_NUM: c_int = 12;
pub const ACP_TO_I2S_DMA_CH_NUM: c_int = 13;
// Capture DMA channels
pub const I2S_TO_ACP_DMA_CH_NUM: c_int = 14;
pub const ACP_TO_SYSRAM_CH_NUM: c_int = 15;
// Playback DMA Channels for I2S BT instance
pub const SYSRAM_TO_ACP_BT_INSTANCE_CH_NUM: c_int = 8;
pub const ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM: c_int = 9;
// Capture DMA Channels for I2S BT Instance
pub const I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM: c_int = 10;
pub const ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM: c_int = 11;
// Playback DMA channels for I2S MICSP instance
pub const SYSRAM_TO_ACP_MICSP_INSTANCE_CH_NUM: c_int = 4;
pub const ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM: c_int = 5;
pub const NUM_DSCRS_PER_CHANNEL: c_int = 2;
pub const PLAYBACK_START_DMA_DESCR_CH12: c_int = 0;
pub const PLAYBACK_END_DMA_DESCR_CH12: c_int = 1;
pub const PLAYBACK_START_DMA_DESCR_CH13: c_int = 2;
pub const PLAYBACK_END_DMA_DESCR_CH13: c_int = 3;
pub const CAPTURE_START_DMA_DESCR_CH14: c_int = 4;
pub const CAPTURE_END_DMA_DESCR_CH14: c_int = 5;
pub const CAPTURE_START_DMA_DESCR_CH15: c_int = 6;
pub const CAPTURE_END_DMA_DESCR_CH15: c_int = 7;
// I2S BT Instance DMA Descriptors
pub const PLAYBACK_START_DMA_DESCR_CH8: c_int = 8;
pub const PLAYBACK_END_DMA_DESCR_CH8: c_int = 9;
pub const PLAYBACK_START_DMA_DESCR_CH9: c_int = 10;
pub const PLAYBACK_END_DMA_DESCR_CH9: c_int = 11;
pub const CAPTURE_START_DMA_DESCR_CH10: c_int = 12;
pub const CAPTURE_END_DMA_DESCR_CH10: c_int = 13;
pub const CAPTURE_START_DMA_DESCR_CH11: c_int = 14;
pub const CAPTURE_END_DMA_DESCR_CH11: c_int = 15;
// I2S MICSP Instance DMA Descriptors
pub const PLAYBACK_START_DMA_DESCR_CH4: c_int = 0;
pub const PLAYBACK_END_DMA_DESCR_CH4: c_int = 1;
pub const PLAYBACK_START_DMA_DESCR_CH5: c_int = 2;
pub const PLAYBACK_END_DMA_DESCR_CH5: c_int = 3;
pub const mmACP_I2S_16BIT_RESOLUTION_EN: c_uint = 0x5209;
pub const ACP_I2S_MIC_16BIT_RESOLUTION_EN: c_uint = 0x01;
pub const ACP_I2S_MICSP_16BIT_RESOLUTION_EN: c_uint = 0x01;
pub const ACP_I2S_SP_16BIT_RESOLUTION_EN: c_uint = 0x02;
pub const ACP_I2S_BT_16BIT_RESOLUTION_EN: c_uint = 0x04;
pub const ACP_BT_UART_PAD_SELECT_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acp_dma_priority_level {
// 0x0 Specifies the DMA channel is given normal priority
    ACP_DMA_PRIORITY_LEVEL_NORMAL = 0x0,
// 0x1 Specifies the DMA channel is given high priority
    ACP_DMA_PRIORITY_LEVEL_HIGH = 0x1,
    ACP_DMA_PRIORITY_LEVEL_FORCESIZE = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_substream_data {
    pub dma_addr: dma_addr_t,
    pub order: c_uint,
    pub num_of_pages: u16,
    pub i2s_instance: u16,
    pub capture_channel: u16,
    pub direction: u16,
    pub ch1: u16,
    pub ch2: u16,
    pub destination: u16,
    pub dma_dscr_idx_1: u16,
    pub dma_dscr_idx_2: u16,
    pub pte_offset: u32,
    pub sram_bank: u32,
    pub byte_cnt_high_reg_offset: u32,
    pub byte_cnt_low_reg_offset: u32,
    pub dma_curr_dscr: u32,
    pub size: u64,
    pub bytescount: u64,
    pub acp_mmio: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_drv_data {
    pub play_i2ssp_stream: *mut snd_pcm_substream,
    pub capture_i2ssp_stream: *mut snd_pcm_substream,
    pub play_i2sbt_stream: *mut snd_pcm_substream,
    pub capture_i2sbt_stream: *mut snd_pcm_substream,
    pub play_i2s_micsp_stream: *mut snd_pcm_substream,
    pub acp_mmio: *mut void __iomem,
    pub asic_type: u32,
    pub delay: snd_pcm_sframes_t,
}

//
// this structure used for platform data transfer between machine driver
// and dma driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
    pub capture_channel: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acp_dma_count {
    pub low: u32,
    pub high: u32,
    pub bcount: },
    pub bytescount: u64,
}

// Specifies the source memory location for the DMA data transfer.
//
// Specifies the destination memory location to where the data will
// be transferred.
//
// Specifies the number of bytes need to be transferred
// from source to destination memory.Transfer direction & IOC enable
//
// Reserved for future use
