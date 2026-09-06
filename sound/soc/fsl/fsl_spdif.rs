//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_spdif.h
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
// fsl_spdif.h - ALSA S/PDIF interface for the Freescale i.MX SoC
//
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <b42378@freescale.com>
//
// Based on fsl_ssi.h
// Author: Timur Tabi <timur@freescale.com>
// Copyright 2007-2008 Freescale Semiconductor, Inc.
//
// S/PDIF Register Map
pub const REG_SPDIF_SCR: c_uint = 0x0	/* SPDIF Configuration Register */;
pub const REG_SPDIF_SRCD: c_uint = 0x4	/* CDText Control Register */;
pub const REG_SPDIF_SRPC: c_uint = 0x8	/* PhaseConfig Register */;
pub const REG_SPDIF_SIE: c_uint = 0xc	/* InterruptEn Register */;
pub const REG_SPDIF_SIS: c_uint = 0x10	/* InterruptStat Register */;
pub const REG_SPDIF_SIC: c_uint = 0x10	/* InterruptClear Register */;
pub const REG_SPDIF_SRL: c_uint = 0x14	/* SPDIFRxLeft Register */;
pub const REG_SPDIF_SRR: c_uint = 0x18	/* SPDIFRxRight Register */;
pub const REG_SPDIF_SRCSH: c_uint = 0x1c	/* SPDIFRxCChannel_h Register */;
pub const REG_SPDIF_SRCSL: c_uint = 0x20	/* SPDIFRxCChannel_l Register */;
pub const REG_SPDIF_SRU: c_uint = 0x24	/* UchannelRx Register */;
pub const REG_SPDIF_SRQ: c_uint = 0x28	/* QchannelRx Register */;
pub const REG_SPDIF_STL: c_uint = 0x2C	/* SPDIFTxLeft Register */;
pub const REG_SPDIF_STR: c_uint = 0x30	/* SPDIFTxRight Register */;
pub const REG_SPDIF_STCSCH: c_uint = 0x34	/* SPDIFTxCChannelCons_h Register */;
pub const REG_SPDIF_STCSCL: c_uint = 0x38	/* SPDIFTxCChannelCons_l Register */;
pub const REG_SPDIF_STCSPH: c_uint = 0x3C	/* SPDIFTxCChannel_Prof_h Register */;
pub const REG_SPDIF_STCSPL: c_uint = 0x40	/* SPDIFTxCChannel_Prof_l Register */;
pub const REG_SPDIF_SRFM: c_uint = 0x44	/* FreqMeas Register */;
pub const REG_SPDIF_STC: c_uint = 0x50	/* SPDIFTxClk Register */;
pub const REG_SPDIF_SRCCA_31_0: c_uint = 0x60	/* SPDIF receive C channel register, bits 31-0 */;
pub const REG_SPDIF_SRCCA_63_32: c_uint = 0x64	/* SPDIF receive C channel register, bits 63-32 */;
pub const REG_SPDIF_SRCCA_95_64: c_uint = 0x68	/* SPDIF receive C channel register, bits 95-64 */;
pub const REG_SPDIF_SRCCA_127_96: c_uint = 0x6C	/* SPDIF receive C channel register, bits 127-96 */;
pub const REG_SPDIF_SRCCA_159_128: c_uint = 0x70	/* SPDIF receive C channel register, bits 159-128 */;
pub const REG_SPDIF_SRCCA_191_160: c_uint = 0x74	/* SPDIF receive C channel register, bits 191-160 */;
pub const REG_SPDIF_STCCA_31_0: c_uint = 0x78	/* SPDIF transmit C channel register, bits 31-0 */;
pub const REG_SPDIF_STCCA_63_32: c_uint = 0x7C	/* SPDIF transmit C channel register, bits 63-32 */;
pub const REG_SPDIF_STCCA_95_64: c_uint = 0x80	/* SPDIF transmit C channel register, bits 95-64 */;
pub const REG_SPDIF_STCCA_127_96: c_uint = 0x84	/* SPDIF transmit C channel register, bits 127-96 */;
pub const REG_SPDIF_STCCA_159_128: c_uint = 0x88	/* SPDIF transmit C channel register, bits 159-128 */;
pub const REG_SPDIF_STCCA_191_160: c_uint = 0x8C	/* SPDIF transmit C channel register, bits 191-160 */;
// SPDIF Configuration register
pub const SCR_RXFIFO_CTL_OFFSET: c_int = 23;

pub const SCR_RXFIFO_OFF_OFFSET: c_int = 22;

pub const SCR_RXFIFO_RST_OFFSET: c_int = 21;

pub const SCR_RXFIFO_FSEL_OFFSET: c_int = 19;

pub const SCR_RXFIFO_AUTOSYNC_OFFSET: c_int = 18;

pub const SCR_TXFIFO_AUTOSYNC_OFFSET: c_int = 17;

pub const SCR_TXFIFO_FSEL_OFFSET: c_int = 15;

pub const SCR_TXFIFO_CTRL_OFFSET: c_int = 10;

pub const SCR_DMA_RX_EN_OFFSET: c_int = 9;

pub const SCR_DMA_TX_EN_OFFSET: c_int = 8;

pub const SCR_VAL_OFFSET: c_int = 5;

pub const SCR_TXSEL_OFFSET: c_int = 2;

pub const SCR_USRC_SEL_OFFSET: c_uint = 0x0;

// SPDIF CDText control
pub const SRCD_CD_USER_OFFSET: c_int = 1;

// SPDIF Phase Configuration register

pub const SRPC_CLKSRC_SEL_OFFSET: c_int = 7;

pub const SRPC_CLKSRC_SEL_LOCKED_OFFSET1: c_int = 5;
pub const SRPC_CLKSRC_SEL_LOCKED_OFFSET2: c_int = 2;
pub const SRPC_GAINSEL_OFFSET: c_int = 3;

pub const SRPC_CLKSRC_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spdif_gainsel {
    GAINSEL_MULTI_24 = 0,
    GAINSEL_MULTI_16,
    GAINSEL_MULTI_12,
    GAINSEL_MULTI_8,
    GAINSEL_MULTI_6,
    GAINSEL_MULTI_4,
    GAINSEL_MULTI_3,
}

// SPDIF interrupt mask define

// SPDIF Clock register
pub const STC_SYSCLK_DF_OFFSET: c_int = 11;

pub const STC_TXCLK_SRC_OFFSET: c_int = 8;

pub const STC_TXCLK_ALL_EN_OFFSET: c_int = 7;

pub const STC_TXCLK_DF_OFFSET: c_int = 0;

pub const STC_TXCLK_SRC_MAX: c_int = 8;
pub const STC_TXCLK_SPDIF_ROOT: c_int = 1;
// SPDIF tx rate
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spdif_txrate {
    SPDIF_TXRATE_22050 = 0,
    SPDIF_TXRATE_32000,
    SPDIF_TXRATE_44100,
    SPDIF_TXRATE_48000,
    SPDIF_TXRATE_88200,
    SPDIF_TXRATE_96000,
    SPDIF_TXRATE_176400,
    SPDIF_TXRATE_192000,
}

pub const SPDIF_CSTATUS_BYTE: c_int = 6;
pub const SPDIF_UBITS_SIZE: c_int = 96;

