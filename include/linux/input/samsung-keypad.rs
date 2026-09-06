//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/samsung-keypad.h
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
//
// Samsung Keypad platform data definitions
//
// Copyright (C) 2010 Samsung Electronics Co.Ltd
// Author: Joonyoung Shim <jy0922.shim@samsung.com>
//

pub const SAMSUNG_MAX_ROWS: c_int = 8;
pub const SAMSUNG_MAX_COLS: c_int = 8;
//
// struct samsung_keypad_platdata - Platform device data for Samsung Keypad.
// @keymap_data: pointer to &matrix_keymap_data.
// @rows: number of keypad row supported.
// @cols: number of keypad col supported.
// @no_autorepeat: disable key autorepeat.
// @wakeup: controls whether the device should be set up as wakeup source.
// @cfg_gpio: configure the GPIO.
//
// Initialisation data specific to either the machine or the platform
// for the device driver to use or call-back when configuring gpio.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_keypad_platdata {
    pub keymap_data: *const matrix_keymap_data,
    pub rows: c_uint,
    pub cols: c_uint,
    pub no_autorepeat: bool,
    pub wakeup: bool,
    pub cols): *mut *mut void (cfg_gpio)(unsigned int rows, unsigned int,
}
