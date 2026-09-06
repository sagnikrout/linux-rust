//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_power.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//

// IPA device power management function block
//
// ipa_core_clock_rate() - Return the current IPA core clock rate
// @ipa:	IPA structure
//
// Return: The current clock rate (in Hz), or 0.
//
extern "C" {
    pub fn ipa_core_clock_rate(ipa: *mut ipa) -> u32;
}
//
// ipa_power_retention() - Control register retention on power collapse
// @ipa:	IPA pointer
// @enable:	Whether retention should be enabled or disabled
//
extern "C" {
    pub fn ipa_power_retention(ipa: *mut ipa, enable: bool);
}
//
// ipa_power_init() - Initialize IPA power management
// @dev:	IPA device
// @data:	Clock configuration data
//
// Return:	A pointer to an ipa_power structure, or a pointer-coded error
//
// ipa_power_exit() - Inverse of ipa_power_init()
// @power:	IPA power pointer
//
extern "C" {
    pub fn ipa_power_exit(power: *mut ipa_power);
}
