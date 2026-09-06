//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lm3639_bl.h
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
// Simple driver for Texas Instruments LM3630 LED Flash driver chip
// Copyright (C) 2012 Texas Instruments
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_pwm {
    LM3639_PWM_DISABLE = 0x00,
    LM3639_PWM_EN_ACTLOW = 0x48,
    LM3639_PWM_EN_ACTHIGH = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_strobe {
    LM3639_STROBE_DISABLE = 0x00,
    LM3639_STROBE_EN_ACTLOW = 0x10,
    LM3639_STROBE_EN_ACTHIGH = 0x30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_txpin {
    LM3639_TXPIN_DISABLE = 0x00,
    LM3639_TXPIN_EN_ACTLOW = 0x04,
    LM3639_TXPIN_EN_ACTHIGH = 0x0C,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_fleds {
    LM3639_FLED_DIASBLE_ALL = 0x00,
    LM3639_FLED_EN_1 = 0x40,
    LM3639_FLED_EN_2 = 0x20,
    LM3639_FLED_EN_ALL = 0x60,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_bleds {
    LM3639_BLED_DIASBLE_ALL = 0x00,
    LM3639_BLED_EN_1 = 0x10,
    LM3639_BLED_EN_2 = 0x08,
    LM3639_BLED_EN_ALL = 0x18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3639_bled_mode {
    LM3639_BLED_MODE_EXPONETIAL = 0x00,
    LM3639_BLED_MODE_LINEAR = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3639_platform_data {
    pub max_brt_led: c_uint,
    pub init_brt_led: c_uint,
// input pins
    pub pin_pwm: lm3639_pwm,
    pub pin_strobe: lm3639_strobe,
    pub pin_tx: lm3639_txpin,
// output pins
    pub fled_pins: lm3639_fleds,
    pub bled_pins: lm3639_bleds,
    pub bled_mode: lm3639_bled_mode,
    pub max_brightness): *mut *mut void (pwm_set_intensity) (int brightness, int,
    pub (void): *mut *mut int (pwm_get_intensity),
}
