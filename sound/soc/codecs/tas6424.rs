//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas6424.h
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
// ALSA SoC Texas Instruments TAS6424 Quad-Channel Audio Amplifier
//
// Copyright (C) 2016-2017 Texas Instruments Incorporated - https://www.ti.com
// Author: Andreas Dannenberg <dannenberg@ti.com>
// Andrew F. Davis <afd@ti.com>
//

// Register Address Map
pub const TAS6424_MODE_CTRL: c_uint = 0x00;
pub const TAS6424_MISC_CTRL1: c_uint = 0x01;
pub const TAS6424_MISC_CTRL2: c_uint = 0x02;
pub const TAS6424_SAP_CTRL: c_uint = 0x03;
pub const TAS6424_CH_STATE_CTRL: c_uint = 0x04;
pub const TAS6424_CH1_VOL_CTRL: c_uint = 0x05;
pub const TAS6424_CH2_VOL_CTRL: c_uint = 0x06;
pub const TAS6424_CH3_VOL_CTRL: c_uint = 0x07;
pub const TAS6424_CH4_VOL_CTRL: c_uint = 0x08;
pub const TAS6424_DC_DIAG_CTRL1: c_uint = 0x09;
pub const TAS6424_DC_DIAG_CTRL2: c_uint = 0x0a;
pub const TAS6424_DC_DIAG_CTRL3: c_uint = 0x0b;
pub const TAS6424_DC_LOAD_DIAG_REP12: c_uint = 0x0c;
pub const TAS6424_DC_LOAD_DIAG_REP34: c_uint = 0x0d;
pub const TAS6424_DC_LOAD_DIAG_REPLO: c_uint = 0x0e;
pub const TAS6424_CHANNEL_STATE: c_uint = 0x0f;
pub const TAS6424_CHANNEL_FAULT: c_uint = 0x10;
pub const TAS6424_GLOB_FAULT1: c_uint = 0x11;
pub const TAS6424_GLOB_FAULT2: c_uint = 0x12;
pub const TAS6424_WARN: c_uint = 0x13;
pub const TAS6424_PIN_CTRL: c_uint = 0x14;
pub const TAS6424_AC_DIAG_CTRL1: c_uint = 0x15;
pub const TAS6424_AC_DIAG_CTRL2: c_uint = 0x16;
pub const TAS6424_AC_LOAD_DIAG_REP1: c_uint = 0x17;
pub const TAS6424_AC_LOAD_DIAG_REP2: c_uint = 0x18;
pub const TAS6424_AC_LOAD_DIAG_REP3: c_uint = 0x19;
pub const TAS6424_AC_LOAD_DIAG_REP4: c_uint = 0x1a;
pub const TAS6424_MISC_CTRL3: c_uint = 0x21;
pub const TAS6424_CLIP_CTRL: c_uint = 0x22;
pub const TAS6424_CLIP_WINDOW: c_uint = 0x23;
pub const TAS6424_CLIP_WARN: c_uint = 0x24;
pub const TAS6424_CBC_STAT: c_uint = 0x25;
pub const TAS6424_MISC_CTRL4: c_uint = 0x26;

// TAS6424_MODE_CTRL_REG

// TAS6424_SAP_CTRL_REG

// TAS6424_CH_STATE_CTRL_REG

// TAS6424_DC_DIAG_CTRL1
pub const TAS6424_LDGBYPASS_SHIFT: c_int = 0;

// TAS6424_GLOB_FAULT1_REG

// TAS6424_GLOB_FAULT1_REG

// TAS6424_GLOB_FAULT2_REG

// TAS6424_WARN_REG

// TAS6424_MISC_CTRL3_REG

