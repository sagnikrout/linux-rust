//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/leds/common.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// This header provides macros for the common LEDs device tree bindings.
//
// Copyright (C) 2015, Samsung Electronics Co., Ltd.
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//
// Copyright (C) 2019 Jacek Anaszewski <jacek.anaszewski@gmail.com>
// Copyright (C) 2020 Pavel Machek <pavel@ucw.cz>
//
// External trigger type
pub const LEDS_TRIG_TYPE_EDGE: c_int = 0;
pub const LEDS_TRIG_TYPE_LEVEL: c_int = 1;
// Boost modes
pub const LEDS_BOOST_OFF: c_int = 0;
pub const LEDS_BOOST_ADAPTIVE: c_int = 1;
pub const LEDS_BOOST_FIXED: c_int = 2;
// Standard LED colors
pub const LED_COLOR_ID_WHITE: c_int = 0;
pub const LED_COLOR_ID_RED: c_int = 1;
pub const LED_COLOR_ID_GREEN: c_int = 2;
pub const LED_COLOR_ID_BLUE: c_int = 3;
pub const LED_COLOR_ID_AMBER: c_int = 4;
pub const LED_COLOR_ID_VIOLET: c_int = 5;
pub const LED_COLOR_ID_YELLOW: c_int = 6;
pub const LED_COLOR_ID_IR: c_int = 7;

pub const LED_COLOR_ID_PURPLE: c_int = 10;
pub const LED_COLOR_ID_ORANGE: c_int = 11;
pub const LED_COLOR_ID_PINK: c_int = 12;
pub const LED_COLOR_ID_CYAN: c_int = 13;
pub const LED_COLOR_ID_LIME: c_int = 14;
pub const LED_COLOR_ID_MAX: c_int = 15;
// Standard LED functions
// Keyboard LEDs, usually it would be input4::capslock etc.
// Obsolete equivalent: "shift-key-light"

// Obsolete equivalents: "tpacpi::thinklight" (IBM/Lenovo Thinkpads),

// System LEDs, usually found on system body.

// Obsolete: "platform:*:charging" (allwinner sun50i)

// Used RGB notification LEDs common on phones.

// Used for player LEDs as found on game controllers from e.g. Nintendo, Sony.

// Miscelleaus functions. Use functions above if you can.

