//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/ps/acp63.h
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
// AMD Common ACP header file for ACP6.3, ACP7.0 & ACP7.1 platforms
//
// Copyright (C) 2022, 2023, 2025 Advanced Micro Devices, Inc. All rights reserved.
//

pub const ACP_DEVICE_ID: c_uint = 0x15E2;
pub const ACP63_REG_START: c_uint = 0x1240000;
pub const ACP63_REG_END: c_uint = 0x125C000;
pub const ACP63_PCI_REV: c_uint = 0x63;
pub const ACP70_PCI_REV: c_uint = 0x70;
pub const ACP71_PCI_REV: c_uint = 0x71;
pub const ACP72_PCI_REV: c_uint = 0x72;
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: c_uint = 0x00010001;
pub const ACP63_PGFSM_CNTL_POWER_ON_MASK: c_int = 1;
pub const ACP63_PGFSM_CNTL_POWER_OFF_MASK: c_int = 0;
pub const ACP63_PGFSM_STATUS_MASK: c_int = 3;
pub const ACP63_POWERED_ON: c_int = 0;
pub const ACP63_POWER_ON_IN_PROGRESS: c_int = 1;
pub const ACP63_POWERED_OFF: c_int = 2;
pub const ACP63_POWER_OFF_IN_PROGRESS: c_int = 3;
pub const ACP_ERROR_MASK: c_uint = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: c_uint = 0xFFFFFFFF;
pub const PDM_DMA_STAT: c_uint = 0x10;
pub const PDM_DMA_INTR_MASK: c_uint = 0x10000;
pub const ACP_ERROR_STAT: c_int = 29;
pub const PDM_DECIMATION_FACTOR: c_int = 2;
pub const ACP_PDM_CLK_FREQ_MASK: c_int = 7;

pub const ACP_PDM_ENABLE: c_int = 1;
pub const ACP_PDM_DISABLE: c_int = 0;
pub const ACP_PDM_DMA_EN_STATUS: c_int = 2;
pub const TWO_CH: c_int = 2;
pub const DELAY_US: c_int = 5;
pub const ACP_COUNTER: c_int = 20000;
pub const ACP_SRAM_PTE_OFFSET: c_uint = 0x03800000;
pub const PAGE_SIZE_4K_ENABLE: c_int = 2;
pub const PDM_PTE_OFFSET: c_int = 0;
pub const PDM_MEM_WINDOW_START: c_uint = 0x4000000;
pub const CAPTURE_MIN_NUM_PERIODS: c_int = 4;
pub const CAPTURE_MAX_NUM_PERIODS: c_int = 4;
pub const CAPTURE_MAX_PERIOD_SIZE: c_int = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: c_int = 4096;

// time in ms for runtime suspend delay
pub const ACP_SUSPEND_DELAY_MS: c_int = 2000;
pub const ACP_DMIC_DEV: c_int = 2;
pub const ACP63_DMIC_ADDR: c_int = 2;
pub const ACP63_SDW_ADDR: c_int = 5;
pub const AMD_SDW_MAX_MANAGERS: c_int = 2;
// time in ms for acp timeout
pub const ACP63_TIMEOUT: c_int = 500;

pub const ACP_AUDIO0_TX_THRESHOLD: c_uint = 0x1c;
pub const ACP_AUDIO1_TX_THRESHOLD: c_uint = 0x1a;
pub const ACP_AUDIO2_TX_THRESHOLD: c_uint = 0x18;
pub const ACP_AUDIO0_RX_THRESHOLD: c_uint = 0x1b;
pub const ACP_AUDIO1_RX_THRESHOLD: c_uint = 0x19;
pub const ACP_AUDIO2_RX_THRESHOLD: c_uint = 0x17;

pub const ACP63_SDW_DMA_IRQ_MASK: c_uint = 0x1F800000;
pub const ACP63_P1_SDW_DMA_IRQ_MASK: c_uint = 0x60;
pub const ACP63_SDW0_DMA_MAX_STREAMS: c_int = 6;
pub const ACP63_SDW1_DMA_MAX_STREAMS: c_int = 2;
pub const ACP63_P1_AUDIO_TX_THRESHOLD: c_int = 6;
//
// Below entries describes SDW0 instance DMA stream id and DMA irq bit mapping
// in ACP_EXTENAL_INTR_CNTL register.
// Stream id		IRQ Bit
// 0 (SDW0_AUDIO0_TX)	28
// 1 (SDW0_AUDIO1_TX)	26
// 2 (SDW0_AUDIO2_TX)	24
// 3 (SDW0_AUDIO0_RX)	27
// 4 (SDW0_AUDIO1_RX)	25
// 5 (SDW0_AUDIO2_RX)	23
//

//
// Below entries describes SDW1 instance DMA stream id and DMA irq bit mapping
// in ACP_EXTENAL_INTR_CNTL1 register.
// Stream id		IRQ Bit
// 0 (SDW1_AUDIO1_TX)	6
// 1 (SDW1_AUDIO1_RX)	5
//

pub const ACP_DELAY_US: c_int = 5;

pub const SDW0_MEM_WINDOW_START: c_uint = 0x4800000;
pub const ACP_SDW_SRAM_PTE_OFFSET: c_uint = 0x03800400;
pub const SDW0_PTE_OFFSET: c_uint = 0x400;
pub const SDW_FIFO_SIZE: c_uint = 0x100;
pub const SDW_DMA_SIZE: c_uint = 0x40;
pub const ACP_SDW0_FIFO_OFFSET: c_uint = 0x100;
pub const ACP_SDW_PTE_OFFSET: c_uint = 0x100;
pub const SDW_FIFO_OFFSET: c_uint = 0x100;

pub const SDW_PLAYBACK_MIN_NUM_PERIODS: c_int = 2;
pub const SDW_PLAYBACK_MAX_NUM_PERIODS: c_int = 8;
pub const SDW_PLAYBACK_MAX_PERIOD_SIZE: c_int = 8192;
pub const SDW_PLAYBACK_MIN_PERIOD_SIZE: c_int = 1024;
pub const SDW_CAPTURE_MIN_NUM_PERIODS: c_int = 2;
pub const SDW_CAPTURE_MAX_NUM_PERIODS: c_int = 8;
pub const SDW_CAPTURE_MAX_PERIOD_SIZE: c_int = 8192;
pub const SDW_CAPTURE_MIN_PERIOD_SIZE: c_int = 1024;

pub const ACP70_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x1F;
pub const ACP70_PGFSM_CNTL_POWER_OFF_MASK: c_int = 0;
pub const ACP70_PGFSM_STATUS_MASK: c_uint = 0xFF;
pub const ACP70_TIMEOUT: c_int = 2000;
pub const ACP70_SDW_HOST_WAKE_MASK: c_uint = 0x0C00000;

pub const ACP70_SDW0_DMA_MAX_STREAMS: c_int = 6;

pub const ACP70_SDW_DMA_IRQ_MASK: c_uint = 0x1F800000;
pub const ACP70_P1_SDW_DMA_IRQ_MASK: c_uint = 0x1F8;
pub const ACP70_P1_AUDIO0_TX_THRESHOLD: c_uint = 0x8;
pub const ACP70_P1_AUDIO1_TX_THRESHOLD: c_uint = 0x6;
pub const ACP70_P1_AUDIO2_TX_THRESHOLD: c_uint = 0x4;
pub const ACP70_P1_AUDIO0_RX_THRESHOLD: c_uint = 0x7;
pub const ACP70_P1_AUDIO1_RX_THRESHOLD: c_uint = 0x5;
pub const ACP70_P1_AUDIO2_RX_THRESHOLD: c_uint = 0x3;

//
// Below entries describes SDW1 instance DMA stream id and DMA irq bit mapping
// in ACP_EXTENAL_INTR_CNTL1 register for ACP70/ACP71 platforms
// Stream id		IRQ Bit
// 0 (SDW1_AUDIO0_TX)	8
// 1 (SDW1_AUDIO1_TX)	6
// 2 (SDW1_AUDIO2_TX)	4
// 3 (SDW1_AUDIO0_RX)	7
// 4 (SDW1_AUDIO1_RX)	5
// 5 (SDW1_AUDIO2_RX)	3
//

pub const ACP70_SW1_AUDIO0_TX_EN: c_uint = 0x0003C10;
pub const ACP70_SW1_AUDIO1_TX_EN: c_uint = 0x0003C50;
pub const ACP70_SW1_AUDIO2_TX_EN: c_uint = 0x0003C6C;
pub const ACP70_SW1_AUDIO0_RX_EN: c_uint = 0x0003C88;
pub const ACP70_SW1_AUDIO1_RX_EN: c_uint = 0x0003D28;
pub const ACP70_SW1_AUDIO2_RX_EN: c_uint = 0x0003D44;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acp_config {
    ACP_CONFIG_0 = 0,
    ACP_CONFIG_1,
    ACP_CONFIG_2,
    ACP_CONFIG_3,
    ACP_CONFIG_4,
    ACP_CONFIG_5,
    ACP_CONFIG_6,
    ACP_CONFIG_7,
    ACP_CONFIG_8,
    ACP_CONFIG_9,
    ACP_CONFIG_10,
    ACP_CONFIG_11,
    ACP_CONFIG_12,
    ACP_CONFIG_13,
    ACP_CONFIG_14,
    ACP_CONFIG_15,
    ACP_CONFIG_16,
    ACP_CONFIG_17,
    ACP_CONFIG_18,
    ACP_CONFIG_19,
    ACP_CONFIG_20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_acp63_sdw0_channel {
    ACP63_SDW0_AUDIO0_TX = 0,
    ACP63_SDW0_AUDIO1_TX,
    ACP63_SDW0_AUDIO2_TX,
    ACP63_SDW0_AUDIO0_RX,
    ACP63_SDW0_AUDIO1_RX,
    ACP63_SDW0_AUDIO2_RX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_acp63_sdw1_channel {
    ACP63_SDW1_AUDIO1_TX,
    ACP63_SDW1_AUDIO1_RX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_acp70_sdw_channel {
    ACP70_SDW_AUDIO0_TX = 0,
    ACP70_SDW_AUDIO1_TX,
    ACP70_SDW_AUDIO2_TX,
    ACP70_SDW_AUDIO0_RX,
    ACP70_SDW_AUDIO1_RX,
    ACP70_SDW_AUDIO2_RX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdm_stream_instance {
    pub num_pages: u16,
    pub channels: u16,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp63_base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdm_dev_data {
    pub pdm_irq: u32,
    pub acp63_base: *mut void __iomem,
    pub acp_lock: *mut mutex,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dma_dev_data {
    pub acp_base: *mut void __iomem,
    pub /: *mut *mut *mut mutex acp_lock; / used to protect acp common register access,
    pub acp_rev: u32,
    pub acp63_sdw0_dma_stream: [*mut snd_pcm_substream; ACP63_SDW0_DMA_MAX_STREAMS],
    pub acp63_sdw1_dma_stream: [*mut snd_pcm_substream; ACP63_SDW1_DMA_MAX_STREAMS],
    pub acp70_sdw0_dma_stream: [*mut snd_pcm_substream; ACP70_SDW0_DMA_MAX_STREAMS],
    pub acp70_sdw1_dma_stream: [*mut snd_pcm_substream; ACP70_SDW1_DMA_MAX_STREAMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_sdw_dma_stream {
    pub num_pages: u16,
    pub channels: u16,
    pub stream_id: u32,
    pub instance: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acp_sdw_dma_count {
    pub low: u32,
    pub high: u32,
    pub bcount: },
    pub bytescount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dma_ring_buf_reg {
    pub reg_dma_size: u32,
    pub reg_fifo_addr: u32,
    pub reg_fifo_size: u32,
    pub reg_ring_buf_size: u32,
    pub reg_ring_buf_addr: u32,
    pub water_mark_size_reg: u32,
    pub pos_low_reg: u32,
    pub pos_high_reg: u32,
}

//
// struct acp_hw_ops - ACP PCI driver platform specific ops
// @acp_init: ACP initialization
// @acp_deinit: ACP de-initialization
// @acp_get_config: function to read the acp pin configuration
// @acp_sdw_dma_irq_thread: ACP SoundWire DMA interrupt thread
// acp_suspend: ACP system level suspend callback
// acp_resume: ACP system level resume callback
// acp_suspend_runtime: ACP runtime suspend callback
// acp_resume_runtime: ACP runtime resume callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_hw_ops {
    pub dev): *mut *mut *mut int (acp_init)(void __iomem acp_base, struct device,
    pub dev): *mut *mut *mut int (acp_deinit)(void __iomem acp_base, struct device,
    pub acp_data): *mut *mut *mut void (acp_get_config)(struct pci_dev pci, struct acp63_dev_data,
    pub acp_data): *mut *mut void (acp_sdw_dma_irq_thread)(struct acp63_dev_data,
    pub dev): *mut *mut int (acp_suspend)(struct device,
    pub dev): *mut *mut int (acp_resume)(struct device,
    pub dev): *mut *mut int (acp_suspend_runtime)(struct device,
    pub dev): *mut *mut int (acp_resume_runtime)(struct device,
}

//
// struct acp63_dev_data - acp pci driver context
// @acp63_base: acp mmio base
// @res: resource
// @hw_ops: ACP pci driver platform-specific ops
// @pdm_dev: ACP PDM controller platform device
// @dmic_codec: platform device for DMIC Codec
// sdw_dma_dev: platform device for SoundWire DMA controller
// @mach_dev: platform device for machine driver to support ACP PDM/SoundWire configuration
// @acp_lock: used to protect acp common registers
// @info: SoundWire AMD information found in ACPI tables
// @sdw: SoundWire context for all SoundWire manager instances
// @machine: ACPI machines for SoundWire interface
// @is_sdw_dev: flag set to true when any SoundWire manager instances are available
// @is_pdm_dev: flag set to true when ACP PDM controller exists
// @is_pdm_config: flat set to true when PDM configuration is selected from BIOS
// @is_sdw_config: flag set to true when SDW configuration is selected from BIOS
// @sdw_en_stat: flag set to true when any one of the SoundWire manager instance is enabled
// @acp70_sdw0_wake_event: flag set to true when wake irq asserted for SW0 instance
// @acp70_sdw1_wake_event: flag set to true when wake irq asserted for SW1 instance
// @addr: pci ioremap address
// @reg_range: ACP reigister range
// @acp_rev: ACP PCI revision id
// @acp_sw_pad_keeper_en: store acp SoundWire pad keeper enable register value
// @acp_pad_pulldown_ctrl: store acp pad pulldown control register value
// @acp63_sdw0-dma_intr_stat: DMA interrupt status array for ACP6.3 platform SoundWire
// manager-SW0 instance
// @acp63_sdw_dma_intr_stat: DMA interrupt status array for ACP6.3 platform SoundWire
// manager-SW1 instance
// @acp70_sdw0-dma_intr_stat: DMA interrupt status array for ACP7.0 platform SoundWire
// manager-SW0 instance
// @acp70_sdw_dma_intr_stat: DMA interrupt status array for ACP7.0 platform SoundWire
// manager-SW1 instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp63_dev_data {
    pub acp63_base: *mut void __iomem,
    pub res: *mut resource,
    pub hw_ops: *mut acp_hw_ops,
    pub pdm_dev: *mut platform_device,
    pub dmic_codec_dev: *mut platform_device,
    pub sdw_dma_dev: *mut platform_device,
    pub mach_dev: *mut platform_device,
    pub /: *mut *mut mutex acp_lock; / protect shared registers,
    pub info: sdw_amd_acpi_info,
// sdw context allocated by SoundWire driver
    pub sdw: *mut sdw_amd_ctx,
    pub machines: *mut snd_soc_acpi_mach,
    pub is_sdw_dev: bool,
    pub is_pdm_dev: bool,
    pub is_pdm_config: bool,
    pub is_sdw_config: bool,
    pub sdw_en_stat: bool,
    pub acp70_sdw0_wake_event: bool,
    pub acp70_sdw1_wake_event: bool,
    pub addr: u32,
    pub reg_range: u32,
    pub acp_rev: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub acp_sw_pad_keeper_en: u32,
    pub acp_pad_pulldown_ctrl: u32,
    pub acp63_sdw0_dma_intr_stat: [u16; ACP63_SDW0_DMA_MAX_STREAMS],
    pub acp63_sdw1_dma_intr_stat: [u16; ACP63_SDW1_DMA_MAX_STREAMS],
    pub acp70_sdw0_dma_intr_stat: [u16; ACP70_SDW0_DMA_MAX_STREAMS],
    pub acp70_sdw1_dma_intr_stat: [u16; ACP70_SDW1_DMA_MAX_STREAMS],
}

extern "C" {
    pub fn acp63_hw_init_ops(hw_ops: *mut acp_hw_ops);
}
extern "C" {
    pub fn acp70_hw_init_ops(hw_ops: *mut acp_hw_ops);
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_init)(adata->acp63_base, _arg: dev) -> return;
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_deinit)(adata->acp63_base, _arg: dev) -> return;
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_suspend)(dev) -> return;
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_resume)(dev) -> return;
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_suspend_runtime)(dev) -> return;
}
extern "C" {
    pub fn ACP_HW_OPS(_arg: adata, _arg: acp_resume_runtime)(dev) -> return;
}
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
