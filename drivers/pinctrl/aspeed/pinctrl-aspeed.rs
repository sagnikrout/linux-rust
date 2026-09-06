//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/aspeed/pinctrl-aspeed.h
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
// Copyright (C) 2016 IBM Corp.
//

// Macro flag: #define PINCTRL_ASPEED

//
// @param The pinconf parameter type
// @pins The pin range this config struct covers, [low, high]
// @reg The register housing the configuration bits
// @mask The mask to select the bits of interest in @reg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_pin_config {
    pub param: pin_config_param,
    pub pins: [c_uint; 2],
    pub reg: c_uint,
    pub mask: u32,
}

//
// Aspeed pin configuration description.
//
// @param: pinconf configuration parameter
// @arg: The supported argument for @param, or -1 if any value is supported
// @val: The register value to write to configure @arg for @param
// @mask: The bitfield mask for @val
//
// The map is to be used in conjunction with the configuration array supplied
// by the driver implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_pin_config_map {
    pub param: pin_config_param,
    pub arg: i32,
    pub val: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_pinctrl_data {
    pub scu: *mut regmap,
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub configs: *const aspeed_pin_config,
    pub nconfigs: c_uint,
    pub pinmux: aspeed_pinmux_data,
    pub confmaps: *const aspeed_pin_config_map,
    pub nconfmaps: c_uint,
}

// Aspeed pinctrl helpers
extern "C" {
    pub fn aspeed_pinctrl_get_groups_count(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn aspeed_pinmux_get_fn_count(pctldev: *mut pinctrl_dev) -> c_int;
}
