//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/bmi160/bmi160.h
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
#[derive(Copy, Clone)]
pub struct bmi160_data {
    pub regmap: *mut regmap,
    pub trig: *mut iio_trigger,
    pub supplies: [regulator_bulk_data; 2],
    pub orientation: iio_mount_matrix,
//
// Ensure natural alignment for timestamp if present.
// Max length needed: 2 * 3 channels + 4 bytes padding + 8 byte ts.
// If fewer channels are enabled, less space may be needed, as
// long as the timestamp is still aligned to 8 bytes.
//
    pub __aligned(8): __le16 buf[12],
}

extern "C" {
    pub fn bmi160_enable_irq(regmap: *mut regmap, enable: bool) -> c_int;
}
extern "C" {
    pub fn bmi160_probe_trigger(indio_dev: *mut iio_dev, irq: c_int, irq_type: u32) -> c_int;
}
