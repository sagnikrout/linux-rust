//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/gpio-regulator.h
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
// gpio-regulator.h
//
// Copyright 2011 Heiko Stuebner <heiko@sntech.de>
//
// based on fixed.h
//
// Copyright 2008 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Copyright (c) 2009 Nokia Corporation
// Roger Quadros <ext-roger.quadros@nokia.com>
//

//
// struct gpio_regulator_state - state description
// @value:		microvolts or microamps
// @gpios:		bitfield of gpio target-states for the value
//
// This structure describes a supported setting of the regulator
// and the necessary gpio-state to achieve it.
//
// The n-th bit in the bitfield describes the state of the n-th GPIO
// from the gpios-array defined in gpio_regulator_config below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_regulator_state {
    pub value: c_int,
    pub gpios: c_int,
}

//
// struct gpio_regulator_config - config structure
// @supply_name:	Name of the regulator supply
// @input_supply:	Name of the input regulator supply
// @enabled_at_boot:	Whether regulator has been enabled at
// boot or not. 1 = Yes, 0 = No
// This is used to keep the regulator at
// the default state
// @startup_delay:	Start-up time in microseconds
// @gflags:		Array of GPIO configuration flags for initial
// states
// @ngpios:		Number of GPIOs and configurations available
// @states:		Array of gpio_regulator_state entries describing
// the gpio state for specific voltages
// @nr_states:		Number of states available
// @regulator_type:	either REGULATOR_CURRENT or REGULATOR_VOLTAGE
// @init_data:		regulator_init_data
//
// This structure contains gpio-voltage regulator configuration
// information that must be passed by platform code to the
// gpio-voltage regulator driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_regulator_config {
    pub supply_name: *const c_char,
    pub input_supply: *const c_char,
    pub enabled_at_boot:1: unsigned,
    pub startup_delay: unsigned,
    pub gflags: *mut gpiod_flags,
    pub ngpios: c_int,
    pub states: *mut gpio_regulator_state,
    pub nr_states: c_int,
    pub type: regulator_type,
    pub init_data: *mut regulator_init_data,
}
