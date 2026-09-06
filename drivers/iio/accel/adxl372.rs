//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/adxl372.h
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
// ADXL371/ADXL372 3-Axis Digital Accelerometer
//
// Copyright 2018 Analog Devices Inc.
//
pub const ADXL372_REVID: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl372_chip_info {
    pub name: *const c_char,
    pub samp_freq_tbl: *const c_int,
    pub bw_freq_tbl: *const c_int,
    pub num_freqs: c_uint,
    pub act_time_scale_us: c_uint,
    pub act_time_scale_low_us: c_uint,
    pub inact_time_scale_ms: c_uint,
    pub inact_time_scale_low_ms: c_uint,
    pub max_odr: c_uint,
    pub fifo_supported: bool,
}

extern "C" {
    pub fn adxl372_readable_noinc_reg(dev: *mut device, reg: c_uint) -> bool;
}
