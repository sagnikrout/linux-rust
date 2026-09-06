//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/ltc2497.h
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
pub const LTC2497_ENABLE: c_uint = 0xA0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2497_chip_info {
    pub resolution: u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2497core_driverdata {
    pub ref: *mut regulator,
    pub time_prev: ktime_t,
// lock to protect against multiple access to the device
    pub lock: mutex,
    pub chip_info: *const ltc2497_chip_info,
    pub addr_prev: u8,
    pub val): *mut u8 address, int,
}

extern "C" {
    pub fn ltc2497core_probe(dev: *mut device, indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn ltc2497core_remove(indio_dev: *mut iio_dev);
}
