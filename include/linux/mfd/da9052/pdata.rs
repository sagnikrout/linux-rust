//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9052/pdata.h
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
// Platform data declarations for DA9052 PMICs.
//
// Copyright(c) 2011 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//
pub const DA9052_MAX_REGULATORS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_pdata {
    pub pled: *mut led_platform_data,
    pub da9052): *mut *mut int (init) (struct da9052,
    pub irq_base: c_int,
    pub gpio_base: c_int,
    pub use_for_apm: c_int,
    pub regulators: [*mut regulator_init_data; DA9052_MAX_REGULATORS],
}
