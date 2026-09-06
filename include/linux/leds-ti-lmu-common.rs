//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-ti-lmu-common.h
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
// TI LMU Common Core
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com

pub const LMU_11BIT_MSB_SHIFT: c_int = 3;
pub const MAX_BRIGHTNESS_8BIT: c_int = 255;
pub const MAX_BRIGHTNESS_11BIT: c_int = 2047;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_lmu_bank {
    pub regmap: *mut regmap,
    pub max_brightness: c_int,
    pub lsb_brightness_reg: u8,
    pub msb_brightness_reg: u8,
    pub runtime_ramp_reg: u8,
    pub ramp_up_usec: u32,
    pub ramp_down_usec: u32,
}

extern "C" {
    pub fn ti_lmu_common_set_brightness(lmu_bank: *mut ti_lmu_bank, brightness: c_int) -> c_int;
}
extern "C" {
    pub fn ti_lmu_common_set_ramp(lmu_bank: *mut ti_lmu_bank) -> c_int;
}
