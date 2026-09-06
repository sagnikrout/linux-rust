//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/max8952.h
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
// max8952.h - Voltage regulation for the Maxim 8952
//
// Copyright (C) 2010 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

// Macro flag: #define REGULATOR_MAX8952

pub const MAX8952_NUM_DVS_MODE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8952_platform_data {
    pub default_mode: u32,
    pub /: *mut *mut u32 dvs_mode[MAX8952_NUM_DVS_MODE]; / MAX8952_DVS_MODEx_XXXXmV,
    pub sync_freq: u32,
    pub ramp_speed: u32,
    pub reg_data: *mut regulator_init_data,
}
