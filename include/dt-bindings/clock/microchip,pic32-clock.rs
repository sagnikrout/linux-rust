//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/microchip,pic32-clock.h
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
// Purna Chandra Mandal,<purna.mandal@microchip.com>
// Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
//
// clock output indices
pub const POSCCLK: c_int = 0;
pub const FRCCLK: c_int = 1;
pub const BFRCCLK: c_int = 2;
pub const LPRCCLK: c_int = 3;
pub const SOSCCLK: c_int = 4;
pub const FRCDIVCLK: c_int = 5;
pub const PLLCLK: c_int = 6;
pub const SCLK: c_int = 7;
pub const PB1CLK: c_int = 8;
pub const PB2CLK: c_int = 9;
pub const PB3CLK: c_int = 10;
pub const PB4CLK: c_int = 11;
pub const PB5CLK: c_int = 12;
pub const PB6CLK: c_int = 13;
pub const PB7CLK: c_int = 14;
pub const REF1CLK: c_int = 15;
pub const REF2CLK: c_int = 16;
pub const REF3CLK: c_int = 17;
pub const REF4CLK: c_int = 18;
pub const REF5CLK: c_int = 19;
pub const UPLLCLK: c_int = 20;
pub const MAXCLKS: c_int = 21;
