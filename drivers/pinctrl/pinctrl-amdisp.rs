//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-amdisp.h
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
// AMD ISP Pinctrl Driver
//
// Copyright (C) 2025 Advanced Micro Devices, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdisp_functions {
    mux_gpio,
    mux_NA
}

//
// struct amdisp_function - a pinmux function
// @name:    Name of the pinmux function.
// @groups:  List of pingroups for this function.
// @ngroups: Number of entries in @groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdisp_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: c_uint,
}

//
// struct amdisp_pingroup - a pinmux group
// @name:  Name of the pinmux group.
// @pins:  List of pins for this group.
// @npins: Number of entries in @pins.
// @funcs: List of functions belongs to this group.
// @nfuncs: Number of entries in @funcs.
// @offset: Group offset in amdisp pinmux groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdisp_pingroup {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub npins: c_uint,
    pub funcs: *mut c_uint,
    pub nfuncs: c_uint,
    pub offset: c_uint,
}
