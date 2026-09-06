//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/rockchip.h
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
// Header providing constants for Rockchip pinctrl bindings.
//
// Copyright (c) 2013 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
pub const RK_PA0: c_int = 0;
pub const RK_PA1: c_int = 1;
pub const RK_PA2: c_int = 2;
pub const RK_PA3: c_int = 3;
pub const RK_PA4: c_int = 4;
pub const RK_PA5: c_int = 5;
pub const RK_PA6: c_int = 6;
pub const RK_PA7: c_int = 7;
pub const RK_PB0: c_int = 8;
pub const RK_PB1: c_int = 9;
pub const RK_PB2: c_int = 10;
pub const RK_PB3: c_int = 11;
pub const RK_PB4: c_int = 12;
pub const RK_PB5: c_int = 13;
pub const RK_PB6: c_int = 14;
pub const RK_PB7: c_int = 15;
pub const RK_PC0: c_int = 16;
pub const RK_PC1: c_int = 17;
pub const RK_PC2: c_int = 18;
pub const RK_PC3: c_int = 19;
pub const RK_PC4: c_int = 20;
pub const RK_PC5: c_int = 21;
pub const RK_PC6: c_int = 22;
pub const RK_PC7: c_int = 23;
pub const RK_PD0: c_int = 24;
pub const RK_PD1: c_int = 25;
pub const RK_PD2: c_int = 26;
pub const RK_PD3: c_int = 27;
pub const RK_PD4: c_int = 28;
pub const RK_PD5: c_int = 29;
pub const RK_PD6: c_int = 30;
pub const RK_PD7: c_int = 31;
pub const RK_FUNC_GPIO: c_int = 0;
