//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/ad714x.h
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
// include/linux/input/ad714x.h
//
// AD714x is very flexible, it can be used as buttons, scrollwheel,
// slider, touchpad at the same time. That depends on the boards.
// The platform_data for the device's "struct device" holds this
// information.
//
// Copyright 2009-2011 Analog Devices Inc.
//
pub const STAGE_NUM: c_int = 12;
pub const STAGE_CFGREG_NUM: c_int = 8;
pub const SYS_CFGREG_NUM: c_int = 8;
// board information which need be initialized in arch/mach...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_slider_plat {
    pub start_stage: c_int,
    pub end_stage: c_int,
    pub max_coord: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_wheel_plat {
    pub start_stage: c_int,
    pub end_stage: c_int,
    pub max_coord: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_touchpad_plat {
    pub x_start_stage: c_int,
    pub x_end_stage: c_int,
    pub x_max_coord: c_int,
    pub y_start_stage: c_int,
    pub y_end_stage: c_int,
    pub y_max_coord: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_button_plat {
    pub keycode: c_int,
    pub l_mask: c_ushort,
    pub h_mask: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_platform_data {
    pub slider_num: c_int,
    pub wheel_num: c_int,
    pub touchpad_num: c_int,
    pub button_num: c_int,
    pub slider: *mut ad714x_slider_plat,
    pub wheel: *mut ad714x_wheel_plat,
    pub touchpad: *mut ad714x_touchpad_plat,
    pub button: *mut ad714x_button_plat,
    pub stage_cfg_reg: [c_ushort; STAGE_NUM][STAGE_CFGREG_NUM],
    pub sys_cfg_reg: [c_ushort; SYS_CFGREG_NUM],
    pub irqflags: c_ulong,
}
