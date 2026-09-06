//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp/amd.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

pub const DMIC_INSTANCE: c_uint = 0x00;
pub const I2S_SP_INSTANCE: c_uint = 0x01;
pub const I2S_BT_INSTANCE: c_uint = 0x02;
pub const I2S_HS_INSTANCE: c_uint = 0x03;
pub const MEM_WINDOW_START: c_uint = 0x4080000;
pub const ACP_I2S_REG_START: c_uint = 0x1242400;
pub const ACP_I2S_REG_END: c_uint = 0x1242810;
pub const ACP3x_I2STDM_REG_START: c_uint = 0x1242400;
pub const ACP3x_I2STDM_REG_END: c_uint = 0x1242410;
pub const ACP3x_BT_TDM_REG_START: c_uint = 0x1242800;
pub const ACP3x_BT_TDM_REG_END: c_uint = 0x1242810;

pub const ACP_SRAM_SP_PB_PTE_OFFSET: c_uint = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: c_uint = 0x100;
pub const ACP_SRAM_BT_PB_PTE_OFFSET: c_uint = 0x200;
pub const ACP_SRAM_BT_CP_PTE_OFFSET: c_uint = 0x300;
pub const ACP_SRAM_PDM_PTE_OFFSET: c_uint = 0x400;
pub const ACP_SRAM_HS_PB_PTE_OFFSET: c_uint = 0x500;
pub const ACP_SRAM_HS_CP_PTE_OFFSET: c_uint = 0x600;
pub const PAGE_SIZE_4K_ENABLE: c_uint = 0x2;
pub const I2S_SP_TX_MEM_WINDOW_START: c_uint = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: c_uint = 0x4020000;
pub const I2S_BT_TX_MEM_WINDOW_START: c_uint = 0x4040000;
pub const I2S_BT_RX_MEM_WINDOW_START: c_uint = 0x4060000;
pub const I2S_HS_TX_MEM_WINDOW_START: c_uint = 0x40A0000;
pub const I2S_HS_RX_MEM_WINDOW_START: c_uint = 0x40C0000;
pub const ACP7x_I2S_SP_TX_MEM_WINDOW_START: c_uint = 0x4000000;
pub const ACP7x_I2S_SP_RX_MEM_WINDOW_START: c_uint = 0x4200000;
pub const ACP7x_I2S_BT_TX_MEM_WINDOW_START: c_uint = 0x4400000;
pub const ACP7x_I2S_BT_RX_MEM_WINDOW_START: c_uint = 0x4600000;
pub const ACP7x_I2S_HS_TX_MEM_WINDOW_START: c_uint = 0x4800000;
pub const ACP7x_I2S_HS_RX_MEM_WINDOW_START: c_uint = 0x4A00000;
pub const ACP7x_DMIC_MEM_WINDOW_START: c_uint = 0x4C00000;
pub const SP_PB_FIFO_ADDR_OFFSET: c_uint = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: c_uint = 0x700;
pub const BT_PB_FIFO_ADDR_OFFSET: c_uint = 0x900;
pub const BT_CAPT_FIFO_ADDR_OFFSET: c_uint = 0xB00;
pub const HS_PB_FIFO_ADDR_OFFSET: c_uint = 0xD00;
pub const HS_CAPT_FIFO_ADDR_OFFSET: c_uint = 0xF00;
pub const PLAYBACK_MIN_NUM_PERIODS: c_int = 2;
pub const PLAYBACK_MAX_NUM_PERIODS: c_int = 8;
pub const PLAYBACK_MAX_PERIOD_SIZE: c_int = 8192;
pub const PLAYBACK_MIN_PERIOD_SIZE: c_int = 1024;
pub const CAPTURE_MIN_NUM_PERIODS: c_int = 2;
pub const CAPTURE_MAX_NUM_PERIODS: c_int = 8;
pub const CAPTURE_MAX_PERIOD_SIZE: c_int = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: c_int = 1024;
pub const MAX_BUFFER: c_int = 65536;

pub const FIFO_SIZE: c_uint = 0x100;
pub const DMA_SIZE: c_uint = 0x40;
pub const FRM_LEN: c_uint = 0x100;
pub const ACP3x_ITER_IRER_SAMP_LEN_MASK: c_uint = 0x38;
pub const ACP_MAX_STREAM: c_int = 8;
pub const TDM_ENABLE: c_int = 1;
pub const TDM_DISABLE: c_int = 0;
pub const SLOT_WIDTH_8: c_uint = 0x8;
pub const SLOT_WIDTH_16: c_uint = 0x10;
pub const SLOT_WIDTH_24: c_uint = 0x18;
pub const SLOT_WIDTH_32: c_uint = 0x20;
pub const ACP6X_PGFSM_CONTROL: c_uint = 0x1024;
pub const ACP6X_PGFSM_STATUS: c_uint = 0x1028;

pub const ACP_ZSC_DSP_CTRL: c_uint = 0x0001014;
pub const ACP_ZSC_STS: c_uint = 0x0001018;
pub const ACP_SOFT_RST_DONE_MASK: c_uint = 0x00010001;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0xffffffff;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: c_uint = 0x00;
pub const ACP_PGFSM_STATUS_MASK: c_uint = 0x03;
pub const ACP_POWERED_ON: c_uint = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: c_uint = 0x01;
pub const ACP_POWERED_OFF: c_uint = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: c_uint = 0x03;
pub const ACP_ERROR_MASK: c_uint = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: c_uint = 0xffffffff;
pub const ACP_TIMEOUT: c_int = 500;
pub const DELAY_US: c_int = 5;
pub const ACP_SUSPEND_DELAY_MS: c_int = 2000;
pub const PDM_DMA_STAT: c_uint = 0x10;
pub const PDM_DMA_INTR_MASK: c_uint = 0x10000;
pub const PDM_DEC_64: c_uint = 0x2;
pub const PDM_CLK_FREQ_MASK: c_uint = 0x07;
pub const PDM_MISC_CTRL_MASK: c_uint = 0x18;
pub const PDM_ENABLE: c_uint = 0x01;
pub const PDM_DISABLE: c_uint = 0x00;
pub const DMA_EN_MASK: c_uint = 0x02;
pub const DELAY_US: c_int = 5;
pub const PDM_TIMEOUT: c_int = 1000;
pub const ACP_REGION2_OFFSET: c_uint = 0x02000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_chip_info {
    pub /: *mut *mut *mut char name; / Platform name,
    pub res: *mut resource,
    pub dev: *mut device,
    pub dai_driver: *mut snd_soc_dai_driver,
    pub /: *mut *mut unsigned int acp_rev; / ACP Revision id,
    pub /: *mut *mut *mut void __iomem base; / ACP memory PCI base,
    pub acp_hw_ops: *mut snd_acp_hw_ops,
    pub chip): *mut *mut int (acp_hw_ops_init)(struct acp_chip_info,
    pub chip_pdev: *mut platform_device,
    pub resources*/: *mut *mut *mut acp_resource rsrc; / Platform specific,
    pub stream_list: list_head,
    pub /: *mut *mut spinlock_t acp_lock; / Used to protect stream_list,
    pub dmic_codec_dev: *mut platform_device,
    pub acp_plat_dev: *mut platform_device,
    pub mach_dev: *mut platform_device,
    pub machines: *mut snd_soc_acpi_mach,
    pub num_dai: c_int,
    pub addr: u32,
    pub bclk_div: u32,
    pub lrclk_div: u32,
    pub ch_mask: u32,
    pub tdm_tx_fmt: [u32; 3],
    pub tdm_rx_fmt: [u32; 3],
    pub xfer_tx_resolution: [u32; 3],
    pub xfer_rx_resolution: [u32; 3],
    pub /: *mut *mut unsigned int flag; / Distinguish b/w Legacy or Only PDM,
    pub /: *mut *mut bool is_pdm_dev; / flag set to true when ACP PDM controller exists,
    pub /: *mut *mut bool is_pdm_config; / flag set to true when PDM configuration is selected from BIOS,
    pub /: *mut *mut bool is_i2s_config; / flag set to true when I2S configuration is selected from BIOS,
    pub tdm_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_stream {
    pub list: list_head,
    pub substream: *mut snd_pcm_substream,
    pub irq_bit: c_int,
    pub dai_id: c_int,
    pub id: c_int,
    pub dir: c_int,
    pub bytescount: u64,
    pub reg_offset: u32,
    pub pte_offset: u32,
    pub fifo_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_resource {
    pub offset: c_int,
    pub no_of_ctrls: c_int,
    pub irqp_used: c_int,
    pub soc_mclk: bool,
    pub irq_reg_offset: u32,
    pub scratch_reg_offset: u64,
    pub sram_pte_offset: u64,
}

//
// struct snd_acp_hw_ops - ACP PCI driver platform specific ops
// @acp_init: ACP initialization
// @acp_deinit: ACP de-initialization
// @irq: ACP irq handler
// @en_interrupts: ACP enable interrupts
// @dis_interrupts: ACP disable interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_acp_hw_ops {
// ACP hardware initilizations
    pub chip): *mut *mut int (acp_init)(struct acp_chip_info,
    pub chip): *mut *mut int (acp_deinit)(struct acp_chip_info,
// ACP Interrupts
    pub data): *mut *mut irqreturn_t (irq)(int irq, void,
    pub chip): *mut *mut int (en_interrupts)(struct acp_chip_info,
    pub chip): *mut *mut int (dis_interrupts)(struct acp_chip_info,
}

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

extern "C" {
    pub fn acp_platform_register(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acp_platform_unregister(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acp_machine_select(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp_init(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp_deinit(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp_enable_interrupts(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp_disable_interrupts(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn acp31_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp6x_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp63_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
}
extern "C" {
    pub fn acp70_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
}
// Machine configuration
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn config_pte_for_stream(chip: *mut acp_chip_info, stream: *mut acp_stream);
}
extern "C" {
    pub fn config_acp_dma(chip: *mut acp_chip_info, stream: *mut acp_stream, size: c_int);
}
extern "C" {
    pub fn check_acp_config(pci: *mut pci_dev, chip: *mut acp_chip_info);
}
// Get 64 bit value from two 32 bit registers
