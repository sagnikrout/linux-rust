//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mxs/mxs-saif.h
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
// Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
//
pub const SAIF_CTRL: c_uint = 0x0;
pub const SAIF_STAT: c_uint = 0x10;
pub const SAIF_DATA: c_uint = 0x20;

// SAIF_CTRL
pub const BM_SAIF_CTRL_SFTRST: c_uint = 0x80000000;
pub const BM_SAIF_CTRL_CLKGATE: c_uint = 0x40000000;
pub const BP_SAIF_CTRL_BITCLK_MULT_RATE: c_int = 27;
pub const BM_SAIF_CTRL_BITCLK_MULT_RATE: c_uint = 0x38000000;

pub const BM_SAIF_CTRL_BITCLK_BASE_RATE: c_uint = 0x04000000;
pub const BM_SAIF_CTRL_FIFO_ERROR_IRQ_EN: c_uint = 0x02000000;
pub const BM_SAIF_CTRL_FIFO_SERVICE_IRQ_EN: c_uint = 0x01000000;
pub const BP_SAIF_CTRL_RSRVD2: c_int = 21;
pub const BM_SAIF_CTRL_RSRVD2: c_uint = 0x00E00000;
pub const BP_SAIF_CTRL_DMAWAIT_COUNT: c_int = 16;
pub const BM_SAIF_CTRL_DMAWAIT_COUNT: c_uint = 0x001F0000;

pub const BP_SAIF_CTRL_CHANNEL_NUM_SELECT: c_int = 14;
pub const BM_SAIF_CTRL_CHANNEL_NUM_SELECT: c_uint = 0x0000C000;

pub const BM_SAIF_CTRL_LRCLK_PULSE: c_uint = 0x00002000;
pub const BM_SAIF_CTRL_BIT_ORDER: c_uint = 0x00001000;
pub const BM_SAIF_CTRL_DELAY: c_uint = 0x00000800;
pub const BM_SAIF_CTRL_JUSTIFY: c_uint = 0x00000400;
pub const BM_SAIF_CTRL_LRCLK_POLARITY: c_uint = 0x00000200;
pub const BM_SAIF_CTRL_BITCLK_EDGE: c_uint = 0x00000100;
pub const BP_SAIF_CTRL_WORD_LENGTH: c_int = 4;
pub const BM_SAIF_CTRL_WORD_LENGTH: c_uint = 0x000000F0;

pub const BM_SAIF_CTRL_BITCLK_48XFS_ENABLE: c_uint = 0x00000008;
pub const BM_SAIF_CTRL_SLAVE_MODE: c_uint = 0x00000004;
pub const BM_SAIF_CTRL_READ_MODE: c_uint = 0x00000002;
pub const BM_SAIF_CTRL_RUN: c_uint = 0x00000001;
// SAIF_STAT
pub const BM_SAIF_STAT_PRESENT: c_uint = 0x80000000;
pub const BP_SAIF_STAT_RSRVD2: c_int = 17;
pub const BM_SAIF_STAT_RSRVD2: c_uint = 0x7FFE0000;

pub const BM_SAIF_STAT_DMA_PREQ: c_uint = 0x00010000;
pub const BP_SAIF_STAT_RSRVD1: c_int = 7;
pub const BM_SAIF_STAT_RSRVD1: c_uint = 0x0000FF80;

pub const BM_SAIF_STAT_FIFO_UNDERFLOW_IRQ: c_uint = 0x00000040;
pub const BM_SAIF_STAT_FIFO_OVERFLOW_IRQ: c_uint = 0x00000020;
pub const BM_SAIF_STAT_FIFO_SERVICE_IRQ: c_uint = 0x00000010;
pub const BP_SAIF_STAT_RSRVD0: c_int = 1;
pub const BM_SAIF_STAT_RSRVD0: c_uint = 0x0000000E;

pub const BM_SAIF_STAT_BUSY: c_uint = 0x00000001;
// SAFI_DATA
pub const BP_SAIF_DATA_PCM_RIGHT: c_int = 16;
pub const BM_SAIF_DATA_PCM_RIGHT: c_uint = 0xFFFF0000;

pub const BP_SAIF_DATA_PCM_LEFT: c_int = 0;
pub const BM_SAIF_DATA_PCM_LEFT: c_uint = 0x0000FFFF;

// SAIF_VERSION
pub const BP_SAIF_VERSION_MAJOR: c_int = 24;
pub const BM_SAIF_VERSION_MAJOR: c_uint = 0xFF000000;

pub const BP_SAIF_VERSION_MINOR: c_int = 16;
pub const BM_SAIF_VERSION_MINOR: c_uint = 0x00FF0000;

pub const BP_SAIF_VERSION_STEP: c_int = 0;
pub const BM_SAIF_VERSION_STEP: c_uint = 0x0000FFFF;

pub const MXS_SAIF_MCLK: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_saif {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub mclk: c_uint,
    pub mclk_in_use: c_uint,
    pub base: *mut void __iomem,
    pub id: c_uint,
    pub master_id: c_uint,
    pub cur_rate: c_uint,
    pub ongoing: c_uint,
    pub fifo_underrun: u32,
    pub fifo_overrun: u32,
    pub state: },
}

extern "C" {
    pub fn mxs_saif_put_mclk(saif_id: c_uint) -> c_int;
}
