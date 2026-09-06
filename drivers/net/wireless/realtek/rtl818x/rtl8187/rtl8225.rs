//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8187/rtl8225.h
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
// Radio tuning definitions for RTL8225 on RTL8187
//
// Copyright 2007 Michael Wu <flamingice@sourmilk.net>
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
// Based on the r8187 driver, which is:
// Copyright 2005 Andrea Merello <andrea.merello@gmail.com>, et al.
//
pub const RTL8187_RTL8225_ANAPARAM_ON: c_uint = 0xa0000a59;
pub const RTL8187_RTL8225_ANAPARAM2_ON: c_uint = 0x860c7312;
pub const RTL8187_RTL8225_ANAPARAM_OFF: c_uint = 0xa00beb59;
pub const RTL8187_RTL8225_ANAPARAM2_OFF: c_uint = 0x840dec11;
pub const RTL8187B_RTL8225_ANAPARAM_ON: c_uint = 0x45090658;
pub const RTL8187B_RTL8225_ANAPARAM2_ON: c_uint = 0x727f3f52;
pub const RTL8187B_RTL8225_ANAPARAM3_ON: c_uint = 0x00;
pub const RTL8187B_RTL8225_ANAPARAM_OFF: c_uint = 0x55480658;
pub const RTL8187B_RTL8225_ANAPARAM2_OFF: c_uint = 0x72003f50;
pub const RTL8187B_RTL8225_ANAPARAM3_OFF: c_uint = 0x00;
extern "C" {
    pub fn rtl8187_detect_rf(: *mut ieee80211_hw) -> *const rtl818x_rf_ops;
}
