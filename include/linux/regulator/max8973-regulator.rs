//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/max8973-regulator.h
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
// max8973-regulator.h -- MAXIM 8973 regulator
//
// Interface for regulator driver for MAXIM 8973 DC-DC step-down
// switching regulator.
//
// Copyright (C) 2012 NVIDIA Corporation
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// Control flags for configuration of the device.
// Client need to pass this information with ORed
//
pub const MAX8973_CONTROL_REMOTE_SENSE_ENABLE: c_uint = 0x00000001;
pub const MAX8973_CONTROL_FALLING_SLEW_RATE_ENABLE: c_uint = 0x00000002;
pub const MAX8973_CONTROL_OUTPUT_ACTIVE_DISCH_ENABLE: c_uint = 0x00000004;
pub const MAX8973_CONTROL_BIAS_ENABLE: c_uint = 0x00000008;
pub const MAX8973_CONTROL_PULL_DOWN_ENABLE: c_uint = 0x00000010;
pub const MAX8973_CONTROL_FREQ_SHIFT_9PER_ENABLE: c_uint = 0x00000020;
pub const MAX8973_CONTROL_CLKADV_TRIP_DISABLED: c_uint = 0x00000000;
pub const MAX8973_CONTROL_CLKADV_TRIP_75mV_PER_US: c_uint = 0x00010000;
pub const MAX8973_CONTROL_CLKADV_TRIP_150mV_PER_US: c_uint = 0x00020000;
pub const MAX8973_CONTROL_CLKADV_TRIP_75mV_PER_US_HIST_DIS: c_uint = 0x00030000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_NOMINAL: c_uint = 0x00000000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_MINUS_30_PER: c_uint = 0x00100000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_PLUS_30_PER: c_uint = 0x00200000;
pub const MAX8973_CONTROL_INDUCTOR_VALUE_PLUS_60_PER: c_uint = 0x00300000;
//
// struct max8973_regulator_platform_data - max8973 regulator platform data.
//
// @reg_init_data: The regulator init data.
// @control_flags: Control flags which are ORed value of above flags to
// configure device.
// @junction_temp_warning: Junction temp in millicelcius on which warning need
// to be set. Thermal functionality is only supported on
// MAX77621. The threshold warning supported by MAX77621
// are 120C and 140C.
// @enable_ext_control: Enable the voltage enable/disable through external
// control signal from EN input pin. If it is false then
// voltage output will be enabled/disabled through EN bit of
// device register.
// @dvs_def_state: Default state of dvs. 1 if it is high else 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8973_regulator_platform_data {
    pub reg_init_data: *mut regulator_init_data,
    pub control_flags: c_ulong,
    pub junction_temp_warning: c_ulong,
    pub enable_ext_control: bool,
    pub dvs_def_state:1: unsigned,
}
