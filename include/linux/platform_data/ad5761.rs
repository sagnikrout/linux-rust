//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ad5761.h
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
// AD5721, AD5721R, AD5761, AD5761R, Voltage Output Digital to Analog Converter
//
// Copyright 2016 Qtechnology A/S
// 2016 Ricardo Ribalda <ribalda@kernel.org>
//
// enum ad5761_voltage_range - Voltage range the AD5761 is configured for.
// @AD5761_VOLTAGE_RANGE_M10V_10V:  -10V to  10V
// @AD5761_VOLTAGE_RANGE_0V_10V:      0V to  10V
// @AD5761_VOLTAGE_RANGE_M5V_5V:     -5V to   5V
// @AD5761_VOLTAGE_RANGE_0V_5V:       0V to   5V
// @AD5761_VOLTAGE_RANGE_M2V5_7V5: -2.5V to 7.5V
// @AD5761_VOLTAGE_RANGE_M3V_3V:     -3V to   3V
// @AD5761_VOLTAGE_RANGE_0V_16V:      0V to  16V
// @AD5761_VOLTAGE_RANGE_0V_20V:      0V to  20V
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad5761_voltage_range {
    AD5761_VOLTAGE_RANGE_M10V_10V,
    AD5761_VOLTAGE_RANGE_0V_10V,
    AD5761_VOLTAGE_RANGE_M5V_5V,
    AD5761_VOLTAGE_RANGE_0V_5V,
    AD5761_VOLTAGE_RANGE_M2V5_7V5,
    AD5761_VOLTAGE_RANGE_M3V_3V,
    AD5761_VOLTAGE_RANGE_0V_16V,
    AD5761_VOLTAGE_RANGE_0V_20V,
}

//
// struct ad5761_platform_data - AD5761 DAC driver platform data
// @voltage_range: Voltage range the AD5761 is configured for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5761_platform_data {
    pub voltage_range: ad5761_voltage_range,
}
