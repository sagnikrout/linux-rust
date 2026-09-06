//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/max98095.h
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
// Platform data for MAX98095
//
// Copyright 2011 Maxim Integrated Products
//
// Equalizer filter response configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98095_eq_cfg {
    pub name: *const c_char,
    pub rate: c_uint,
    pub band1: [u16; 5],
    pub band2: [u16; 5],
    pub band3: [u16; 5],
    pub band4: [u16; 5],
    pub band5: [u16; 5],
}

// Biquad filter response configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98095_biquad_cfg {
    pub name: *const c_char,
    pub rate: c_uint,
    pub band1: [u16; 5],
    pub band2: [u16; 5],
}

// codec platform data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98095_pdata {
// Equalizers for DAI1 and DAI2
    pub eq_cfg: *mut max98095_eq_cfg,
    pub eq_cfgcnt: c_uint,
// Biquad filter for DAI1 and DAI2
    pub bq_cfg: *mut max98095_biquad_cfg,
    pub bq_cfgcnt: c_uint,
// Analog/digital microphone configuration:
// 0 = analog microphone input (normal setting)
// 1 = digital microphone input
//
    pub digmic_left_mode:1: c_uint,
    pub digmic_right_mode:1: c_uint,
// Pin5 is the mechanical method of sensing jack insertion
// but it is something that might not be supported.
// 0 = PIN5 not supported
// 1 = PIN5 supported
//
    pub jack_detect_pin5en:1: c_uint,
// Slew amount for jack detection. Calculated as 4 * (delay + 1).
// Default delay is 24 to get a time of 100ms.
//
    pub jack_detect_delay: c_uint,
}
