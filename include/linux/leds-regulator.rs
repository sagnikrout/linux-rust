//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-regulator.h
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
// leds-regulator.h - platform data structure for regulator driven LEDs.
//
// Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
//
// Use "vled" as supply id when declaring the regulator consumer:
//
// static struct regulator_consumer_supply pcap_regulator_VVIB_consumers [] = {
// { .dev_name = "leds-regulator.0", .supply = "vled" },
// };
//
// If you have several regulator driven LEDs, you can append a numerical id to
// .dev_name as done above, and use the same id when declaring the platform
// device:
//
// static struct led_regulator_platform_data a780_vibrator_data = {
// .name   = "a780::vibrator",
// };
//
// static struct platform_device a780_vibrator = {
// .name = "leds-regulator",
// .id   = 0,
// .dev  = {
// .platform_data = &a780_vibrator_data,
// },
// };
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_regulator_platform_data {
    pub /: *mut *mut *mut char name; / LED name as expected by LED class,
    pub /: *mut *mut led_brightness brightness; / initial brightness value,
}
