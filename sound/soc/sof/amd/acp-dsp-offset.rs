//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/amd/acp-dsp-offset.h
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
// Copyright(c) 2021, 2023, 2024 Advanced Micro Devices, Inc. All rights reserved.
//
// Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//
// Registers from ACP_DMA_0 block
pub const ACP_DMA_CNTL_0: c_uint = 0x00;
pub const ACP_DMA_DSCR_STRT_IDX_0: c_uint = 0x20;
pub const ACP_DMA_DSCR_CNT_0: c_uint = 0x40;
pub const ACP_DMA_PRIO_0: c_uint = 0x60;
pub const ACP_DMA_CUR_DSCR_0: c_uint = 0x80;
pub const ACP_DMA_ERR_STS_0: c_uint = 0xC0;
pub const ACP_DMA_DESC_BASE_ADDR: c_uint = 0xE0;
pub const ACP_DMA_DESC_MAX_NUM_DSCR: c_uint = 0xE4;
pub const ACP_DMA_CH_STS: c_uint = 0xE8;
pub const ACP_DMA_CH_GROUP: c_uint = 0xEC;
pub const ACP_DMA_CH_RST_STS: c_uint = 0xF0;
pub const ACP70_DMA_CNTL_0: c_uint = 0x00;
pub const ACP70_DMA_DSCR_STRT_IDX_0: c_uint = 0x28;
pub const ACP70_DMA_DSCR_CNT_0: c_uint = 0x50;
pub const ACP70_DMA_PRIO_0: c_uint = 0x78;
pub const ACP70_DMA_CUR_DSCR_0: c_uint = 0xA0;
pub const ACP70_DMA_ERR_STS_0: c_uint = 0xF0;
pub const ACP70_DMA_DESC_BASE_ADDR: c_uint = 0x118;
pub const ACP70_DMA_DESC_MAX_NUM_DSCR: c_uint = 0x11C;
pub const ACP70_DMA_CH_STS: c_uint = 0x120;
pub const ACP70_DMA_CH_GROUP: c_uint = 0x124;
pub const ACP70_DMA_CH_RST_STS: c_uint = 0x128;
// Registers from ACP_DSP_0 block
pub const ACP_DSP0_RUNSTALL: c_uint = 0x414;
// Registers from ACP_AXI2AXIATU block
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: c_uint = 0xC00;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: c_uint = 0xC04;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2: c_uint = 0xC08;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_2: c_uint = 0xC0C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_3: c_uint = 0xC10;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_3: c_uint = 0xC14;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_4: c_uint = 0xC18;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_4: c_uint = 0xC1C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5: c_uint = 0xC20;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_5: c_uint = 0xC24;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_6: c_uint = 0xC28;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_6: c_uint = 0xC2C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_7: c_uint = 0xC30;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_7: c_uint = 0xC34;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_8: c_uint = 0xC38;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_8: c_uint = 0xC3C;
pub const ACPAXI2AXI_ATU_CTRL: c_uint = 0xC40;
pub const ACP_SOFT_RESET: c_uint = 0x1000;
pub const ACP_CONTROL: c_uint = 0x1004;
pub const ACP3X_I2S_PIN_CONFIG: c_uint = 0x1400;
pub const ACP5X_I2S_PIN_CONFIG: c_uint = 0x1400;
pub const ACP6X_I2S_PIN_CONFIG: c_uint = 0x1440;
// Registers offsets from ACP_PGFSM block
pub const ACP3X_PGFSM_BASE: c_uint = 0x141C;
pub const ACP5X_PGFSM_BASE: c_uint = 0x1424;
pub const ACP6X_PGFSM_BASE: c_uint = 0x1024;

pub const PGFSM_CONTROL_OFFSET: c_uint = 0x0;
pub const PGFSM_STATUS_OFFSET: c_uint = 0x4;
pub const ACP3X_CLKMUX_SEL: c_uint = 0x1424;
pub const ACP5X_CLKMUX_SEL: c_uint = 0x142C;
pub const ACP6X_CLKMUX_SEL: c_uint = 0x102C;

// Registers from ACP_INTR block
pub const ACP3X_EXT_INTR_STAT: c_uint = 0x1808;
pub const ACP5X_EXT_INTR_STAT: c_uint = 0x1808;
pub const ACP6X_EXTERNAL_INTR_ENB: c_uint = 0x1A00;
pub const ACP6X_EXTERNAL_INTR_CNTL: c_uint = 0x1A04;
pub const ACP6X_EXT_INTR_STAT: c_uint = 0x1A0C;
pub const ACP6X_EXT_INTR_STAT1: c_uint = 0x1A10;

pub const ACP7X_EXTERNAL_INTR_CNTL: c_uint = 0x1A04;
pub const ACP7X_EXT_INTR_STAT: c_uint = 0x1A1C;
pub const ACP7X_EXTERNAL_INTR_CNTL1: c_uint = 0x1A08;
pub const ACP7X_EXT_INTR_STAT1: c_uint = 0x1A20;
pub const ACP3X_DSP_SW_INTR_BASE: c_uint = 0x1814;
pub const ACP5X_DSP_SW_INTR_BASE: c_uint = 0x1814;
pub const ACP6X_DSP_SW_INTR_BASE: c_uint = 0x1808;

pub const ACP7X_DSP_SW_INTR_BASE: c_uint = 0x1860;
pub const DSP_SW_INTR_CNTL_OFFSET: c_uint = 0x0;
pub const DSP_SW_INTR_STAT_OFFSET: c_uint = 0x4;

pub const DSP_SW_INTR_TRIG_OFFSET: c_uint = 0x8;
pub const ACP7X_DSP_SW_INTR_TRIG_OFFSET: c_uint = 0x30;
pub const ACP3X_ERROR_STATUS: c_uint = 0x18C4;
pub const ACP6X_ERROR_STATUS: c_uint = 0x1A4C;

pub const ACP7X_ERROR_STATUS: c_uint = 0x1A88;
pub const ACP3X_AXI2DAGB_SEM_0: c_uint = 0x1880;
pub const ACP5X_AXI2DAGB_SEM_0: c_uint = 0x1884;
pub const ACP6X_AXI2DAGB_SEM_0: c_uint = 0x1874;

pub const ACP7X_AXI2DAGB_SEM_0: c_uint = 0x18F4;
// ACP common registers to report errors related to I2S & SoundWire interfaces
pub const ACP3X_SW_I2S_ERROR_REASON: c_uint = 0x18C8;
pub const ACP6X_SW0_I2S_ERROR_REASON: c_uint = 0x18B4;

pub const ACP_SW1_I2S_ERROR_REASON: c_uint = 0x1A50;
// Registers from ACP_SHA block
pub const ACP_SHA_DSP_FW_QUALIFIER: c_uint = 0x1C70;
pub const ACP_SHA_DMA_CMD: c_uint = 0x1CB0;
pub const ACP_SHA_MSG_LENGTH: c_uint = 0x1CB4;
pub const ACP_SHA_DMA_STRT_ADDR: c_uint = 0x1CB8;
pub const ACP_SHA_DMA_DESTINATION_ADDR: c_uint = 0x1CBC;
pub const ACP_SHA_DMA_CMD_STS: c_uint = 0x1CC0;
pub const ACP_SHA_DMA_ERR_STATUS: c_uint = 0x1CC4;
pub const ACP_SHA_TRANSFER_BYTE_CNT: c_uint = 0x1CC8;
pub const ACP_SHA_DMA_INCLUDE_HDR: c_uint = 0x1CCC;
pub const ACP_SHA_PSP_ACK: c_uint = 0x1C74;
pub const ACP_SCRATCH_REG_0: c_uint = 0x10000;
pub const ACP6X_DSP_FUSION_RUNSTALL: c_uint = 0x0644;

// Cache window registers
pub const ACP_DSP0_CACHE_OFFSET0: c_uint = 0x0420;
pub const ACP_DSP0_CACHE_SIZE0: c_uint = 0x0424;
pub const ACP_SW0_EN: c_uint = 0x3000;
pub const ACP_SW1_EN: c_uint = 0x3C00;
pub const ACP70_PME_EN: c_uint = 0x1400;
pub const ACP70_EXTERNAL_INTR_CNTL1: c_uint = 0x1A08;
pub const ACP70_SW0_WAKE_EN: c_uint = 0x1458;
pub const ACP70_SW1_WAKE_EN: c_uint = 0x1460;
pub const ACP70_SDW_HOST_WAKE_MASK: c_uint = 0x0C00000;

pub const ACP7X_DSP0_IDMA_ERROR_MASK: c_uint = 0x4B0;
pub const ACP7X_IDMA_ERROR_MASK: c_uint = 0x1FF9FF;
pub const ACP7X_ZSC_DSP_CTRL: c_uint = 0x001014;

