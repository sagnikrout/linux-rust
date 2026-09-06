//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/adp1653.h
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
// include/media/i2c/adp1653.h
//
// Copyright (C) 2008--2011 Nokia Corporation
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//
// Contributors:
// Sakari Ailus <sakari.ailus@iki.fi>
// Tuukka Toivonen <tuukkat76@gmail.com>
//

// Register definitions
pub const ADP1653_REG_OUT_SEL: c_uint = 0x00;
pub const ADP1653_REG_OUT_SEL_HPLED_TORCH_MIN: c_uint = 0x01;
pub const ADP1653_REG_OUT_SEL_HPLED_TORCH_MAX: c_uint = 0x0b;
pub const ADP1653_REG_OUT_SEL_HPLED_FLASH_MIN: c_uint = 0x0c;
pub const ADP1653_REG_OUT_SEL_HPLED_FLASH_MAX: c_uint = 0x1f;
pub const ADP1653_REG_OUT_SEL_HPLED_SHIFT: c_int = 3;
pub const ADP1653_REG_OUT_SEL_ILED_MAX: c_uint = 0x07;
pub const ADP1653_REG_OUT_SEL_ILED_SHIFT: c_int = 0;
pub const ADP1653_REG_CONFIG: c_uint = 0x01;

pub const ADP1653_REG_CONFIG_TMR_SET_MAX: c_uint = 0x0f;
pub const ADP1653_REG_CONFIG_TMR_SET_SHIFT: c_int = 0;
pub const ADP1653_REG_SW_STROBE: c_uint = 0x02;

pub const ADP1653_REG_FAULT: c_uint = 0x03;

pub const ADP1653_INDICATOR_INTENSITY_MIN: c_int = 0;
pub const ADP1653_INDICATOR_INTENSITY_STEP: c_int = 2500;

pub const ADP1653_FLASH_INTENSITY_BASE: c_int = 35;
pub const ADP1653_FLASH_INTENSITY_STEP: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp1653_platform_data {
    pub on): *mut *mut *mut int (power)(struct v4l2_subdev sd, int,
    pub /: *mut *mut u32 max_flash_timeout; / flash light timeout in us,
    pub /: *mut *mut u32 max_flash_intensity; / led intensity, flash mode, mA,
    pub /: *mut *mut u32 max_torch_intensity; / led intensity, torch mode, mA,
    pub /: *mut *mut u32 max_indicator_intensity; / indicator led intensity, uA,
    pub /: *mut *mut *mut gpio_desc enable_gpio; / for device-tree based boot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp1653_flash {
    pub subdev: v4l2_subdev,
    pub platform_data: *mut adp1653_platform_data,
    pub ctrls: v4l2_ctrl_handler,
    pub led_mode: *mut v4l2_ctrl,
    pub flash_timeout: *mut v4l2_ctrl,
    pub flash_intensity: *mut v4l2_ctrl,
    pub torch_intensity: *mut v4l2_ctrl,
    pub indicator_intensity: *mut v4l2_ctrl,
    pub power_lock: mutex,
    pub power_count: c_int,
    pub fault: c_int,
}
