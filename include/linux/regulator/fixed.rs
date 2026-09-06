//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/fixed.h
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
// fixed.h
//
// Copyright 2008 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Copyright (c) 2009 Nokia Corporation
// Roger Quadros <ext-roger.quadros@nokia.com>
//
// struct fixed_voltage_config - fixed_voltage_config structure
// @supply_name:	Name of the regulator supply
// @input_supply:	Name of the input regulator supply
// @microvolts:		Output voltage of regulator
// @startup_delay:	Start-up time in microseconds
// @enabled_at_boot:	Whether regulator has been enabled at
// boot or not. 1 = Yes, 0 = No
// This is used to keep the regulator at
// the default state
// @init_data:		regulator_init_data
//
// This structure contains fixed voltage regulator configuration
// information that must be passed by platform code to the fixed
// voltage regulator driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_voltage_config {
    pub supply_name: *const c_char,
    pub input_supply: *const c_char,
    pub microvolts: c_int,
    pub startup_delay: unsigned,
    pub off_on_delay: c_uint,
    pub enabled_at_boot:1: unsigned,
    pub init_data: *mut regulator_init_data,
}

