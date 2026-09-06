//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8187/leds.h
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
// Definitions for RTL8187 leds
//
// Copyright 2009 Larry Finger <Larry.Finger@lwfinger.net>
//
// Based on the LED handling in the r8187 driver, which is:
// Copyright (c) Realtek Semiconductor Corp. All rights reserved.
//

pub const RTL8187_LED_MAX_NAME_LEN: c_int = 21;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_led {
    pub dev: *mut ieee80211_hw,
// The LED class device
    pub led_dev: led_classdev,
// The pin/method used to control the led
    pub ledpin: u8,
// The unique name string for this LED device.
    pub 1]: char name[RTL8187_LED_MAX_NAME_LEN +,
// If the LED is radio or tx/rx
    pub is_radio: bool,
}

extern "C" {
    pub fn rtl8187_leds_init(dev: *mut ieee80211_hw, code: u16);
}
extern "C" {
    pub fn rtl8187_leds_exit(dev: *mut ieee80211_hw);
}

