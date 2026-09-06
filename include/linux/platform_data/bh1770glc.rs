//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/bh1770glc.h
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
// This file is part of the ROHM BH1770GLC / OSRAM SFH7770 sensor driver.
// Chip is combined proximity and ambient light sensor.
//
// Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
//
// Contact: Samu Onkalo <samu.p.onkalo@nokia.com>
//
// struct bh1770_platform_data - platform data for bh1770glc driver
// @led_def_curr: IR led driving current.
// @glass_attenuation: Attenuation factor for covering window.
// @setup_resources: Call back for interrupt line setup function
// @release_resources: Call back for interrupte line release function
//
// Example of glass attenuation: 16384 * 385 / 100 means attenuation factor
// of 3.85. i.e. light_above_sensor = light_above_cover_window / 3.85
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bh1770_platform_data {
pub const BH1770_LED_5mA: c_int = 0;
pub const BH1770_LED_10mA: c_int = 1;
pub const BH1770_LED_20mA: c_int = 2;
pub const BH1770_LED_50mA: c_int = 3;
pub const BH1770_LED_100mA: c_int = 4;
pub const BH1770_LED_150mA: c_int = 5;
pub const BH1770_LED_200mA: c_int = 6;
    pub led_def_curr: __u8,

    pub glass_attenuation: __u32,
    pub (*setup_resources)(void): *mut c_int,
    pub (*release_resources)(void): *mut c_int,
}
