//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/gpio/gpio-amd-fch.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// AMD FCH gpio driver platform-data
//
// Copyright (C) 2018 metux IT consult
// Author: Enrico Weigelt <info@metux.net>
//

//
// gpio register index definitions
//
pub const AMD_FCH_GPIO_REG_GPIO49: c_uint = 0x40;
pub const AMD_FCH_GPIO_REG_GPIO50: c_uint = 0x41;
pub const AMD_FCH_GPIO_REG_GPIO51: c_uint = 0x42;
pub const AMD_FCH_GPIO_REG_GPIO55_DEVSLP0: c_uint = 0x43;
pub const AMD_FCH_GPIO_REG_GPIO57: c_uint = 0x44;
pub const AMD_FCH_GPIO_REG_GPIO58: c_uint = 0x45;
pub const AMD_FCH_GPIO_REG_GPIO59_DEVSLP1: c_uint = 0x46;
pub const AMD_FCH_GPIO_REG_GPIO64: c_uint = 0x47;
pub const AMD_FCH_GPIO_REG_GPIO68: c_uint = 0x48;
pub const AMD_FCH_GPIO_REG_GPIO66_SPKR: c_uint = 0x5B;
pub const AMD_FCH_GPIO_REG_GPIO71: c_uint = 0x4D;
pub const AMD_FCH_GPIO_REG_GPIO32_GE1: c_uint = 0x59;
pub const AMD_FCH_GPIO_REG_GPIO33_GE2: c_uint = 0x5A;
pub const AMT_FCH_GPIO_REG_GEVT22: c_uint = 0x09;
//
// struct amd_fch_gpio_pdata - GPIO chip platform data
// @gpio_num: number of entries
// @gpio_reg: array of gpio registers
// @gpio_names: array of gpio names
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_fch_gpio_pdata {
    pub gpio_num: c_int,
    pub gpio_reg: *mut c_int,
    pub gpio_names: *const *const c_char,
}
