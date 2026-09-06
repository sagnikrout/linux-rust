//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/common/ssp_sensors/ssp_iio_sensor.h
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

pub const SSP_MS_PER_S: c_int = 1000;

extern "C" {
    pub fn ssp_common_buffer_postenable(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn ssp_common_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int;
}
// Converts time in ms to frequency
// fractional = 0;
// integer_part = 0;
// integer_part = SSP_FACTOR_WITH_MS / time;
// fractional = *integer_part % SSP_INVERTED_SCALING_FACTOR;
// integer_part = *integer_part / SSP_INVERTED_SCALING_FACTOR;
// Converts frequency to time in ms
extern "C" {
    pub fn div64_u64(_arg: (u64)SSP_FACTOR_WITH_MS, _arg: value) -> return;
}
