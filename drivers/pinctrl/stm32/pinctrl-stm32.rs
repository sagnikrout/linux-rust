//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/stm32/pinctrl-stm32.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) Maxime Coquelin 2015
// Copyright (C) STMicroelectronics 2017
// Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
//

pub const STM32_PIN_GPIO: c_int = 0;

// package information

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_desc_function {
    pub name: *const c_char,
    pub num: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_desc_pin {
    pub pin: pinctrl_pin_desc,
    pub functions: [stm32_desc_function; STM32_CONFIG_NUM],
    pub pkg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_pinctrl_match_data {
    pub pins: *const stm32_desc_pin,
    pub npins: c_uint,
    pub secure_control: bool,
    pub io_sync_control: bool,
    pub rif_control: bool,
}

//
// stm32_pctl_probe() - Common probe for stm32 pinctrl drivers.
// @pdev: Pinctrl platform device.
//
extern "C" {
    pub fn stm32_pctl_probe(pdev: *mut platform_device) -> c_int;
}
//
// stm32_pinctrl_suspend() - Common suspend for stm32 pinctrl drivers.
// @dev: Pinctrl device.
//
extern "C" {
    pub fn stm32_pinctrl_suspend(dev: *mut device) -> c_int;
}
//
// stm32_pinctrl_resume() - Common resume for stm32 pinctrl drivers.
// @dev: Pinctrl device.
//
extern "C" {
    pub fn stm32_pinctrl_resume(dev: *mut device) -> c_int;
}
