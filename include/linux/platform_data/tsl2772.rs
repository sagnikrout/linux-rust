//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/tsl2772.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Device driver for monitoring ambient light intensity (lux)
// and proximity (prox) within the TAOS TSL2772 family of devices.
//
// Copyright (c) 2012, TAOS Corporation.
// Copyright (c) 2017-2018 Brian Masney <masneyb@onstation.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2772_lux {
    pub ch0: c_uint,
    pub ch1: c_uint,
}

// Max number of segments allowable in LUX table
pub const TSL2772_MAX_LUX_TABLE_SIZE: c_int = 6;
// The default LUX tables all have 3 elements.
pub const TSL2772_DEF_LUX_TABLE_SZ: c_int = 3;

// Proximity diode to use
pub const TSL2772_DIODE0: c_uint = 0x01;
pub const TSL2772_DIODE1: c_uint = 0x02;
pub const TSL2772_DIODE_BOTH: c_uint = 0x03;
// LED Power
pub const TSL2772_100_mA: c_uint = 0x00;
pub const TSL2772_50_mA: c_uint = 0x01;
pub const TSL2772_25_mA: c_uint = 0x02;
pub const TSL2772_13_mA: c_uint = 0x03;
//
// struct tsl2772_settings - Settings for the tsl2772 driver
// @als_time:              Integration time of the ALS channel ADCs in 2.73 ms
// increments. Total integration time is
// (256 - als_time) * 2.73.
// @als_gain:              Index into the tsl2772_als_gain array.
// @als_gain_trim:         Default gain trim to account for aperture effects.
// @wait_time:             Time between proximity and ALS cycles in 2.73
// periods.
// @prox_time:             Integration time of the proximity ADC in 2.73 ms
// increments. Total integration time is
// (256 - prx_time) * 2.73.
// @prox_gain:             Index into the tsl2772_prx_gain array.
// @als_prox_config:       The value of the ALS / Proximity configuration
// register.
// @als_cal_target:        Known external ALS reading for calibration.
// @als_persistence:       H/W Filters, Number of 'out of limits' ALS readings.
// @als_interrupt_en:      Enable/Disable ALS interrupts
// @als_thresh_low:        CH0 'low' count to trigger interrupt.
// @als_thresh_high:       CH0 'high' count to trigger interrupt.
// @prox_persistence:      H/W Filters, Number of 'out of limits' proximity
// readings.
// @prox_interrupt_en:     Enable/Disable proximity interrupts.
// @prox_thres_low:        Low threshold proximity detection.
// @prox_thres_high:       High threshold proximity detection.
// @prox_pulse_count:      Number if proximity emitter pulses.
// @prox_max_samples_cal:  The number of samples that are taken when performing
// a proximity calibration.
// @prox_diode:            Which diode(s) to use for driving the external
// LED(s) for proximity sensing.
// @prox_power:            The amount of power to use for the external LED(s).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2772_settings {
    pub als_time: c_int,
    pub als_gain: c_int,
    pub als_gain_trim: c_int,
    pub wait_time: c_int,
    pub prox_time: c_int,
    pub prox_gain: c_int,
    pub als_prox_config: c_int,
    pub als_cal_target: c_int,
    pub als_persistence: u8,
    pub als_interrupt_en: bool,
    pub als_thresh_low: c_int,
    pub als_thresh_high: c_int,
    pub prox_persistence: u8,
    pub prox_interrupt_en: bool,
    pub prox_thres_low: c_int,
    pub prox_thres_high: c_int,
    pub prox_pulse_count: c_int,
    pub prox_max_samples_cal: c_int,
    pub prox_diode: c_int,
    pub prox_power: c_int,
}

//
// struct tsl2772_platform_data - Platform callback, glass and defaults
// @platform_lux_table:        Device specific glass coefficents
// @platform_default_settings: Device specific power on defaults
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl2772_platform_data {
    pub platform_lux_table: [tsl2772_lux; TSL2772_MAX_LUX_TABLE_SIZE],
    pub platform_default_settings: *mut tsl2772_settings,
}
