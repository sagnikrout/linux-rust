//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/qcom/pinctrl-lpass-lpi.h
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
// Copyright (c) 2016-2019, The Linux Foundation. All rights reserved.
// Copyright (c) 2020 Linaro Ltd.
//

pub const LPI_SLEW_RATE_CTL_REG: c_uint = 0xa000;
pub const LPI_SPARE_1_REG: c_uint = 0xc000;
pub const LPI_TLMM_REG_OFFSET: c_uint = 0x1000;
pub const LPI_SLEW_RATE_MAX: c_uint = 0x03;
pub const LPI_SLEW_BITS_SIZE: c_uint = 0x02;

pub const LPI_GPIO_CFG_REG: c_uint = 0x00;

pub const LPI_GPIO_VALUE_REG: c_uint = 0x04;

pub const LPI_GPIO_BIAS_DISABLE: c_uint = 0x0;
pub const LPI_GPIO_PULL_DOWN: c_uint = 0x1;
pub const LPI_GPIO_KEEPER: c_uint = 0x2;
pub const LPI_GPIO_PULL_UP: c_uint = 0x3;

//
// Slew rate control is done in the same register as rest of the
// pin configuration.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_pingroup {
    pub pin: c_uint,
// Bit offset in slew register for SoundWire pins only
    pub slew_offset: c_int,
    pub funcs: *mut c_uint,
    pub nfuncs: c_uint,
    pub pin_offset: c_uint,
    pub slew_base_spare_1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpi_pinctrl_variant_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_int,
    pub groups: *const lpi_pingroup,
    pub ngroups: c_int,
    pub functions: *const lpi_function,
    pub nfunctions: c_int,
    pub flags: c_uint,
}

extern "C" {
    pub fn lpi_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn lpi_pinctrl_remove(pdev: *mut platform_device);
}
