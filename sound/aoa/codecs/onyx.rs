//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/codecs/onyx.h
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
// Apple Onboard Audio driver for Onyx codec (header)
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

// PCM3052 register definitions
// the attenuation registers take values from
// -1 (0dB) to -127 (-63.0 dB) or others (muted)
pub const ONYX_REG_DAC_ATTEN_LEFT: c_int = 65;

pub const ONYX_REG_DAC_ATTEN_RIGHT: c_int = 66;
pub const ONYX_REG_CONTROL: c_int = 67;

// all others reserved
pub const ONYX_REG_DAC_CONTROL: c_int = 68;

pub const ONYX_REG_DAC_DEEMPH: c_int = 69;

pub const ONYX_REG_DAC_FILTER: c_int = 70;

pub const ONYX_REG_DAC_OUTPHASE: c_int = 71;

pub const ONYX_REG_ADC_CONTROL: c_int = 72;

// 8 + input gain in dB, valid range for input gain is -4 .. 20 dB

pub const ONYX_REG_ADC_HPF_BYPASS: c_int = 75;

pub const ONYX_REG_DIG_INFO1: c_int = 77;

// bits 1-5 control channel bits 1-5

pub const ONYX_REG_DIG_INFO2: c_int = 78;
// controls channel bits 8-15
pub const ONYX_REG_DIG_INFO3: c_int = 79;
// control channel bits 24-29, high 2 bits reserved
pub const ONYX_REG_DIG_INFO4: c_int = 80;

// lower 4 bits control bits 32-35 of channel control and word length

