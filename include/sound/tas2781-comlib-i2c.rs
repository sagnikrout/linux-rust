//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/tas2781-comlib-i2c.h
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
// ALSA SoC Texas Instruments TAS2563/TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2563/TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2563/TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//
extern "C" {
    pub fn tasdevice_reset(tas_dev: *mut tasdevice_priv);
}
extern "C" {
    pub fn tasdevice_init(tas_priv: *mut tasdevice_priv) -> c_int;
}
