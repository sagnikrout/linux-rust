//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/da9055.h
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
// DA9055 ALSA Soc codec driver
//
// Copyright (c) 2012 Dialog Semiconductor
//
// Tested on (Samsung SMDK6410 board + DA9055 EVB) using I2S and I2C
// Written by David Chen <david.chen@diasemi.com> and
// Ashish Chavan <ashish.chavan@kpitcummins.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9055_micbias_voltage {
    DA9055_MICBIAS_1_6V = 0,
    DA9055_MICBIAS_1_8V = 1,
    DA9055_MICBIAS_2_1V = 2,
    DA9055_MICBIAS_2_2V = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9055_platform_data {
// Selects which of the two MicBias pins acts as the bias source
    pub micbias_source: bool,
// Selects the micbias voltage
    pub micbias: da9055_micbias_voltage,
}
