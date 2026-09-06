//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/iio-gts-helper.h
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
// gain-time-scale conversion helpers for IIO light sensors
//
// Copyright (c) 2023 Matti Vaittinen <mazziesaccount@gmail.com>
//

//
// struct iio_gain_sel_pair - gain - selector values
//
// In many cases devices like light sensors allow setting signal amplification
// (gain) using a register interface. This structure describes amplification
// and corresponding selector (register value)
//
// @gain:	Gain (multiplication) value. Gain must be positive, negative
// values are reserved for error handling.
// @sel:	Selector (usually register value) used to indicate this gain.
// NOTE: Only selectors >= 0 supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_gain_sel_pair {
    pub gain: c_int,
    pub sel: c_int,
}

//
// struct iio_itime_sel_mul - integration time description
//
// In many cases devices like light sensors allow setting the duration of
// collecting data. Typically this duration has also an impact to the magnitude
// of measured values (gain). This structure describes the relation of
// integration time and amplification as well as corresponding selector
// (register value).
//
// An example could be a sensor allowing 50, 100, 200 and 400 mS times. The
// respective multiplication values could be 50 mS => 1, 100 mS => 2,
// 200 mS => 4 and 400 mS => 8 assuming the impact of integration time would be
// linear in a way that when collecting data for 50 mS caused value X, doubling
// the data collection time caused value 2X etc.
//
// @time_us:	Integration time in microseconds. Time values must be positive,
// negative values are reserved for error handling.
// @sel:	Selector (usually register value) used to indicate this time
// NOTE: Only selectors >= 0 supported.
// @mul:	Multiplication to the values caused by this time.
// NOTE: Only multipliers > 0 supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_itime_sel_mul {
    pub time_us: c_int,
    pub sel: c_int,
    pub mul: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_gts {
    pub max_scale: u64,
    pub hwgain_table: *const iio_gain_sel_pair,
    pub num_hwgain: c_int,
    pub itime_table: *const iio_itime_sel_mul,
    pub num_itime: c_int,
    pub per_time_avail_scale_tables: *mut c_int,
    pub avail_all_scales_table: *mut c_int,
    pub num_avail_all_scales: c_int,
    pub avail_time_tables: *mut c_int,
    pub num_avail_time_tables: c_int,
}

//
// iio_gts_find_int_time_by_sel - find integration time matching a selector
// @gts:	Gain time scale descriptor
// @sel:	selector for which matching integration time is searched for
//
// Return:	integration time matching given selector or -EINVAL if
// integration time was not found.
//
// iio_gts_find_sel_by_int_time - find selector matching integration time
// @gts:	Gain time scale descriptor
// @time:	Integration time for which matching selector is searched for
//
// Return:	a selector matching given integration time or -EINVAL if
// selector was not found.
//
// iio_gts_valid_time - check if given integration time is valid
// @gts:	Gain time scale descriptor
// @time_us:	Integration time to check
//
// Return:	True if given time is supported by device. False if not.
//
extern "C" {
    pub fn iio_gts_find_sel_by_gain(gts: *mut iio_gts, gain: c_int) -> c_int;
}
//
// iio_gts_valid_gain - check if given HW-gain is valid
// @gts:	Gain time scale descriptor
// @gain:	HW-gain to check
//
// Return:	True if given time is supported by device. False if not.
//
extern "C" {
    pub fn iio_find_closest_gain_low(gts: *mut iio_gts, gain: c_int, in_range: *mut bool) -> c_int;
}
extern "C" {
    pub fn iio_gts_find_gain_by_sel(gts: *mut iio_gts, sel: c_int) -> c_int;
}
extern "C" {
    pub fn iio_gts_get_min_gain(gts: *mut iio_gts) -> c_int;
}
extern "C" {
    pub fn iio_gts_find_int_time_by_sel(gts: *mut iio_gts, sel: c_int) -> c_int;
}
extern "C" {
    pub fn iio_gts_find_sel_by_int_time(gts: *mut iio_gts, time: c_int) -> c_int;
}
extern "C" {
    pub fn iio_gts_get_total_gain(gts: *mut iio_gts, gain: c_int, time: c_int) -> c_int;
}
