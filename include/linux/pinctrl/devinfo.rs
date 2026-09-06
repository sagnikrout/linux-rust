//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/devinfo.h
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
// Per-device information from the pin control system.
// This is the stuff that get included into the device
// core.
//
// Copyright (C) 2012 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// This interface is used in the core to keep track of pins.
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

// The device core acts as a consumer toward pinctrl

//
// struct dev_pin_info - pin state container for devices
// @p: pinctrl handle for the containing device
// @default_state: the default state for the handle, if found
// @init_state: the state at probe time, if found
// @sleep_state: the state at suspend time, if found
// @idle_state: the state at idle (runtime suspend) time, if found
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pin_info {
    pub p: *mut pinctrl,
    pub default_state: *mut pinctrl_state,
    pub init_state: *mut pinctrl_state,

    pub sleep_state: *mut pinctrl_state,
    pub idle_state: *mut pinctrl_state,

}

extern "C" {
    pub fn pinctrl_init_done(dev: *mut device) -> c_int;
}

// Stubs if we're not using pinctrl

