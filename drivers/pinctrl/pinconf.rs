//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinconf.h
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
// Internal interface between the core pin control system and the
// pin config portions
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

extern "C" {
    pub fn pinconf_check_ops(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn pinconf_validate_map(map: *const pinctrl_map, i: c_int) -> c_int;
}
extern "C" {
    pub fn pinconf_free_setting(setting: *const pinctrl_setting);
}
extern "C" {
    pub fn pinconf_apply_setting(setting: *const pinctrl_setting) -> c_int;
}
//
// You will only be interested in these if you're using PINCONF
// so don't supply any stubs for these.
//

extern "C" {
    pub fn pinconf_show_map(s: *mut seq_file, map: *const pinctrl_map);
}

//
// The following functions are available if the driver uses the generic
// pin config.
//

