//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/dra.h
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
// This header provides constants for DRA pinctrl bindings.
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
// Author: Rajendra Nayak <rnayak@ti.com>
//
// DRA7 mux mode options for each pin. See TRM for options
pub const MUX_MODE0: c_uint = 0x0;
pub const MUX_MODE1: c_uint = 0x1;
pub const MUX_MODE2: c_uint = 0x2;
pub const MUX_MODE3: c_uint = 0x3;
pub const MUX_MODE4: c_uint = 0x4;
pub const MUX_MODE5: c_uint = 0x5;
pub const MUX_MODE6: c_uint = 0x6;
pub const MUX_MODE7: c_uint = 0x7;
pub const MUX_MODE8: c_uint = 0x8;
pub const MUX_MODE9: c_uint = 0x9;
pub const MUX_MODE10: c_uint = 0xa;
pub const MUX_MODE11: c_uint = 0xb;
pub const MUX_MODE12: c_uint = 0xc;
pub const MUX_MODE13: c_uint = 0xd;
pub const MUX_MODE14: c_uint = 0xe;
pub const MUX_MODE15: c_uint = 0xf;
// Certain pins need virtual mode, but note: they may glitch

// Active pin states

//
// Macro to allow using the absolute physical address instead of the
// padconf registers instead of the offset from padconf base.
//

// DRA7 IODELAY configuration parameters

