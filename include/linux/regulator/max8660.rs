//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/max8660.h
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
// max8660.h  --  Voltage regulation for the Maxim 8660/8661
//
// Copyright (C) 2009 Wolfram Sang, Pengutronix e.K.
//

//
// max8660_subdev_data - regulator subdev data
// @id: regulator id
// @name: regulator name
// @platform_data: regulator init data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8660_subdev_data {
    pub id: c_int,
    pub name: *const c_char,
    pub platform_data: *mut regulator_init_data,
}

//
// max8660_platform_data - platform data for max8660
// @num_subdevs: number of regulators used
// @subdevs: pointer to regulators used
// @en34_is_high: if EN34 is driven high, regulators cannot be en-/disabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8660_platform_data {
    pub num_subdevs: c_int,
    pub subdevs: *mut max8660_subdev_data,
    pub en34_is_high:1: unsigned,
}
