//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/rockchip/rockchip_sai.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA SoC Audio Layer - Rockchip SAI Controller driver
//
// Copyright (c) 2022 Rockchip Electronics Co. Ltd.
//
// XCR Transmit / Receive Control Register

pub const SAI_XCR_START_SEL_STANDALONE: c_int = 0;

pub const SAI_XCR_EDGE_SHIFT_0: c_int = 0;

pub const SAI_XCR_SJM_R: c_int = 0;

pub const SAI_XCR_FBM_MSB: c_int = 0;

pub const SAI_XCR_VDJ_R: c_int = 0;

// FSCR Frame Sync Control Register

pub const SAI_FSCR_EDGE_RISING: c_int = 0;

// MONO_CR Mono Control Register

pub const SAI_MCR_RX_MONO_DIS: c_int = 0;

pub const SAI_MCR_TX_MONO_DIS: c_int = 0;
// XFER Transfer Start Register

//
// Used for TX only (VERSION >= SAI_VER_2311)
//
// SCLK/FSYNC auto gated when TX FIFO empty.
//

pub const SAI_XFER_TX_AUTO_DIS: c_int = 0;

pub const SAI_XFER_RX_CNT_DIS: c_int = 0;

pub const SAI_XFER_TX_CNT_DIS: c_int = 0;

pub const SAI_XFER_RXS_DIS: c_int = 0;

pub const SAI_XFER_TXS_DIS: c_int = 0;

pub const SAI_XFER_FSS_DIS: c_int = 0;

pub const SAI_XFER_CLK_DIS: c_int = 0;
// CLR Clear Logic Register

// CKR Clock Generation Register

pub const SAI_CKR_MSS_MASTER: c_int = 0;

pub const SAI_CKR_CKP_NORMAL: c_int = 0;

pub const SAI_CKR_FSP_NORMAL: c_int = 0;
// DMACR DMA Control Register

// INTCR Interrupt Ctrl Register

// INTSR Interrupt Status Register
pub const SAI_INTSR_FSLOSTI_INA: c_int = 0;

pub const SAI_INTSR_FSERRI_INA: c_int = 0;

pub const SAI_INTSR_RXOI_INA: c_int = 0;

pub const SAI_INTSR_TXUI_INA: c_int = 0;

// PATH_SEL: Transfer / Receive Path Select Register

// XSHIFT: Transfer / Receive Frame Sync Shift Register
//
// TX-ONLY: LEFT Direction Feature
// +------------------------------------------------+
// | DATA LEFTx (step: 0.5 cycle) | FSYNC Edge      |
// +------------------------------------------------+
//

//
// +------------------------------------------------+
// | FSYNC Edge | DATA RIGHTx (step: 0.5 cycle)     |
// +------------------------------------------------+
//

// XFIFOLR: Transfer / Receive FIFO Level Register
pub const SAI_FIFOLR_XFL3_SHIFT: c_int = 18;

pub const SAI_FIFOLR_XFL2_SHIFT: c_int = 12;

pub const SAI_FIFOLR_XFL1_SHIFT: c_int = 6;

pub const SAI_FIFOLR_XFL0_SHIFT: c_int = 0;

// STATUS Status Register (VERSION >= SAI_VER_2307)

// VERSION
//
// Updates:
//
// VERSION >= SAI_VER_2311
//
// Support Frame Sync xN (FSXN)
// Support Frame Sync Error Detect (FSE)
// Support Frame Sync Lost Detect (FSLOST)
// Support Force Clear (FCR)
// Support SAIn-Chained (e.g. SAI0-CLK-DATA + SAI3-DATA +...)
// Support Transmit Auto Gate Mode
// Support Timing Shift Left for TX
//
// Optimize SCLK/FSYNC Timing Alignment
//
// VERSION >= SAI_VER_2403
//
// Support Loopback LR Select (e.g. L:MIC R:LP)
//
pub const SAI_VER_2307: c_uint = 0x23073576;
pub const SAI_VER_2311: c_uint = 0x23112118;
pub const SAI_VER_2401: c_uint = 0x24013506;
pub const SAI_VER_2403: c_uint = 0x24031103;
// FS_TIMEOUT: Frame Sync Timeout Register

// SAI Registers

