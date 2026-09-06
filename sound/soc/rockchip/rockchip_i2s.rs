//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/rockchip/rockchip_i2s.h
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
// sound/soc/rockchip/rockchip_i2s.h
//
// ALSA SoC Audio Layer - Rockchip I2S Controller driver
//
// Copyright (c) 2014 Rockchip Electronics Co. Ltd.
// Author: Jianqun xu <jay.xu@rock-chips.com>
//
// TXCR
// transmit operation control register
//
pub const I2S_TXCR_RCNT_SHIFT: c_int = 17;

pub const I2S_TXCR_CSR_SHIFT: c_int = 15;

pub const I2S_TXCR_SJM_SHIFT: c_int = 12;

pub const I2S_TXCR_FBM_SHIFT: c_int = 11;

pub const I2S_TXCR_IBM_SHIFT: c_int = 9;

pub const I2S_TXCR_PBM_SHIFT: c_int = 7;

pub const I2S_TXCR_TFS_SHIFT: c_int = 5;

pub const I2S_TXCR_VDW_SHIFT: c_int = 0;

//
// RXCR
// receive operation control register
//
pub const I2S_RXCR_CSR_SHIFT: c_int = 15;

pub const I2S_RXCR_SJM_SHIFT: c_int = 12;

pub const I2S_RXCR_FBM_SHIFT: c_int = 11;

pub const I2S_RXCR_IBM_SHIFT: c_int = 9;

pub const I2S_RXCR_PBM_SHIFT: c_int = 7;

pub const I2S_RXCR_TFS_SHIFT: c_int = 5;

pub const I2S_RXCR_VDW_SHIFT: c_int = 0;

//
// CKR
// clock generation register
//
pub const I2S_CKR_TRCM_SHIFT: c_int = 28;

pub const I2S_CKR_MSS_SHIFT: c_int = 27;

pub const I2S_CKR_CKP_SHIFT: c_int = 26;

pub const I2S_CKR_RLP_SHIFT: c_int = 25;

pub const I2S_CKR_TLP_SHIFT: c_int = 24;

pub const I2S_CKR_MDIV_SHIFT: c_int = 16;

pub const I2S_CKR_RSD_SHIFT: c_int = 8;

pub const I2S_CKR_TSD_SHIFT: c_int = 0;

//
// FIFOLR
// FIFO level register
//
pub const I2S_FIFOLR_RFL_SHIFT: c_int = 24;

pub const I2S_FIFOLR_TFL3_SHIFT: c_int = 18;

pub const I2S_FIFOLR_TFL2_SHIFT: c_int = 12;

pub const I2S_FIFOLR_TFL1_SHIFT: c_int = 6;

pub const I2S_FIFOLR_TFL0_SHIFT: c_int = 0;

//
// DMACR
// DMA control register
//
pub const I2S_DMACR_RDE_SHIFT: c_int = 24;

pub const I2S_DMACR_RDL_SHIFT: c_int = 16;

pub const I2S_DMACR_TDE_SHIFT: c_int = 8;

pub const I2S_DMACR_TDL_SHIFT: c_int = 0;

//
// INTCR
// interrupt control register
//
pub const I2S_INTCR_RFT_SHIFT: c_int = 20;

pub const I2S_INTCR_RXOIE_SHIFT: c_int = 17;

pub const I2S_INTCR_RXFIE_SHIFT: c_int = 16;

pub const I2S_INTCR_TFT_SHIFT: c_int = 4;

pub const I2S_INTCR_TXUIE_SHIFT: c_int = 1;

//
// INTSR
// interrupt status register
//
pub const I2S_INTSR_TXEIE_SHIFT: c_int = 0;

pub const I2S_INTSR_RXOI_SHIFT: c_int = 17;

pub const I2S_INTSR_RXFI_SHIFT: c_int = 16;

pub const I2S_INTSR_TXUI_SHIFT: c_int = 1;

pub const I2S_INTSR_TXEI_SHIFT: c_int = 0;

//
// XFER
// Transfer start register
//
pub const I2S_XFER_RXS_SHIFT: c_int = 1;

pub const I2S_XFER_TXS_SHIFT: c_int = 0;

//
// CLR
// clear SCLK domain logic register
//

//
// TXDR
// Transimt FIFO data register, write only.
//

//
// RXDR
// Receive FIFO data register, write only.
//

// Clock divider id
// channel select
pub const I2S_CSR_SHIFT: c_int = 15;

// I2S REGS

// io direction cfg register

