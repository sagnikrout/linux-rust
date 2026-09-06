//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/cix/pinctrl-sky1.h
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
// Author: Jerry Zhu <Jerry.Zhu@cixtech.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_pinctrl_group {
    pub name: *const c_char,
    pub config: c_ulong,
    pub pin: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_pin_desc {
    pub pin: pinctrl_pin_desc,
    pub func_group: *const *const c_char,
    pub nfunc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_pinctrl_soc_info {
    pub pins: *const sky1_pin_desc,
    pub npins: c_uint,
}

//
// @dev: a pointer back to containing device
// @base: the offset to the controller in virtual memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub base: *mut void __iomem,
    pub info: *const sky1_pinctrl_soc_info,
    pub groups: *mut sky1_pinctrl_group,
    pub grp_names: *const c_char,
}
