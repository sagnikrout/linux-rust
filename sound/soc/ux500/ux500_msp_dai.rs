//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ux500/ux500_msp_dai.h
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
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>,
// Roger Nilsson <roger.xr.nilsson@stericsson.com>
// for ST-Ericsson.
//

// Macro flag: #define UX500_msp_dai_H

pub const UX500_NBR_OF_DAI: c_int = 4;

pub const FRAME_PER_SINGLE_SLOT_8_KHZ: c_int = 31;
pub const FRAME_PER_SINGLE_SLOT_16_KHZ: c_int = 124;
pub const FRAME_PER_SINGLE_SLOT_44_1_KHZ: c_int = 63;
pub const FRAME_PER_SINGLE_SLOT_48_KHZ: c_int = 49;
pub const FRAME_PER_2_SLOTS: c_int = 31;
pub const FRAME_PER_8_SLOTS: c_int = 138;
pub const FRAME_PER_16_SLOTS: c_int = 277;
pub const UX500_MSP_INTERNAL_CLOCK_FREQ: c_int = 40000000;

pub const UX500_MSP_MIN_CHANNELS: c_int = 1;
pub const UX500_MSP_MAX_CHANNELS: c_int = 8;
pub const PLAYBACK_CONFIGURED: c_int = 1;
pub const CAPTURE_CONFIGURED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ux500_msp_clock_id {
    UX500_MSP_MASTER_CLOCK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_msp_i2s_drvdata {
    pub msp: *mut ux500_msp,
    pub reg_vape: *mut regulator,
    pub fmt: c_uint,
    pub tx_mask: c_uint,
    pub rx_mask: c_uint,
    pub slots: c_int,
    pub slot_width: c_int,
// Clocks
    pub master_clk: c_uint,
    pub clk: *mut clk,
    pub pclk: *mut clk,
// Regulators
    pub vape_opp_constraint: c_int,
}

extern "C" {
    pub fn ux500_msp_dai_set_data_delay(dai: *mut snd_soc_dai, delay: c_int) -> c_int;
}
