//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/cs42l42.h
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
// cs42l42.h -- CS42L42 ALSA SoC audio driver DT bindings header
//
// Copyright 2016 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
// Author: Brian Austin <brian.austin@cirrus.com>
// Author: Michael White <michael.white@cirrus.com>
//
// HPOUT Load Capacity
pub const CS42L42_HPOUT_LOAD_1NF: c_int = 0;
pub const CS42L42_HPOUT_LOAD_10NF: c_int = 1;
// HPOUT Clamp to GND Override
pub const CS42L42_HPOUT_CLAMP_EN: c_int = 0;
pub const CS42L42_HPOUT_CLAMP_DIS: c_int = 1;
// Tip Sense Inversion
pub const CS42L42_TS_INV_DIS: c_int = 0;
pub const CS42L42_TS_INV_EN: c_int = 1;
// Tip Sense Debounce
pub const CS42L42_TS_DBNCE_0: c_int = 0;
pub const CS42L42_TS_DBNCE_125: c_int = 1;
pub const CS42L42_TS_DBNCE_250: c_int = 2;
pub const CS42L42_TS_DBNCE_500: c_int = 3;
pub const CS42L42_TS_DBNCE_750: c_int = 4;
pub const CS42L42_TS_DBNCE_1000: c_int = 5;
pub const CS42L42_TS_DBNCE_1250: c_int = 6;
pub const CS42L42_TS_DBNCE_1500: c_int = 7;
// Button Press Software Debounce Times
pub const CS42L42_BTN_DET_INIT_DBNCE_MIN: c_int = 0;
pub const CS42L42_BTN_DET_INIT_DBNCE_DEFAULT: c_int = 100;
pub const CS42L42_BTN_DET_INIT_DBNCE_MAX: c_int = 200;
pub const CS42L42_BTN_DET_EVENT_DBNCE_MIN: c_int = 0;
pub const CS42L42_BTN_DET_EVENT_DBNCE_DEFAULT: c_int = 10;
pub const CS42L42_BTN_DET_EVENT_DBNCE_MAX: c_int = 20;
// Button Detect Level Sensitivities
pub const CS42L42_NUM_BIASES: c_int = 4;
pub const CS42L42_HS_DET_LEVEL_15: c_uint = 0x0F;
pub const CS42L42_HS_DET_LEVEL_8: c_uint = 0x08;
pub const CS42L42_HS_DET_LEVEL_4: c_uint = 0x04;
pub const CS42L42_HS_DET_LEVEL_1: c_uint = 0x01;
pub const CS42L42_HS_DET_LEVEL_MIN: c_int = 0;
pub const CS42L42_HS_DET_LEVEL_MAX: c_uint = 0x3F;
// HS Bias Ramp Rate
pub const CS42L42_HSBIAS_RAMP_FAST_RISE_SLOW_FALL: c_int = 0;
pub const CS42L42_HSBIAS_RAMP_FAST: c_int = 1;
pub const CS42L42_HSBIAS_RAMP_SLOW: c_int = 2;
pub const CS42L42_HSBIAS_RAMP_SLOWEST: c_int = 3;
pub const CS42L42_HSBIAS_RAMP_TIME0: c_int = 10;
pub const CS42L42_HSBIAS_RAMP_TIME1: c_int = 40;
pub const CS42L42_HSBIAS_RAMP_TIME2: c_int = 90;
pub const CS42L42_HSBIAS_RAMP_TIME3: c_int = 170;
