//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinmux.h
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
// pinmux portions
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

extern "C" {
    pub fn pinmux_check_ops(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn pinmux_validate_map(map: *const pinctrl_map, i: c_int) -> c_int;
}
extern "C" {
    pub fn pinmux_can_be_used_for_gpio(pctldev: *mut pinctrl_dev, pin: c_uint) -> bool;
}
extern "C" {
    pub fn pinmux_free_setting(setting: *const pinctrl_setting);
}
extern "C" {
    pub fn pinmux_enable_setting(setting: *const pinctrl_setting) -> c_int;
}
extern "C" {
    pub fn pinmux_disable_setting(setting: *const pinctrl_setting);
}

extern "C" {
    pub fn pinmux_show_map(s: *mut seq_file, map: *const pinctrl_map);
}

//
// struct function_desc - generic function descriptor
// @func: generic data of the pin function (name and groups of pins)
// @data: pin controller driver specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_desc {
    pub func: *const pinfunction,
    pub data: *mut c_void,
}

extern "C" {
    pub fn pinmux_generic_get_function_count(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn pinmux_generic_free_functions(pctldev: *mut pinctrl_dev);
}

