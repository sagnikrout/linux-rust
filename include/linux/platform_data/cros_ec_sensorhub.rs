//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/cros_ec_sensorhub.h
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
// Chrome OS EC MEMS Sensor Hub driver.
//
// Copyright 2019 Google LLC
//

//
// struct cros_ec_sensor_platform - ChromeOS EC sensor platform information.
// @sensor_num: Id of the sensor, as reported by the EC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensor_platform {
    pub sensor_num: u8,
}

//
// typedef cros_ec_sensorhub_push_data_cb_t - Callback function to send datum
// to specific sensors.
//
// @indio_dev: The IIO device that will process the sample.
// @data: Vector array of the ring sample.
// @timestamp: Timestamp in host timespace when the sample was acquired by
// the EC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensorhub_sensor_push_data {
    pub indio_dev: *mut iio_dev,
    pub push_data_cb: cros_ec_sensorhub_push_data_cb_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensors_ring_sample {
    pub sensor_id: u8,
    pub flag: u8,
    pub vector: [i16; 3],
    pub timestamp: i64,
    pub __packed: },
// State used for cros_ec_ring_fix_overflow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensors_ec_overflow_state {
    pub offset: i64,
    pub last: i64,
}

// Length of the filter, how long to remember entries for
pub const CROS_EC_SENSORHUB_TS_HISTORY_SIZE: c_int = 64;
//
// struct cros_ec_sensors_ts_filter_state - Timestamp filetr state.
//
// @x_offset: x is EC interrupt time. x_offset its last value.
// @y_offset: y is the difference between AP and EC time, y_offset its last
// value.
// @x_history: The past history of x, relative to x_offset.
// @y_history: The past history of y, relative to y_offset.
// @m_history: rate between y and x.
// @history_len: Amount of valid historic data in the arrays.
// @temp_buf: Temporary buffer used when updating the filter.
// @median_m: median value of m_history
// @median_error: final error to apply to AP interrupt timestamp to get the
// "true timestamp" the event occurred.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensors_ts_filter_state {
    pub y_offset: s64 x_offset,,
    pub x_history: [i64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub y_history: [i64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub m_history: [i64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub history_len: c_int,
    pub temp_buf: [i64; CROS_EC_SENSORHUB_TS_HISTORY_SIZE],
    pub median_m: i64,
    pub median_error: i64,
}

// struct cros_ec_sensors_ts_batch_state - State of batch of a single sensor.
//
// Use to store information to batch data using median fileter information.
//
// @penul_ts: last but one batch timestamp (penultimate timestamp).
// Used for timestamp spreading calculations
// when a batch shows up.
// @penul_len: last but one batch length.
// @last_ts: Last batch timestam.
// @last_len: Last batch length.
// @newest_sensor_event: Last sensor timestamp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensors_ts_batch_state {
    pub penul_ts: i64,
    pub penul_len: c_int,
    pub last_ts: i64,
    pub last_len: c_int,
    pub newest_sensor_event: i64,
}

//
// struct cros_ec_sensorhub - Sensor Hub device data.
//
// @dev: Device object, mostly used for logging.
// @ec: Embedded Controller where the hub is located.
// @sensor_num: Number of MEMS sensors present in the EC.
// @msg: Structure to send FIFO requests.
// @params: Pointer to parameters in msg.
// @resp: Pointer to responses in msg.
// @cmd_lock : Lock for sending msg.
// @notifier: Notifier to kick the FIFO interrupt.
// @ring: Preprocessed ring to store events.
// @fifo_timestamp: Array for event timestamp and spreading.
// @fifo_info: Copy of FIFO information coming from the EC.
// @fifo_size: Size of the ring.
// @batch_state: Per sensor information of the last batches received.
// @overflow_a: For handling timestamp overflow for a time (sensor events)
// @overflow_b: For handling timestamp overflow for b time (ec interrupts)
// @filter: Medium fileter structure.
// @tight_timestamps: Set to truen when EC support tight timestamping:
// The timestamps reported from the EC have low jitter.
// Timestamps also come before every sample. Set either
// by feature bits coming from the EC or userspace.
// @future_timestamp_count: Statistics used to compute shaved time.
// This occurs when timestamp interpolation from EC
// time to AP time accidentally puts timestamps in
// the future. These timestamps are clamped to
// `now` and these count/total_ns maintain the
// statistics for how much time was removed in a
// given period.
// @future_timestamp_total_ns: Total amount of time shaved.
// @push_data: Array of callback to send datums to iio sensor object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_sensorhub {
    pub dev: *mut device,
    pub ec: *mut cros_ec_dev,
    pub sensor_num: c_int,
    pub msg: *mut cros_ec_command,
    pub params: *mut ec_params_motion_sense,
    pub resp: *mut ec_response_motion_sense,
    pub /: *mut *mut mutex cmd_lock; / Lock for protecting msg structure.,
    pub notifier: notifier_block,
    pub ring: *mut cros_ec_sensors_ring_sample,
    pub fifo_timestamp: [ktime_t; CROS_EC_SENSOR_ALL_TS],
    pub fifo_info: *mut ec_response_motion_sense_fifo_info,
    pub fifo_size: c_int,
    pub batch_state: *mut cros_ec_sensors_ts_batch_state,
    pub overflow_a: cros_ec_sensors_ec_overflow_state,
    pub overflow_b: cros_ec_sensors_ec_overflow_state,
    pub filter: cros_ec_sensors_ts_filter_state,
    pub tight_timestamps: c_int,
    pub future_timestamp_count: i32,
    pub future_timestamp_total_ns: i64,
    pub push_data: *mut cros_ec_sensorhub_sensor_push_data,
}

extern "C" {
    pub fn cros_ec_sensorhub_ring_allocate(sensorhub: *mut cros_ec_sensorhub) -> c_int;
}
extern "C" {
    pub fn cros_ec_sensorhub_ring_add(sensorhub: *mut cros_ec_sensorhub) -> c_int;
}
extern "C" {
    pub fn cros_ec_sensorhub_ring_remove(arg: *mut c_void);
}
