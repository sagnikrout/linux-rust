//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9055/pdata.h
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
// Copyright (C) 2012 Dialog Semiconductor Ltd.
//
pub const DA9055_MAX_REGULATORS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_select {
    NO_GPIO = 0,
    GPIO_1,
    GPIO_2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9055_pdata {
    pub da9055): *mut *mut int (init) (struct da9055,
    pub irq_base: c_int,
    pub gpio_base: c_int,
    pub regulators: [*mut regulator_init_data; DA9055_MAX_REGULATORS],
// Enable RTC in RESET Mode
    pub reset_enable: bool,
//
// Regulator mode control bits value (GPI offset) that
// controls the regulator state, 0 if not available.
//
    pub reg_ren: *mut gpio_select,
//
// Regulator mode control bits value (GPI offset) that
// controls the regulator set A/B, 0 if  not available.
//
    pub reg_rsel: *mut gpio_select,
}
