//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rt5033.h
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
// MFD core driver for the RT5033
//
// Copyright (C) 2014 Samsung Electronics
// Author: Beomho Seo <beomho.seo@samsung.com>
//

// RT5033 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5033_regulators {
    RT5033_BUCK = 0,
    RT5033_LDO,
    RT5033_SAFE_LDO,

    RT5033_REGULATOR_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5033_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irq: c_int,
    pub wakeup: bool,
}
