//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/consumer.h
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
// Consumer interface the pin control subsystem
//
// Copyright (C) 2012 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

// This struct is private to the core and should be regarded as a cookie

// External interface to pin control
extern "C" {
    pub fn pinctrl_gpio_can_use_line(gc: *mut gpio_chip, offset: c_uint) -> bool;
}
extern "C" {
    pub fn pinctrl_gpio_request(gc: *mut gpio_chip, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn pinctrl_gpio_free(gc: *mut gpio_chip, offset: c_uint);
}
extern "C" {
    pub fn pinctrl_get(dev: *mut device) -> *mut pinctrl  __must_check;
}
extern "C" {
    pub fn pinctrl_put(p: *mut pinctrl);
}
extern "C" {
    pub fn pinctrl_select_state(p: *mut pinctrl, s: *mut pinctrl_state) -> c_int;
}
extern "C" {
    pub fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl  __must_check;
}
extern "C" {
    pub fn devm_pinctrl_put(p: *mut pinctrl);
}
extern "C" {
    pub fn pinctrl_select_default_state(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn pinctrl_pm_select_default_state(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pinctrl_pm_select_init_state(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pinctrl_pm_select_sleep_state(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pinctrl_pm_select_idle_state(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn ERR_CAST(_arg: s) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}
extern "C" {
    pub fn pinctrl_get_select(_arg: dev, _arg: PINCTRL_STATE_DEFAULT) -> return;
}
extern "C" {
    pub fn ERR_CAST(_arg: s) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}
extern "C" {
    pub fn devm_pinctrl_get_select(_arg: dev, _arg: PINCTRL_STATE_DEFAULT) -> return;
}
