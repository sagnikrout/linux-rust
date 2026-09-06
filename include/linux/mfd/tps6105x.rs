//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps6105x.h
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
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// Register definitions to all subdrivers
//
pub const TPS6105X_REG_0: c_uint = 0x00;
pub const TPS6105X_REG0_MODE_SHIFT: c_int = 6;

// These defines for both reg0 and reg1
pub const TPS6105X_REG0_MODE_SHUTDOWN: c_uint = 0x00;
pub const TPS6105X_REG0_MODE_TORCH: c_uint = 0x01;
pub const TPS6105X_REG0_MODE_TORCH_FLASH: c_uint = 0x02;
pub const TPS6105X_REG0_MODE_VOLTAGE: c_uint = 0x03;
pub const TPS6105X_REG0_VOLTAGE_SHIFT: c_int = 4;

pub const TPS6105X_REG0_VOLTAGE_450: c_int = 0;
pub const TPS6105X_REG0_VOLTAGE_500: c_int = 1;
pub const TPS6105X_REG0_VOLTAGE_525: c_int = 2;
pub const TPS6105X_REG0_VOLTAGE_500_2: c_int = 3;
pub const TPS6105X_REG0_DIMMING_SHIFT: c_int = 3;
pub const TPS6105X_REG0_TORCHC_SHIFT: c_int = 0;

pub const TPS6105X_REG0_TORCHC_0: c_uint = 0x00;
pub const TPS6105X_REG0_TORCHC_50: c_uint = 0x01;
pub const TPS6105X_REG0_TORCHC_75: c_uint = 0x02;
pub const TPS6105X_REG0_TORCHC_100: c_uint = 0x03;
pub const TPS6105X_REG0_TORCHC_150: c_uint = 0x04;
pub const TPS6105X_REG0_TORCHC_200: c_uint = 0x05;
pub const TPS6105X_REG0_TORCHC_250_400: c_uint = 0x06;
pub const TPS6105X_REG0_TORCHC_250_500: c_uint = 0x07;
pub const TPS6105X_REG_1: c_uint = 0x01;
pub const TPS6105X_REG1_MODE_SHIFT: c_int = 6;

pub const TPS6105X_REG1_MODE_SHUTDOWN: c_uint = 0x00;
pub const TPS6105X_REG1_MODE_TORCH: c_uint = 0x01;
pub const TPS6105X_REG1_MODE_TORCH_FLASH: c_uint = 0x02;
pub const TPS6105X_REG1_MODE_VOLTAGE: c_uint = 0x03;
pub const TPS6105X_REG_2: c_uint = 0x02;
pub const TPS6105X_REG_3: c_uint = 0x03;
//
// enum tps6105x_mode - desired mode for the TPS6105x
// @TPS6105X_MODE_SHUTDOWN: this instance is inactive, not used for anything
// @TPS6105X_MODE_TORCH: this instance is used as a LED, usually a while
// LED, for example as backlight or flashlight. If this is set, the
// TPS6105X will register to the LED framework
// @TPS6105X_MODE_TORCH_FLASH: this instance is used as a flashgun, usually
// in a camera
// @TPS6105X_MODE_VOLTAGE: this instance is used as a voltage regulator and
// will register to the regulator framework
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps6105x_mode {
    TPS6105X_MODE_SHUTDOWN,
    TPS6105X_MODE_TORCH,
    TPS6105X_MODE_TORCH_FLASH,
    TPS6105X_MODE_VOLTAGE,
}

//
// struct tps6105x_platform_data - TPS61905x platform data
// @mode: what mode this instance shall be operated in,
// this is not selectable at runtime
// @regulator_data: initialization data for the voltage
// regulator if used as a voltage source
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6105x_platform_data {
    pub mode: tps6105x_mode,
    pub regulator_data: *mut regulator_init_data,
}

//
// struct tps6105x - state holder for the TPS6105x drivers
// @pdata: associated platform data
// @client: corresponding I2C client
// @regulator: regulator device if used in voltage mode
// @regmap: used for i2c communcation on accessing registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6105x {
    pub pdata: *mut tps6105x_platform_data,
    pub client: *mut i2c_client,
    pub regulator: *mut regulator_dev,
    pub regmap: *mut regmap,
}
