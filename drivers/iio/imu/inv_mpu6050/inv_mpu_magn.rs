//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_mpu6050/inv_mpu_magn.h
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
//
// Copyright (C) 2019 TDK-InvenSense, Inc.
//

// Magnetometer maximum frequency
pub const INV_MPU_MAGN_FREQ_HZ_MAX: c_int = 50;
extern "C" {
    pub fn inv_mpu_magn_probe(st: *mut inv_mpu6050_state) -> c_int;
}
//
// inv_mpu_magn_get_scale() - get magnetometer scale value
// @st: driver internal state
//
// Returns IIO data format.
//
// val = 0;
// val2 = st->magn_raw_to_gauss[chan->address];
extern "C" {
    pub fn inv_mpu_magn_set_rate(st: *const inv_mpu6050_state, fifo_rate: c_int) -> c_int;
}
extern "C" {
    pub fn inv_mpu_magn_set_orient(st: *mut inv_mpu6050_state) -> c_int;
}
extern "C" {
    pub fn inv_mpu_magn_read(st: *mut inv_mpu6050_state, axis: c_int, val: *mut c_int) -> c_int;
}
