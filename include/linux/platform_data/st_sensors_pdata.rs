//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/st_sensors_pdata.h
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
// STMicroelectronics sensors platform-data driver
//
// Copyright 2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//
// struct st_sensors_platform_data - Platform data for the ST sensors
// @drdy_int_pin: Redirect DRDY on pin 1 (1) or pin 2 (2).
// Available only for accelerometer, magnetometer and pressure sensors.
// Accelerometer DRDY on LSM330 available only on pin 1 (see datasheet).
// Magnetometer DRDY is supported only on LSM9DS0 and LSM303D.
// @open_drain: set the interrupt line to be open drain if possible.
// @spi_3wire: enable spi-3wire mode.
// @pullups: enable/disable i2c controller pullup resistors.
// @wakeup_source: enable/disable device as wakeup generator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensors_platform_data {
    pub drdy_int_pin: u8,
    pub open_drain: bool,
    pub spi_3wire: bool,
    pub pullups: bool,
    pub wakeup_source: bool,
}
