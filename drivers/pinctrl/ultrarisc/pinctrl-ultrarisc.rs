//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/ultrarisc/pinctrl-ultrarisc.h
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
// Copyright (C) 2026 UltraRISC Technology (Shanghai) Co., Ltd.
//
// Author: Jia Wang <wangjia@ultrarisc.com>
//

pub const UR_FUNC_1: c_uint = 0x10000U;
pub const UR_MAX_PINS_PER_PORT: c_int = 16;
pub const UR_BIAS_MASK: c_uint = 0x0000000F;
pub const UR_PULL_MASK: c_uint = 0x0C;
pub const UR_PULL_DIS: c_int = 0;
pub const UR_PULL_UP: c_int = 1;
pub const UR_PULL_DOWN: c_int = 2;
pub const UR_DRIVE_MASK: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ur_port_desc {
    pub pin_base: u32,
    pub npins: u32,
    pub func_offset: u32,
    pub conf_offset: u32,
    pub supported_modes: u32,
    pub supports_gpio: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ur_func_route {
    pub function: *const c_char,
    pub mode: u32,
    pub valid_pins: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ur_pinctrl_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: u32,
    pub routes: *const ur_func_route,
    pub num_routes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ur_pinctrl {
    pub dev: *mut device,
    pub pctl_dev: *mut pinctrl_dev,
    pub data: *const ur_pinctrl_data,
    pub base: *mut void __iomem,
    pub /: *mut *mut raw_spinlock_t lock; / Protects mux and conf registers,
}
