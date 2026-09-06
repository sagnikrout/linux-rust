//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/adxl355.h
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
// ADXL355 3-Axis Digital Accelerometer
//
// Copyright (c) 2021 Puranjay Mohan <puranjay12@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adxl355_device_type {
    ADXL355,
    ADXL359,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl355_fractional_type {
    pub integer: c_int,
    pub decimal: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl355_chip_info {
    pub name: *const c_char,
    pub part_id: u8,
    pub accel_scale: adxl355_fractional_type,
    pub temp_offset: adxl355_fractional_type,
}
