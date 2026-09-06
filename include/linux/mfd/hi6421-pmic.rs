//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/hi6421-pmic.h
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
// Header file for device driver Hi6421 PMIC
//
// Copyright (c) <2011-2014> HiSilicon Technologies Co., Ltd.
// http://www.hisilicon.com
// Copyright (c) <2013-2014> Linaro Ltd.
// https://www.linaro.org
//
// Author: Guodong Xu <guodong.xu@linaro.org>
//
// Hi6421 registers are mapped to memory bus in 4 bytes stride

// Hi6421 maximum register number
pub const HI6421_REG_MAX: c_uint = 0xFF;
// Hi6421 OCP (over current protection) and DEB (debounce) control register

pub const HI6421_OCP_DEB_SEL_MASK: c_uint = 0x0C;
pub const HI6421_OCP_DEB_SEL_8MS: c_uint = 0x00;
pub const HI6421_OCP_DEB_SEL_16MS: c_uint = 0x04;
pub const HI6421_OCP_DEB_SEL_32MS: c_uint = 0x08;
pub const HI6421_OCP_DEB_SEL_64MS: c_uint = 0x0C;
pub const HI6421_OCP_EN_DEBOUNCE_MASK: c_uint = 0x02;
pub const HI6421_OCP_EN_DEBOUNCE_ENABLE: c_uint = 0x02;
pub const HI6421_OCP_AUTO_STOP_MASK: c_uint = 0x01;
pub const HI6421_OCP_AUTO_STOP_ENABLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6421_pmic {
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6421_type {
    HI6421 = 0,
    HI6421_V530,
}
