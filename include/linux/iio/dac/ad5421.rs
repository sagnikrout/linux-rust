//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/dac/ad5421.h
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
// enum ad5421_current_range - Current range the AD5421 is configured for.
// @AD5421_CURRENT_RANGE_4mA_20mA: 4 mA to 20 mA (RANGE1,0 pins = 00)
// @AD5421_CURRENT_RANGE_3mA8_21mA: 3.8 mA to 21 mA (RANGE1,0 pins = x1)
// @AD5421_CURRENT_RANGE_3mA2_24mA: 3.2 mA to 24 mA (RANGE1,0 pins = 10)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad5421_current_range {
    AD5421_CURRENT_RANGE_4mA_20mA,
    AD5421_CURRENT_RANGE_3mA8_21mA,
    AD5421_CURRENT_RANGE_3mA2_24mA,
}

//
// struct ad5421_platform_data - AD5421 DAC driver platform data
// @external_vref: whether an external reference voltage is used or not
// @current_range: Current range the AD5421 is configured for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5421_platform_data {
    pub external_vref: bool,
    pub current_range: ad5421_current_range,
}
