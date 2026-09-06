//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/freescale/pinctrl-imx1.h
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
// IMX pinmux core definitions
//
// Copyright (C) 2012 Freescale Semiconductor, Inc.
// Copyright (C) 2012 Linaro Ltd.
//
// Author: Dong Aisheng <dong.aisheng@linaro.org>
//
// struct imx1_pin - describes an IMX1/21/27 pin.
// @pin_id: ID of the described pin.
// @mux_id: ID of the mux setup.
// @config: Configuration of the pin (currently only pullup-enable).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx1_pin {
    pub pin_id: c_uint,
    pub mux_id: c_uint,
    pub config: c_ulong,
}

//
// struct imx1_pin_group - describes an IMX pin group
// @name: the name of this specific pin group
// @pins: an array of imx1_pin structs used in this group
// @npins: the number of pins in this group array, i.e. the number of
// elements in .pins so we can iterate over that array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx1_pin_group {
    pub name: *const c_char,
    pub pin_ids: *mut c_uint,
    pub pins: *mut imx1_pin,
    pub npins: unsigned,
}

//
// struct imx1_pmx_func - describes IMX pinmux functions
// @name: the name of this specific function
// @groups: corresponding pin groups
// @num_groups: the number of groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx1_pmx_func {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub num_groups: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx1_pinctrl_soc_info {
    pub dev: *mut device,
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub groups: *mut imx1_pin_group,
    pub ngroups: c_uint,
    pub functions: *mut imx1_pmx_func,
    pub nfunctions: c_uint,
}

