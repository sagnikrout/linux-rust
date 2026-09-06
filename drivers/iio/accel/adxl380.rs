//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/adxl380.h
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
// ADXL380 3-Axis Digital Accelerometer
//
// Copyright 2024 Analog Devices Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adxl380_odr {
    ADXL380_ODR_VLP,
    ADXL380_ODR_DSM,
    ADXL380_ODR_DSM_2X,
    ADXL380_ODR_DSM_4X,
    ADXL380_ODR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl380_chip_info {
    pub name: *const c_char,
    pub scale_tbl: [c_int; 3][2],
    pub samp_freq_tbl: [c_int; ADXL380_ODR_MAX],
    pub info: *const iio_info,
    pub temp_offset: c_int,
    pub chip_id: u16,
    pub has_low_power: bool,
}

extern "C" {
    pub fn adxl380_readable_noinc_reg(dev: *mut device, reg: c_uint) -> bool;
}
