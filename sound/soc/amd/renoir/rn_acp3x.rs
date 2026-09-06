//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/renoir/rn_acp3x.h
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
// AMD ALSA SoC PDM Driver
//
// Copyright 2020 Advanced Micro Devices, Inc.
//

pub const ACP_DEVS: c_int = 3;
pub const ACP_PHY_BASE_ADDRESS: c_uint = 0x1240000;
pub const ACP_REG_START: c_uint = 0x1240000;
pub const ACP_REG_END: c_uint = 0x1250200;
pub const ACP_DEVICE_ID: c_uint = 0x15E2;
pub const ACP_POWER_ON: c_uint = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP_POWER_OFF: c_uint = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: c_uint = 0x00010001;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: c_uint = 0x00;
pub const ACP_PGFSM_STATUS_MASK: c_uint = 0x03;
pub const ACP_POWERED_ON: c_uint = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP_POWERED_OFF: c_uint = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP_ERROR_MASK: c_uint = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: c_uint = 0xFFFFFFFF;
pub const PDM_DMA_STAT: c_uint = 0x10;
pub const PDM_DMA_INTR_MASK: c_uint = 0x10000;
pub const ACP_ERROR_STAT: c_int = 29;
pub const PDM_DECIMATION_FACTOR: c_uint = 0x2;
pub const ACP_PDM_CLK_FREQ_MASK: c_uint = 0x07;

pub const ACP_PDM_ENABLE: c_uint = 0x01;
pub const ACP_PDM_DISABLE: c_uint = 0x00;
pub const ACP_PDM_DMA_EN_STATUS: c_uint = 0x02;
pub const TWO_CH: c_uint = 0x02;
pub const DELAY_US: c_int = 5;
pub const ACP_COUNTER: c_int = 20000;
// time in ms for runtime suspend delay
pub const ACP_SUSPEND_DELAY_MS: c_int = 2000;
pub const ACP_SRAM_PTE_OFFSET: c_uint = 0x02050000;
pub const PAGE_SIZE_4K_ENABLE: c_uint = 0x2;
pub const MEM_WINDOW_START: c_uint = 0x4000000;
pub const CAPTURE_MIN_NUM_PERIODS: c_int = 4;
pub const CAPTURE_MAX_NUM_PERIODS: c_int = 4;
pub const CAPTURE_MAX_PERIOD_SIZE: c_int = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdm_dev_data {
    pub pdm_irq: u32,
    pub acp_base: *mut void __iomem,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdm_stream_instance {
    pub num_pages: u16,
    pub channels: u16,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp_base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acp_pdm_dma_count {
    pub low: u32,
    pub high: u32,
    pub bcount: },
    pub bytescount: u64,
}

extern "C" {
    pub fn readl(ACP_PHY_BASE_ADDRESS: base_addr -) -> return;
}
// Machine configuration
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
