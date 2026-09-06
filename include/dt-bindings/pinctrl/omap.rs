//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/omap.h
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
//
// This header provides constants for OMAP pinctrl bindings.
//
// Copyright (C) 2009 Nokia
// Copyright (C) 2009-2010 Texas Instruments
//
// 34xx mux mode options for each pin. See TRM for options
pub const MUX_MODE0: c_int = 0;
pub const MUX_MODE1: c_int = 1;
pub const MUX_MODE2: c_int = 2;
pub const MUX_MODE3: c_int = 3;
pub const MUX_MODE4: c_int = 4;
pub const MUX_MODE5: c_int = 5;
pub const MUX_MODE6: c_int = 6;
pub const MUX_MODE7: c_int = 7;
// 24xx/34xx mux bit defines

// omap3/4/5 specific mux bit defines

// Active pin states
pub const PIN_OUTPUT: c_int = 0;

// Off mode states
pub const PIN_OFF_NONE: c_int = 0;

//
// Macros to allow using the absolute physical address instead of the
// padconf registers instead of the offset from padconf base.
//

//
// Macros to allow using the offset from the padconf physical address
// instead  of the offset from padconf base.
//

//
// Define some commonly used pins configured by the boards.
// Note that some boards use alternative pins, so check
// the schematics before using these.
//
pub const OMAP3_UART1_RX: c_uint = 0x152;
pub const OMAP3_UART2_RX: c_uint = 0x14a;
pub const OMAP3_UART3_RX: c_uint = 0x16e;
pub const OMAP4_UART2_RX: c_uint = 0xdc;
pub const OMAP4_UART3_RX: c_uint = 0x104;
pub const OMAP4_UART4_RX: c_uint = 0x11c;
