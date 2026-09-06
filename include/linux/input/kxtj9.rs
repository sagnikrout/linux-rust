//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/kxtj9.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2011 Kionix, Inc.
// Written by Chris Hudson <chudson@kionix.com>
//
pub const KXTJ9_I2C_ADDR: c_uint = 0x0F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kxtj9_platform_data {
    pub /: *mut *mut unsigned int min_interval; / minimum poll interval (in milli-seconds),
    pub /: *mut *mut unsigned int init_interval; / initial poll interval (in milli-seconds),
//
// By default, x is axis 0, y is axis 1, z is axis 2; these can be
// changed to account for sensor orientation within the host device.
//
    pub axis_map_x: u8,
    pub axis_map_y: u8,
    pub axis_map_z: u8,
//
// Each axis can be negated to account for sensor orientation within
// the host device.
//
    pub negate_x: bool,
    pub negate_y: bool,
    pub negate_z: bool,
// CTRL_REG1: set resolution, g-range, data ready enable
// Output resolution: 8-bit valid or 12-bit valid
pub const RES_8BIT: c_int = 0;

    pub res_12bit: u8,
// Output g-range: +/-2g, 4g, or 8g
pub const KXTJ9_G_2G: c_int = 0;

    pub g_range: u8,
    pub (*init)(void): *mut c_int,
    pub (*exit)(void): *mut c_void,
    pub (*power_on)(void): *mut c_int,
    pub (*power_off)(void): *mut c_int,
}
