//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/common/cros_ec_sensors_core.h
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
// ChromeOS EC sensor hub
//
// Copyright (C) 2016 Google, Inc
//

// EC returns sensor values using signed 16 bit registers
pub const CROS_EC_SENSOR_BITS: c_int = 16;
//
// 4 16 bit channels are allowed.
// Good enough for current sensors, they use up to 3 16 bit vectors.
//

extern "C" {
    pub fn irqreturn_t(irq: *mut *mut cros_ec_sensors_capture_t)(int, p: *mut c_void) -> typedef;
}
//
// struct cros_ec_sensors_core_state - state data for EC sensors IIO driver
// @ec:				cros EC device structure
// @cmd_lock:			lock used to prevent simultaneous access to the
// commands.
// @msg:			cros EC command structure
// @param:			motion sensor parameters structure
// @resp:			motion sensor response structure
// @type:			type of motion sensor
// @range_updated:		True if the range of the sensor has been
// updated.
// @curr_range:			If updated, the current range value.
// It will be reapplied at every resume.
// @calib:			calibration parameters. Note that trigger
// captured data will always provide the calibrated
// data
// @samples:			static array to hold data from a single capture.
// For each channel we need 2 bytes, except for
// the timestamp. The timestamp is always last and
// is always 8-byte aligned.
// @read_ec_sensors_data:	function used for accessing sensors values
// @fifo_max_event_count:	Size of the EC sensor FIFO
// @frequencies:		Table of known available frequencies:
// 0, Min and Max in mHz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensors_core_state {
    pub ec: *mut cros_ec_device,
    pub cmd_lock: mutex,
    pub msg: *mut cros_ec_command,
    pub param: ec_params_motion_sense,
    pub resp: *mut ec_response_motion_sense,
    pub type: motionsensor_type,
    pub range_updated: bool,
    pub curr_range: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct calib_data {
    pub offset: i16,
    pub scale: u16,
    pub calib: [}; CROS_EC_SENSOR_MAX_AXIS],
    pub sign: [i8; CROS_EC_SENSOR_MAX_AXIS],
    pub __aligned(8): u8 samples[CROS_EC_SAMPLE_SIZE],
    pub data): *mut unsigned long scan_mask, s16,
    pub fifo_max_event_count: u32,
    pub frequencies: [c_int; 6],
}

extern "C" {
    pub fn cros_ec_sensors_capture(irq: c_int, p: *mut c_void) -> irqreturn_t;
}
// List of extended channel specification for all sensors.
