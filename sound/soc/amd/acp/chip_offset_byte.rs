//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp/chip_offset_byte.h
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
pub const ACPAXI2AXI_ATU_CTRL: c_uint = 0xC40;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: c_uint = 0xC00;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: c_uint = 0xC04;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2: c_uint = 0xC08;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_2: c_uint = 0xC0C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5: c_uint = 0xC20;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_5: c_uint = 0xC24;
pub const GRP1_OFFSET: c_uint = 0x0;
pub const GRP2_OFFSET: c_uint = 0x4000;
pub const ACP_PGFSM_CONTROL: c_uint = 0x141C;
pub const ACP_PGFSM_STATUS: c_uint = 0x1420;
pub const ACP_SOFT_RESET: c_uint = 0x1000;
pub const ACP_CONTROL: c_uint = 0x1004;
pub const ACP_PIN_CONFIG: c_uint = 0x1440;
pub const ACP3X_PIN_CONFIG: c_uint = 0x1400;

// Registers from ACP_AUDIO_BUFFERS block

pub const ACP_HS_RX_RINGBUFADDR: c_uint = 0x3A90;
pub const ACP_HS_RX_RINGBUFSIZE: c_uint = 0x3A94;
pub const ACP_HS_RX_LINKPOSITIONCNTR: c_uint = 0x3A98;
pub const ACP_HS_RX_FIFOADDR: c_uint = 0x3A9C;
pub const ACP_HS_RX_FIFOSIZE: c_uint = 0x3AA0;
pub const ACP_HS_RX_DMA_SIZE: c_uint = 0x3AA4;
pub const ACP_HS_RX_LINEARPOSITIONCNTR_HIGH: c_uint = 0x3AA8;
pub const ACP_HS_RX_LINEARPOSITIONCNTR_LOW: c_uint = 0x3AAC;
pub const ACP_HS_RX_INTR_WATERMARK_SIZE: c_uint = 0x3AB0;
pub const ACP_HS_TX_RINGBUFADDR: c_uint = 0x3AB4;
pub const ACP_HS_TX_RINGBUFSIZE: c_uint = 0x3AB8;
pub const ACP_HS_TX_LINKPOSITIONCNTR: c_uint = 0x3ABC;
pub const ACP_HS_TX_FIFOADDR: c_uint = 0x3AC0;
pub const ACP_HS_TX_FIFOSIZE: c_uint = 0x3AC4;
pub const ACP_HS_TX_DMA_SIZE: c_uint = 0x3AC8;
pub const ACP_HS_TX_LINEARPOSITIONCNTR_HIGH: c_uint = 0x3ACC;
pub const ACP_HS_TX_LINEARPOSITIONCNTR_LOW: c_uint = 0x3AD0;
pub const ACP_HS_TX_INTR_WATERMARK_SIZE: c_uint = 0x3AD4;
pub const ACP_I2STDM_IER: c_uint = 0x2400;
pub const ACP_I2STDM_IRER: c_uint = 0x2404;
pub const ACP_I2STDM_RXFRMT: c_uint = 0x2408;
pub const ACP_I2STDM_ITER: c_uint = 0x240C;
pub const ACP_I2STDM_TXFRMT: c_uint = 0x2410;
// Registers from ACP_BT_TDM block
pub const ACP_BTTDM_IER: c_uint = 0x2800;
pub const ACP_BTTDM_IRER: c_uint = 0x2804;
pub const ACP_BTTDM_RXFRMT: c_uint = 0x2808;
pub const ACP_BTTDM_ITER: c_uint = 0x280C;
pub const ACP_BTTDM_TXFRMT: c_uint = 0x2810;
// Registers from ACP_HS_TDM block
pub const ACP_HSTDM_IER: c_uint = 0x2814;
pub const ACP_HSTDM_IRER: c_uint = 0x2818;
pub const ACP_HSTDM_RXFRMT: c_uint = 0x281C;
pub const ACP_HSTDM_ITER: c_uint = 0x2820;
pub const ACP_HSTDM_TXFRMT: c_uint = 0x2824;
// Registers from ACP_WOV_PDM block
pub const ACP_WOV_PDM_ENABLE: c_uint = 0x2C04;
pub const ACP_WOV_PDM_DMA_ENABLE: c_uint = 0x2C08;
pub const ACP_WOV_RX_RINGBUFADDR: c_uint = 0x2C0C;
pub const ACP_WOV_RX_RINGBUFSIZE: c_uint = 0x2C10;
pub const ACP_WOV_RX_LINKPOSITIONCNTR: c_uint = 0x2C14;
pub const ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH: c_uint = 0x2C18;
pub const ACP_WOV_RX_LINEARPOSITIONCNTR_LOW: c_uint = 0x2C1C;
pub const ACP_WOV_RX_INTR_WATERMARK_SIZE: c_uint = 0x2C20;
pub const ACP_WOV_PDM_FIFO_FLUSH: c_uint = 0x2C24;
pub const ACP_WOV_PDM_NO_OF_CHANNELS: c_uint = 0x2C28;
pub const ACP_WOV_PDM_DECIMATION_FACTOR: c_uint = 0x2C2C;
pub const ACP_WOV_PDM_VAD_CTRL: c_uint = 0x2C30;
pub const ACP_WOV_BUFFER_STATUS: c_uint = 0x2C58;
pub const ACP_WOV_MISC_CTRL: c_uint = 0x2C5C;
pub const ACP_WOV_CLK_CTRL: c_uint = 0x2C60;
pub const ACP_PDM_VAD_DYNAMIC_CLK_GATING_EN: c_uint = 0x2C64;
pub const ACP_WOV_ERROR_STATUS_REGISTER: c_uint = 0x2C68;
pub const ACP_I2STDM0_MSTRCLKGEN: c_uint = 0x2414;
pub const ACP_I2STDM1_MSTRCLKGEN: c_uint = 0x2418;
pub const ACP_I2STDM2_MSTRCLKGEN: c_uint = 0x241C;
