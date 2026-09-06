//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/twl4030_madc_battery.h
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
// Dumb driver for LiIon batteries using TWL4030 madc.
//
// Copyright 2013 Golden Delicious Computers
// Nikolaus Schaller <hns@goldelico.com>
//
// Usually we can assume 100% @ 4.15V and 0% @ 3.3V but curves differ for
// charging and discharging!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_madc_bat_calibration {
    pub /: *mut *mut short voltage; / in mV - specify -1 for end of list,
    pub /: *mut *mut short level; / in percent (0 .. 100%),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_madc_bat_platform_data {
    pub /: *mut *mut unsigned int capacity; / total capacity in uAh,
    pub charging: *mut twl4030_madc_bat_calibration,
    pub charging_size: c_int,
    pub discharging: *mut twl4030_madc_bat_calibration,
    pub discharging_size: c_int,
}
