//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/nxp/pinctrl-s32.h
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
// S32 pinmux core definitions
//
// Copyright 2016-2020, 2022 NXP
// Copyright (C) 2022 SUSE LLC
// Copyright 2015-2016 Freescale Semiconductor, Inc.
// Copyright (C) 2012 Linaro Ltd.
//
// struct s32_pin_group - describes an S32 pin group
// @data: generic data describes group name, number of pins, and a pin array in
// this group.
// @pin_sss: an array of source signal select configs paired with pin array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32_pin_group {
    pub data: pingroup,
    pub pin_sss: *mut c_uint,
}

//
// struct s32_pin_range - pin ID range for each memory region.
// @start: start pin ID
// @end: end pin ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32_pin_range {
    pub start: c_uint,
    pub end: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32_pinctrl_soc_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub mem_pin_ranges: *const s32_pin_range,
    pub mem_regions: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32_pinctrl_soc_info {
    pub dev: *mut device,
    pub soc_data: *const s32_pinctrl_soc_data,
    pub groups: *mut s32_pin_group,
    pub ngroups: c_uint,
    pub functions: *mut pinfunction,
    pub nfunctions: c_uint,
    pub grp_index: c_uint,
}

extern "C" {
    pub fn s32_pinctrl_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn s32_pinctrl_suspend(dev: *mut device) -> c_int;
}
