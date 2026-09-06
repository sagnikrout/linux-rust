//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lp8788-isink.h
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
// TI LP8788 MFD - common definitions for current sinks
//
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//
// register address
pub const LP8788_ISINK_CTRL: c_uint = 0x99;
pub const LP8788_ISINK12_IOUT: c_uint = 0x9A;
pub const LP8788_ISINK3_IOUT: c_uint = 0x9B;
pub const LP8788_ISINK1_PWM: c_uint = 0x9C;
pub const LP8788_ISINK2_PWM: c_uint = 0x9D;
pub const LP8788_ISINK3_PWM: c_uint = 0x9E;
// mask bits
pub const LP8788_ISINK1_IOUT_M: c_uint = 0x0F	/* Addr 9Ah */;
pub const LP8788_ISINK2_IOUT_M: c_uint = 0xF0;
pub const LP8788_ISINK3_IOUT_M: c_uint = 0x0F	/* Addr 9Bh */;
// 6 bits used for PWM code : Addr 9C ~ 9Eh
pub const LP8788_ISINK_MAX_PWM: c_int = 63;
pub const LP8788_ISINK_SCALE_OFFSET: c_int = 3;
