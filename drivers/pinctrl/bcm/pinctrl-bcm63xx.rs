//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/bcm/pinctrl-bcm63xx.h
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
// Copyright (C) 2021 Álvaro Fernández Rojas <noltari@gmail.com>
// Copyright (C) 2016 Jonas Gorski <jonas.gorski@gmail.com>
//

pub const BCM63XX_BANK_GPIOS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_pinctrl_soc {
    pub pctl_ops: *const pinctrl_ops,
    pub pmx_ops: *const pinmux_ops,
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub ngpios: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63xx_pinctrl {
    pub dev: *mut device,
    pub regs: *mut regmap,
    pub pctl_desc: pinctrl_desc,
    pub pctl_dev: *mut pinctrl_dev,
    pub driver_data: *mut c_void,
}
