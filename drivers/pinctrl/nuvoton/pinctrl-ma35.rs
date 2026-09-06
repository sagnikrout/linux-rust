//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/nuvoton/pinctrl-ma35.h
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
// Copyright (C) 2024 Nuvoton Technology Corp.
//
// Author: Shan-Chun Hung <schung@nuvoton.com>
// *       Jacky Huang <ychuang3@nuvoton.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_mux_desc {
    pub name: *const c_char,
    pub muxval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_pin_data {
    pub offset: u32,
    pub shift: u32,
    pub muxes: *mut ma35_mux_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_pinctrl_soc_info {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub shift): *mut *mut int (get_pin_num)(int offset, int,
}

extern "C" {
    pub fn ma35_pinctrl_probe(pdev: *mut platform_device, info: *const ma35_pinctrl_soc_info) -> c_int;
}
extern "C" {
    pub fn ma35_pinctrl_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ma35_pinctrl_resume(dev: *mut device) -> c_int;
}
