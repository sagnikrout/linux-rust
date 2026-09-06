//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/gsc_hwmon.h
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
pub enum gsc_hwmon_mode {
    mode_temperature,
    mode_voltage_24bit,
    mode_voltage_raw,
    mode_voltage_16bit,
    mode_fan,
    mode_max,
}

//
// struct gsc_hwmon_channel - configuration parameters
// @reg:  I2C register offset
// @mode: channel mode
// @name: channel name
// @mvoffset: voltage offset
// @vdiv: voltage divider array (2 resistor values in milli-ohms)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_hwmon_channel {
    pub reg: c_uint,
    pub mode: c_uint,
    pub name: *const c_char,
    pub mvoffset: c_uint,
    pub vdiv: [c_uint; 2],
}

//
// struct gsc_hwmon_platform_data - platform data for gsc_hwmon driver
// @nchannels:	number of elements in @channels array
// @vreference: voltage reference (mV)
// @resolution: ADC bit resolution
// @fan_base: register base for FAN controller
// @channels:	array of gsc_hwmon_channel structures describing channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_hwmon_platform_data {
    pub nchannels: c_int,
    pub resolution: c_uint,
    pub vreference: c_uint,
    pub fan_base: c_uint,
    pub __counted_by(nchannels): gsc_hwmon_channel channels[],
}
