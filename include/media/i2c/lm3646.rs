//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/lm3646.h
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
// include/media/i2c/lm3646.h
//
// Copyright (C) 2014 Texas Instruments
//
// Contact: Daniel Jeong <gshark.jeong@gmail.com>
// Ldd-Mlp <ldd-mlp@list.ti.com>
//

// TOTAL FLASH Brightness Max
// min 93350uA, step 93750uA, max 1499600uA
//
pub const LM3646_TOTAL_FLASH_BRT_MIN: c_int = 93350;
pub const LM3646_TOTAL_FLASH_BRT_STEP: c_int = 93750;
pub const LM3646_TOTAL_FLASH_BRT_MAX: c_int = 1499600;

// TOTAL TORCH Brightness Max
// min 23040uA, step 23430uA, max 187100uA
//
pub const LM3646_TOTAL_TORCH_BRT_MIN: c_int = 23040;
pub const LM3646_TOTAL_TORCH_BRT_STEP: c_int = 23430;
pub const LM3646_TOTAL_TORCH_BRT_MAX: c_int = 187100;

// LED1 FLASH Brightness
// min 23040uA, step 11718uA, max 1499600uA
//
pub const LM3646_LED1_FLASH_BRT_MIN: c_int = 23040;
pub const LM3646_LED1_FLASH_BRT_STEP: c_int = 11718;
pub const LM3646_LED1_FLASH_BRT_MAX: c_int = 1499600;

// LED1 TORCH Brightness
// min 2530uA, step 1460uA, max 187100uA
//
pub const LM3646_LED1_TORCH_BRT_MIN: c_int = 2530;
pub const LM3646_LED1_TORCH_BRT_STEP: c_int = 1460;
pub const LM3646_LED1_TORCH_BRT_MAX: c_int = 187100;

// FLASH TIMEOUT DURATION
// min 50ms, step 50ms, max 400ms
//
pub const LM3646_FLASH_TOUT_MIN: c_int = 50;
pub const LM3646_FLASH_TOUT_STEP: c_int = 50;
pub const LM3646_FLASH_TOUT_MAX: c_int = 400;

// struct lm3646_platform_data
//
// @flash_timeout: flash timeout
// @led1_flash_brt: led1 flash mode brightness, uA
// @led1_torch_brt: led1 torch mode brightness, uA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3646_platform_data {
    pub flash_timeout: u32,
    pub led1_flash_brt: u32,
    pub led1_torch_brt: u32,
}
