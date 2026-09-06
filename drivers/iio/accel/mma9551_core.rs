//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/mma9551_core.h
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
// Common code for Freescale MMA955x Intelligent Sensor Platform drivers
// Copyright (c) 2014, Intel Corporation.
//
// Applications IDs
pub const MMA9551_APPID_VERSION: c_uint = 0x00;
pub const MMA9551_APPID_GPIO: c_uint = 0x03;
pub const MMA9551_APPID_AFE: c_uint = 0x06;
pub const MMA9551_APPID_TILT: c_uint = 0x0B;
pub const MMA9551_APPID_SLEEP_WAKE: c_uint = 0x12;
pub const MMA9551_APPID_PEDOMETER: c_uint = 0x15;
pub const MMA9551_APPID_RSC: c_uint = 0x17;
pub const MMA9551_APPID_NONE: c_uint = 0xff;
// Reset/Suspend/Clear application app masks

pub const MMA9551_AUTO_SUSPEND_DELAY_MS: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mma9551_gpio_pin {
    mma9551_gpio6 = 0,
    mma9551_gpio7,
    mma9551_gpio8,
    mma9551_gpio9,
    mma9551_gpio_max = mma9551_gpio9,
}

extern "C" {
    pub fn mma9551_read_version(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn mma9551_set_device_state(client: *mut i2c_client, enable: bool) -> c_int;
}
extern "C" {
    pub fn mma9551_set_power_state(client: *mut i2c_client, on: bool) -> c_int;
}
extern "C" {
    pub fn mma9551_sleep(freq: c_int);
}
extern "C" {
    pub fn mma9551_read_accel_scale(val: *mut c_int, val2: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mma9551_app_reset(client: *mut i2c_client, app_mask: u32) -> c_int;
}
