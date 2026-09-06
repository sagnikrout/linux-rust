//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-mux-reg.h
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
// I2C multiplexer using a single register
//
// Copyright 2015 Freescale Semiconductor
// York Sun <yorksun@freescale.com>
//
// struct i2c_mux_reg_platform_data - Platform-dependent data for i2c-mux-reg
// @parent: Parent I2C bus adapter number
// @base_nr: Base I2C bus number to number adapters from or zero for dynamic
// @values: Array of value for each channel
// @n_values: Number of multiplexer channels
// @little_endian: Indicating if the register is in little endian
// @write_only: Reading the register is not allowed by hardware
// @idle: Value to write to mux when idle
// @idle_in_use: indicate if idle value is in use
// @reg: Virtual address of the register to switch channel
// @reg_size: register size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_mux_reg_platform_data {
    pub parent: c_int,
    pub base_nr: c_int,
    pub values: *const c_uint,
    pub n_values: c_int,
    pub little_endian: bool,
    pub write_only: bool,
    pub idle: u32,
    pub idle_in_use: bool,
    pub reg: *mut void __iomem,
    pub reg_size: resource_size_t,
}
