//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/fan53555.h
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
// fan53555.h - Fairchild Regulator FAN53555 Driver
//
// Copyright (C) 2012 Marvell Technology Ltd.
// Yunfan Zhang <yfzhang@marvell.com>
//
// VSEL ID
// Transition slew rate limiting from a low to high voltage.
// -----------------------
// Bin |Slew Rate(mV/uS)
// ------|----------------
// 000 |    64.00
// ------|----------------
// 001 |    32.00
// ------|----------------
// 010 |    16.00
// ------|----------------
// 011 |     8.00
// ------|----------------
// 100 |     4.00
// ------|----------------
// 101 |     2.00
// ------|----------------
// 110 |     1.00
// ------|----------------
// 111 |     0.50
// -----------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fan53555_platform_data {
    pub regulator: *mut regulator_init_data,
    pub slew_rate: c_uint,
// Sleep VSEL ID
    pub sleep_vsel_id: c_uint,
}
