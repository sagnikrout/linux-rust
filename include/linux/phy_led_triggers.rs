//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy_led_triggers.h
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
// Copyright (C) 2016 National Instruments Corp.
//

pub const PHY_LED_TRIGGER_SPEED_SUFFIX_SIZE: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_led_trigger {
    pub trigger: led_trigger,
    pub name: [c_char; PHY_LINK_LED_TRIGGER_NAME_SIZE],
    pub speed: c_uint,
}

extern "C" {
    pub fn phy_led_triggers_register(phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn phy_led_triggers_unregister(phy: *mut phy_device);
}
extern "C" {
    pub fn phy_led_trigger_change_speed(phy: *mut phy_device);
}

