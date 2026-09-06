//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/ad7877.h
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
// linux/spi/ad7877.h
// Touchscreen characteristics vary between boards and models.  The
// platform_data for the device's "struct device" holds this information.
//
// It's OK if the min/max values are zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7877_platform_data {
    pub /: *mut *mut u16 model; / 7877,
    pub /: *mut *mut u16 vref_delay_usecs; / 0 for external vref; etc,
    pub x_plate_ohms: u16,
    pub y_plate_ohms: u16,
    pub x_max: u16 x_min,,
    pub y_max: u16 y_min,,
    pub pressure_max: u16 pressure_min,,
    pub /: *mut *mut u8 stopacq_polarity; / 1 = Active HIGH, 0 = Active LOW,
    pub /: *mut *mut u8 first_conversion_delay; / 0 = 0.5us, 1 = 128us, 2 = 1ms, 3 = 8ms,
    pub /: *mut *mut u8 acquisition_time; / 0 = 2us, 1 = 4us, 2 = 8us, 3 = 16us,
    pub /: *mut *mut u8 averaging; / 0 = 1, 1 = 4, 2 = 8, 3 = 16,
    pub ms,: *mut *mut u8 pen_down_acc_interval; / 0 = covert once, 1 = every 0.5,
}
