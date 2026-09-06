//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/chemical/scd30.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scd30_cmd {
// start continuous measurement with pressure compensation
    CMD_START_MEAS,
// stop continuous measurement
    CMD_STOP_MEAS,
// set/get measurement interval
    CMD_MEAS_INTERVAL,
// check whether new measurement is ready
    CMD_MEAS_READY,
// get measurement
    CMD_READ_MEAS,
// turn on/off automatic self calibration
    CMD_ASC,
// set/get forced recalibration value
    CMD_FRC,
// set/get temperature offset
    CMD_TEMP_OFFSET,
// get firmware version
    CMD_FW_VERSION,
// reset sensor
    CMD_RESET,
//
// Command for altitude compensation was omitted intentionally because
// the same can be achieved by means of CMD_START_MEAS which takes
// pressure above the sea level as an argument.
//
}

pub const SCD30_MEAS_COUNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scd30_state {
// serialize access to the device
    pub lock: mutex,
    pub dev: *mut device,
    pub vdd: *mut regulator,
    pub meas_ready: completion,
//
// priv pointer is solely for serdev driver private data. We keep it
// here because driver_data inside dev has been already used for iio and
// struct serdev_device doesn't have one.
//
    pub priv: *mut c_void,
    pub irq: c_int,
//
// no way to retrieve current ambient pressure compensation value from
// the sensor so keep one around
//
    pub pressure_comp: u16,
    pub meas_interval: u16,
    pub meas: [c_int; SCD30_MEAS_COUNT],
    pub command: scd30_command_t,
}

extern "C" {
    pub fn scd30_probe(dev: *mut device, irq: c_int, name: *const c_char, priv: *mut c_void, command: scd30_command_t) -> c_int;
}
