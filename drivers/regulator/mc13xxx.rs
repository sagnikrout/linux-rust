//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/mc13xxx.h
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
// mc13xxx.h - regulators for the Freescale mc13xxx PMIC
//
// Copyright (C) 2010 Yong Shen <yong.shen@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_regulator {
    pub desc: regulator_desc,
    pub reg: c_int,
    pub enable_bit: c_int,
    pub vsel_reg: c_int,
    pub vsel_shift: c_int,
    pub vsel_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_regulator_priv {
    pub mc13xxx: *mut mc13xxx,
    pub powermisc_pwgt_state: u32,
    pub mc13xxx_regulators: *mut mc13xxx_regulator,
    pub num_regulators: c_int,
    pub __counted_by(num_regulators): *mut *mut regulator_dev regulators[],
}

extern "C" {
    pub fn mc13xxx_get_num_regulators_dt(pdev: *mut platform_device) -> c_int;
}

