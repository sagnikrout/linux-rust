//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/led-lm3530.h
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
// Copyright (C) 2011 ST-Ericsson SA.
// Copyright (C) 2009 Motorola, Inc.
//
// Simple driver for National Semiconductor LM35330 Backlight driver chip
//
// Author: Shreshtha Kumar SAHU <shreshthakumar.sahu@stericsson.com>
// based on leds-lm3530.c by Dan Murphy <D.Murphy@motorola.com>
//

// ALS Resistor Select

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3530_mode {
    LM3530_BL_MODE_MANUAL = 0,	/* "man" */
    LM3530_BL_MODE_ALS,		/* "als" */
    LM3530_BL_MODE_PWM,		/* "pwm" */
}

// ALS input select
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3530_als_mode {
    LM3530_INPUT_AVRG = 0,	/* ALS1 and ALS2 input average */
    LM3530_INPUT_ALS1,	/* ALS1 Input */
    LM3530_INPUT_ALS2,	/* ALS2 Input */
    LM3530_INPUT_CEIL,	/* Max of ALS1 and ALS2 */
}

// PWM Platform Specific Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3530_pwm_data {
    pub max_brightness): *mut *mut void (pwm_set_intensity) (int brightness, int,
    pub max_brightness): *mut *mut int (pwm_get_intensity) (int,
}

//
// struct lm3530_platform_data
// @mode: mode of operation i.e. Manual, ALS or PWM
// @als_input_mode: select source of ALS input - ALS1/2 or average
// @max_current: full scale LED current
// @pwm_pol_hi: PWM input polarity - active high/active low
// @als_avrg_time: ALS input averaging time
// @brt_ramp_law: brightness mapping mode - exponential/linear
// @brt_ramp_fall: rate of fall of led current
// @brt_ramp_rise: rate of rise of led current
// @als1_resistor_sel: internal resistance from ALS1 input to ground
// @als2_resistor_sel: internal resistance from ALS2 input to ground
// @als_vmin: als input voltage calibrated for max brightness in mV
// @als_vmax: als input voltage calibrated for min brightness in mV
// @brt_val: brightness value (0-127)
// @pwm_data: PWM control functions (only valid when the mode is PWM)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3530_platform_data {
    pub mode: lm3530_mode,
    pub als_input_mode: lm3530_als_mode,
    pub max_current: u8,
    pub pwm_pol_hi: bool,
    pub als_avrg_time: u8,
    pub brt_ramp_law: bool,
    pub brt_ramp_fall: u8,
    pub brt_ramp_rise: u8,
    pub als1_resistor_sel: u8,
    pub als2_resistor_sel: u8,
    pub als_vmin: u32,
    pub als_vmax: u32,
    pub brt_val: u8,
    pub pwm_data: lm3530_pwm_data,
}
