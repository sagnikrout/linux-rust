//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/tps6507x.h
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
// tps6507x.h  --  Voltage regulation for the Texas Instruments TPS6507X
//
// Copyright (C) 2010 Texas Instruments, Inc.
//

// Macro flag: #define REGULATOR_TPS6507X
//
// tps6507x_reg_platform_data - platform data for tps6507x
// @defdcdc_default: Defines whether DCDC high or the low register controls
// output voltage by default. Valid for DCDC2 and DCDC3 outputs only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6507x_reg_platform_data {
    pub defdcdc_default: bool,
}
