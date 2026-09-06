//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8180/rtl8225se.h
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
// Definitions for RTL8187SE hardware
//
// Copyright 2009 Larry Finger <Larry.Finger@lwfinger.net>
// Copyright 2014 Andrea Merello <andrea.merello@gmail.com>
//
// Based on the r8180 and Realtek r8187se drivers, which are:
// Copyright 2004-2005 Andrea Merello <andrea.merello@gmail.com>, et al.
//
// Also based on the rtl8187 driver, which is:
// Copyright 2007 Michael Wu <flamingice@sourmilk.net>
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
pub const RTL8225SE_ANAPARAM_ON: c_uint = 0xb0054d00;
pub const RTL8225SE_ANAPARAM2_ON: c_uint = 0x000004c6;
// all off except PLL
pub const RTL8225SE_ANAPARAM_OFF: c_uint = 0xb0054dec;
// all on including PLL
pub const RTL8225SE_ANAPARAM_OFF2: c_uint = 0xb0054dfc;
pub const RTL8225SE_ANAPARAM2_OFF: c_uint = 0x00ff04c6;
pub const RTL8225SE_ANAPARAM3: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8187se_power_state {
    RTL8187SE_POWER_ON,
    RTL8187SE_POWER_OFF,
    RTL8187SE_POWER_SLEEP
}

extern "C" {
    pub fn rtl8225se_rf_stop(dev: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8225se_rf_init(dev: *mut ieee80211_hw);
}
