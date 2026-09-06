//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/types.h
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
// industrial I/O data types needed both in and out of kernel
//
// Copyright (c) 2008 Jonathan Cameron
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_event_info {
    IIO_EV_INFO_ENABLE,
    IIO_EV_INFO_VALUE,
    IIO_EV_INFO_HYSTERESIS,
    IIO_EV_INFO_PERIOD,
    IIO_EV_INFO_HIGH_PASS_FILTER_3DB,
    IIO_EV_INFO_LOW_PASS_FILTER_3DB,
    IIO_EV_INFO_TIMEOUT,
    IIO_EV_INFO_RESET_TIMEOUT,
    IIO_EV_INFO_TAP2_MIN_DELAY,
    IIO_EV_INFO_RUNNING_PERIOD,
    IIO_EV_INFO_RUNNING_COUNT,
    IIO_EV_INFO_SCALE,
}

pub const IIO_VAL_INT: c_int = 1;
pub const IIO_VAL_INT_PLUS_MICRO: c_int = 2;
pub const IIO_VAL_INT_PLUS_NANO: c_int = 3;
pub const IIO_VAL_INT_PLUS_MICRO_DB: c_int = 4;
pub const IIO_VAL_INT_MULTIPLE: c_int = 5;

pub const IIO_VAL_FRACTIONAL: c_int = 10;
pub const IIO_VAL_FRACTIONAL_LOG2: c_int = 11;
pub const IIO_VAL_CHAR: c_int = 12;
pub const IIO_VAL_DECIMAL64_BASE: c_int = 32;

// val0 = lower_32_bits(dec64);
// val1 = upper_32_bits(dec64);
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_available_type {
    IIO_AVAIL_LIST,
    IIO_AVAIL_RANGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_chan_info_enum {
    IIO_CHAN_INFO_RAW = 0,
    IIO_CHAN_INFO_PROCESSED,
    IIO_CHAN_INFO_SCALE,
    IIO_CHAN_INFO_OFFSET,
    IIO_CHAN_INFO_CALIBSCALE,
    IIO_CHAN_INFO_CALIBBIAS,
    IIO_CHAN_INFO_PEAK,
    IIO_CHAN_INFO_PEAK_SCALE,
    IIO_CHAN_INFO_QUADRATURE_CORRECTION_RAW,
    IIO_CHAN_INFO_AVERAGE_RAW,
    IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY,
    IIO_CHAN_INFO_HIGH_PASS_FILTER_3DB_FREQUENCY,
    IIO_CHAN_INFO_SAMP_FREQ,
    IIO_CHAN_INFO_FREQUENCY,
    IIO_CHAN_INFO_PHASE,
    IIO_CHAN_INFO_HARDWAREGAIN,
    IIO_CHAN_INFO_HYSTERESIS,
    IIO_CHAN_INFO_HYSTERESIS_RELATIVE,
    IIO_CHAN_INFO_INT_TIME,
    IIO_CHAN_INFO_ENABLE,
    IIO_CHAN_INFO_CALIBHEIGHT,
    IIO_CHAN_INFO_CALIBWEIGHT,
    IIO_CHAN_INFO_DEBOUNCE_COUNT,
    IIO_CHAN_INFO_DEBOUNCE_TIME,
    IIO_CHAN_INFO_CALIBEMISSIVITY,
    IIO_CHAN_INFO_OVERSAMPLING_RATIO,
    IIO_CHAN_INFO_THERMOCOUPLE_TYPE,
    IIO_CHAN_INFO_CALIBAMBIENT,
    IIO_CHAN_INFO_ZEROPOINT,
    IIO_CHAN_INFO_TROUGH,
    IIO_CHAN_INFO_CONVDELAY,
    IIO_CHAN_INFO_POWERFACTOR,
}
