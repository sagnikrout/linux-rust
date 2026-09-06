//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/upboard-fpga.h
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
// UP Board CPLD/FPGA driver
//
// Copyright (c) AAEON. All rights reserved.
// Copyright (C) 2024 Bootlin
//
// Author: Gary Wang <garywang@aaeon.com.tw>
// Author: Thomas Richard <thomas.richard@bootlin.com>
//
pub const UPBOARD_REGISTER_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upboard_fpgareg {
    UPBOARD_REG_PLATFORM_ID   = 0x10,
    UPBOARD_REG_FIRMWARE_ID   = 0x11,
    UPBOARD_REG_FUNC_EN0      = 0x20,
    UPBOARD_REG_FUNC_EN1      = 0x21,
    UPBOARD_REG_GPIO_EN0      = 0x30,
    UPBOARD_REG_GPIO_EN1      = 0x31,
    UPBOARD_REG_GPIO_EN2      = 0x32,
    UPBOARD_REG_GPIO_DIR0     = 0x40,
    UPBOARD_REG_GPIO_DIR1     = 0x41,
    UPBOARD_REG_GPIO_DIR2     = 0x42,
    UPBOARD_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum upboard_fpga_type {
    UPBOARD_UP_FPGA,
    UPBOARD_UP2_FPGA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_fpga_data {
    pub type: upboard_fpga_type,
    pub regmap_config: *const regmap_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_fpga {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
    pub clear_gpio: *mut gpio_desc,
    pub strobe_gpio: *mut gpio_desc,
    pub datain_gpio: *mut gpio_desc,
    pub dataout_gpio: *mut gpio_desc,
    pub firmware_version: c_uint,
    pub fpga_data: *const upboard_fpga_data,
}
