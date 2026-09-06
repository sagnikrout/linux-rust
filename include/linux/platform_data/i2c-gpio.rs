//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-gpio.h
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
// i2c-gpio interface to platform code
//
// Copyright (C) 2007 Atmel Corporation
//
// struct i2c_gpio_platform_data - Platform-dependent data for i2c-gpio
// @udelay: signal toggle delay. SCL frequency is (500 / udelay) kHz
// @timeout: clock stretching timeout in jiffies. If the slave keeps
// SCL low for longer than this, the transfer will time out.
// @sda_is_open_drain: SDA is configured as open drain, i.e. the pin
// isn't actively driven high when setting the output value high.
// gpio_get_value() must return the actual pin state even if the
// pin is configured as an output.
// @sda_is_output_only: SDA output drivers can't be turned off.
// This is for clients that can only read SDA/SCL.
// @sda_has_no_pullup: SDA is used in a non-compliant way and has no pull-up.
// Therefore disable open-drain.
// @scl_is_open_drain: SCL is set up as open drain. Same requirements
// as for sda_is_open_drain apply.
// @scl_is_output_only: SCL output drivers cannot be turned off.
// @scl_has_no_pullup: SCL is used in a non-compliant way and has no pull-up.
// Therefore disable open-drain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_gpio_platform_data {
    pub udelay: c_int,
    pub timeout: c_int,
    pub sda_is_open_drain:1: c_uint,
    pub sda_is_output_only:1: c_uint,
    pub sda_has_no_pullup:1: c_uint,
    pub scl_is_open_drain:1: c_uint,
    pub scl_is_output_only:1: c_uint,
    pub scl_has_no_pullup:1: c_uint,
}
