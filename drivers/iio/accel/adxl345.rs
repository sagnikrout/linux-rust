//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/adxl345.h
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
// ADXL345 3-Axis Digital Accelerometer
//
// Copyright (c) 2017 Eva Rachel Retuya <eraretuya@gmail.com>
//
pub const ADXL345_REG_DEVID: c_uint = 0x00;
pub const ADXL345_REG_THRESH_TAP: c_uint = 0x1D;
pub const ADXL345_REG_OFSX: c_uint = 0x1E;
pub const ADXL345_REG_OFSY: c_uint = 0x1F;
pub const ADXL345_REG_OFSZ: c_uint = 0x20;

// Tap duration
pub const ADXL345_REG_DUR: c_uint = 0x21;
// Tap latency
pub const ADXL345_REG_LATENT: c_uint = 0x22;
// Tap window
pub const ADXL345_REG_WINDOW: c_uint = 0x23;
// Activity threshold
pub const ADXL345_REG_THRESH_ACT: c_uint = 0x24;
// Inactivity threshold
pub const ADXL345_REG_THRESH_INACT: c_uint = 0x25;
// Inactivity time
pub const ADXL345_REG_TIME_INACT: c_uint = 0x26;
// Axis enable control for activity and inactivity detection
pub const ADXL345_REG_ACT_INACT_CTRL: c_uint = 0x27;
// Free-fall threshold
pub const ADXL345_REG_THRESH_FF: c_uint = 0x28;
// Free-fall time
pub const ADXL345_REG_TIME_FF: c_uint = 0x29;
// Axis control for single tap or double tap
pub const ADXL345_REG_TAP_AXIS: c_uint = 0x2A;
// Source of single tap or double tap
pub const ADXL345_REG_ACT_TAP_STATUS: c_uint = 0x2B;
// Data rate and power mode control
pub const ADXL345_REG_BW_RATE: c_uint = 0x2C;
pub const ADXL345_REG_POWER_CTL: c_uint = 0x2D;
pub const ADXL345_REG_INT_ENABLE: c_uint = 0x2E;
pub const ADXL345_REG_INT_MAP: c_uint = 0x2F;
pub const ADXL345_REG_INT_SOURCE: c_uint = 0x30;
pub const ADXL345_REG_DATA_FORMAT: c_uint = 0x31;
pub const ADXL345_REG_XYZ_BASE: c_uint = 0x32;

pub const ADXL345_REG_FIFO_CTL: c_uint = 0x38;

// 0: INT1, 1: INT2

pub const ADXL345_REG_FIFO_STATUS: c_uint = 0x39;
pub const ADXL345_REG_FIFO_STATUS_MSK: c_uint = 0x3F;

//
// BW_RATE bits - Bandwidth and output data rate. The default value is
// 0x0A, which translates to a 100 Hz output data rate
//

// Set the g range

// Data is left justified

// Up to 13-bits resolution

pub const ADXL345_DATA_FORMAT_2G: c_int = 0;
pub const ADXL345_DATA_FORMAT_4G: c_int = 1;
pub const ADXL345_DATA_FORMAT_8G: c_int = 2;
pub const ADXL345_DATA_FORMAT_16G: c_int = 3;
pub const ADXL345_DEVID: c_uint = 0xE5;
pub const ADXL345_FIFO_SIZE: c_int = 32;
//
// In full-resolution mode, scale factor is maintained at ~4 mg/LSB
// in all g ranges.
//
// At +/- 16g with 13-bit resolution, scale is computed as:
// (16 + 16) * 9.81 / (2^13 - 1) = 0.0383
//
pub const ADXL345_USCALE: c_int = 38300;
//
// The Datasheet lists a resolution of Resolution is ~49 mg per LSB. That's
// ~480mm/s**2 per LSB.
//
pub const ADXL375_USCALE: c_int = 480000;
extern "C" {
    pub fn adxl345_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl345_chip_info {
    pub name: *const c_char,
    pub uscale: c_int,
}
