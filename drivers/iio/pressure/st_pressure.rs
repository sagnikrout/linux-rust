//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/st_pressure.h
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
// STMicroelectronics pressures driver
//
// Copyright 2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
// v. 1.0.0
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_press_type {
    LPS001WP,
    LPS25H,
    LPS331AP,
    LPS22HB,
    LPS33HW,
    LPS35HW,
    LPS22HH,
    LPS22DF,
    ST_PRESS_MAX,
}

//
// struct st_sensors_platform_data - default press platform data
// @drdy_int_pin: default press DRDY is available on INT1 pin.
//

extern "C" {
    pub fn st_press_allocate_ring(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn st_press_trig_set_state(trig: *mut iio_trigger, state: bool) -> c_int;
}

