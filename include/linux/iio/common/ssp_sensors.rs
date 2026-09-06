//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/common/ssp_sensors.h
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
// Copyright (C) 2014, Samsung Electronics Co. Ltd. All Rights Reserved.
//

pub const SSP_TIME_SIZE: c_int = 4;
pub const SSP_ACCELEROMETER_SIZE: c_int = 6;
pub const SSP_GYROSCOPE_SIZE: c_int = 6;
pub const SSP_BIO_HRM_RAW_SIZE: c_int = 8;
pub const SSP_BIO_HRM_RAW_FAC_SIZE: c_int = 36;
pub const SSP_BIO_HRM_LIB_SIZE: c_int = 8;
//
// enum ssp_sensor_type - SSP sensor type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssp_sensor_type {
    SSP_ACCELEROMETER_SENSOR = 0,
    SSP_GYROSCOPE_SENSOR,
    SSP_GEOMAGNETIC_UNCALIB_SENSOR,
    SSP_GEOMAGNETIC_RAW,
    SSP_GEOMAGNETIC_SENSOR,
    SSP_PRESSURE_SENSOR,
    SSP_GESTURE_SENSOR,
    SSP_PROXIMITY_SENSOR,
    SSP_TEMPERATURE_HUMIDITY_SENSOR,
    SSP_LIGHT_SENSOR,
    SSP_PROXIMITY_RAW,
    SSP_ORIENTATION_SENSOR,
    SSP_STEP_DETECTOR,
    SSP_SIG_MOTION_SENSOR,
    SSP_GYRO_UNCALIB_SENSOR,
    SSP_GAME_ROTATION_VECTOR,
    SSP_ROTATION_VECTOR,
    SSP_STEP_COUNTER,
    SSP_BIO_HRM_RAW,
    SSP_BIO_HRM_RAW_FAC,
    SSP_BIO_HRM_LIB,
    SSP_SENSOR_MAX,
}

//
// struct ssp_sensor_data - Sensor object
// @process_data:	Callback to feed sensor data.
// @type:		Used sensor type.
// @buffer:		Received data buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_sensor_data {
    pub timestamp): i64,
    pub type: ssp_sensor_type,
    pub buffer: *mut u8,
}

extern "C" {
    pub fn ssp_disable_sensor(data: *mut ssp_data, type: ssp_sensor_type) -> c_int;
}
extern "C" {
    pub fn ssp_get_sensor_delay(data: *mut ssp_data, ssp_sensor_type: enum) -> u32;
}
