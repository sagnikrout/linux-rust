//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lp87565.h
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
// Functions to access LP87565 power management chip.
//
// Copyright (C) 2017 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp87565_device_type {
    LP87565_DEVICE_TYPE_UNKNOWN	= 0,
    LP87565_DEVICE_TYPE_LP87524_Q1,
    LP87565_DEVICE_TYPE_LP87561_Q1,
    LP87565_DEVICE_TYPE_LP87565_Q1,
}

// All register addresses

pub const LP87565_REG_BUCK_2_3_STAT: c_uint = 0x20;
pub const LP87565_REG_TOP_MASK_1: c_uint = 0x21;
pub const LP87565_REG_TOP_MASK_2: c_uint = 0x22;
pub const LP87565_REG_BUCK_0_1_MASK: c_uint = 0x23;
pub const LP87565_REG_BUCK_2_3_MASK: c_uint = 0x24;
pub const LP87565_REG_SEL_I_LOAD: c_uint = 0x25;
pub const LP87565_REG_I_LOAD_2: c_uint = 0x26;
pub const LP87565_REG_I_LOAD_1: c_uint = 0x27;
pub const LP87565_REG_PGOOD_CTRL1: c_uint = 0x28;
pub const LP87565_REG_PGOOD_CTRL2: c_uint = 0x29;
pub const LP87565_REG_PGOOD_FLT: c_uint = 0x2A;
pub const LP87565_REG_PLL_CTRL: c_uint = 0x2B;
pub const LP87565_REG_PIN_FUNCTION: c_uint = 0x2C;
pub const LP87565_REG_GPIO_CONFIG: c_uint = 0x2D;
pub const LP87565_REG_GPIO_IN: c_uint = 0x2E;
pub const LP87565_REG_GPIO_OUT: c_uint = 0x2F;

// Register field definitions
pub const LP87565_DEV_REV_DEV_ID: c_uint = 0xC0;
pub const LP87565_DEV_REV_ALL_LAYER: c_uint = 0x30;
pub const LP87565_DEV_REV_METAL_LAYER: c_uint = 0x0F;
pub const LP87565_OTP_REV_OTP_ID: c_uint = 0xFF;

pub const LP87565_BUCK_CTRL_1_PIN_SELECT_EN: c_uint = 0x30;

// Bit0 is reserved for BUCK1 and BUCK3 and valid only for BUCK0 and BUCK2

pub const LP87565_BUCK_CTRL_2_ILIM: c_uint = 0x38;
pub const LP87565_BUCK_CTRL_2_SLEW_RATE: c_uint = 0x07;
pub const LP87565_BUCK_VSET: c_uint = 0xFF;
pub const LP87565_BUCK_FLOOR_VSET: c_uint = 0xFF;
pub const LP87565_BUCK_SHUTDOWN_DELAY: c_uint = 0xF0;
pub const LP87565_BUCK_STARTUP_DELAY: c_uint = 0x0F;
pub const LP87565_GPIO_SHUTDOWN_DELAY: c_uint = 0xF0;
pub const LP87565_GPIO_STARTUP_DELAY: c_uint = 0x0F;

pub const LP87565_LOAD_CURRENT_BUCK_SELECT: c_uint = 0x3;
pub const LP87565_I_LOAD2_BUCK_LOAD_CURRENT: c_uint = 0x3;
pub const LP87565_I_LOAD1_BUCK_LOAD_CURRENT: c_uint = 0xFF;
pub const LP87565_PG3_SEL: c_uint = 0xC0;
pub const LP87565_PG2_SEL: c_uint = 0x30;
pub const LP87565_PG1_SEL: c_uint = 0x0C;
pub const LP87565_PG0_SEL: c_uint = 0x03;

pub const LP87565_PLL_MODE: c_uint = 0xC0;
pub const LP87565_EXT_CLK_FREQ: c_uint = 0x1F;

//
// struct LP87565 - state holder for the LP87565 driver
// @dev: struct device pointer for MFD device
// @rev: revision of the LP87565
// @dev_type: The device type for example lp87565-q1
// @lock: lock guarding the data structure
// @regmap: register map of the LP87565 PMIC
//
// Device data may be used to access the LP87565 chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp87565 {
    pub dev: *mut device,
    pub rev: u8,
    pub dev_type: u8,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
}
