//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/max1586.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// max1586.h  --  Voltage regulation for the Maxim 1586
//
// Copyright (C) 2008 Robert Jarzmik
//

// Macro flag: #define REGULATOR_MAX1586

pub const MAX1586_V3: c_int = 0;
pub const MAX1586_V6: c_int = 1;
// precalculated values for v3_gain

//
// max1586_subdev_data - regulator data
// @id: regulator Id (either MAX1586_V3 or MAX1586_V6)
// @name: regulator cute name (example for V3: "vcc_core")
// @platform_data: regulator init data (constraints, supplies, ...)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max1586_subdev_data {
    pub id: c_int,
    pub name: *const c_char,
    pub platform_data: *mut regulator_init_data,
}

//
// max1586_platform_data - platform data for max1586
// @num_subdevs: number of regulators used (may be 1 or 2)
// @subdevs: regulator used
// At most, there will be a regulator for V3 and one for V6 voltages.
// @v3_gain: gain on the V3 voltage output multiplied by 1e6.
// This can be calculated as ((1 + R24/R25 + R24/185.5kOhm) * 1e6)
// for an external resistor configuration as described in the
// data sheet (R25=100kOhm).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max1586_platform_data {
    pub num_subdevs: c_int,
    pub subdevs: *mut max1586_subdev_data,
    pub v3_gain: c_int,
}
