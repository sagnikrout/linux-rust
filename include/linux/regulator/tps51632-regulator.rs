//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/tps51632-regulator.h
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
// tps51632-regulator.h -- TPS51632 regulator
//
// Interface for regulator driver for TPS51632 3-2-1 Phase D-Cap Step Down
// Driverless Controller with serial VID control and DVFS.
//
// Copyright (C) 2012 NVIDIA Corporation
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// struct tps51632_regulator_platform_data - tps51632 regulator platform data.
//
// @reg_init_data: The regulator init data.
// @enable_pwm_dvfs: Enable PWM DVFS or not.
// @dvfs_step_20mV: Step for DVFS is 20mV or 10mV.
// @max_voltage_uV: Maximum possible voltage in PWM-DVFS mode.
// @base_voltage_uV: Base voltage when PWM-DVFS enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps51632_regulator_platform_data {
    pub reg_init_data: *mut regulator_init_data,
    pub enable_pwm_dvfs: bool,
    pub dvfs_step_20mV: bool,
    pub max_voltage_uV: c_int,
    pub base_voltage_uV: c_int,
}
