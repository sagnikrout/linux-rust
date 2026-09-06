//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/pic32.h
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
// Joshua Henderson <joshua.henderson@microchip.com>
// Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
//

//
// PIC32 register offsets for SET/CLR/INV where supported.
//

//
// PIC32 Base Register Offsets
//
pub const PIC32_BASE_CONFIG: c_uint = 0x1f800000;
pub const PIC32_BASE_OSC: c_uint = 0x1f801200;
pub const PIC32_BASE_RESET: c_uint = 0x1f801240;
pub const PIC32_BASE_PPS: c_uint = 0x1f801400;
pub const PIC32_BASE_UART: c_uint = 0x1f822000;
pub const PIC32_BASE_PORT: c_uint = 0x1f860000;
pub const PIC32_BASE_DEVCFG2: c_uint = 0x1fc4ff44;

// Register unlock sequence required for some register access.
extern "C" {
    pub fn pic32_syskey_unlock_debug(fn: *const c_char, ln: c_ulong);
}

// COMPILE_TEST on all other architectures
// Macro flag: #define pic32_syskey_unlock()

