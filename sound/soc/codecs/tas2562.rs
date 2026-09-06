//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2562.h
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
// tas2562.h - ALSA SoC Texas Instruments TAS2562 Mono Audio Amplifier
//
// Copyright (C) 2019 Texas Instruments Incorporated -  https://www.ti.com
//
// Author: Dan Murphy <dmurphy@ti.com>
//
pub const TAS2562_PAGE_CTRL: c_uint = 0x00;

pub const TAS2562_RIGHT_SLOT_SHIFT: c_int = 4;
// Page 2

pub const TAS2562_ACTIVE: c_uint = 0x0;
pub const TAS2562_MUTE: c_uint = 0x1;
pub const TAS2562_SHUTDOWN: c_uint = 0x2;

pub const TAS2562_TDM_CFG1_RX_FALLING: c_int = 1;

pub const TAS2562_TDM_CFG2_RXLEN_16B: c_uint = 0x0;

pub const TAS2562_TDM_CFG2_RXWLEN_16B: c_uint = 0x0;

pub const TAS2562_VSENSE_POWER_EN: c_int = 2;
pub const TAS2562_ISENSE_POWER_EN: c_int = 3;

