//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-mux-gpio.h
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
// i2c-mux-gpio interface to platform code
//
// Peter Korsgaard <peter.korsgaard@barco.com>
//
// MUX has no specific idle mode

//
// struct i2c_mux_gpio_platform_data - Platform-dependent data for i2c-mux-gpio
// @parent: Parent I2C bus adapter number
// @base_nr: Base I2C bus number to number adapters from or zero for dynamic
// @values: Array of bitmasks of GPIO settings (low/high) for each
// position
// @n_values: Number of multiplexer positions (busses to instantiate)
// @idle: Bitmask to write to MUX when idle or GPIO_I2CMUX_NO_IDLE if not used
// @settle_time: Delay to wait when a new bus is selected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_mux_gpio_platform_data {
    pub parent: c_int,
    pub base_nr: c_int,
    pub values: *const unsigned,
    pub n_values: c_int,
    pub idle: unsigned,
    pub settle_time: u32,
}
