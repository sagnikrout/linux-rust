//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/bq2415x_charger.h
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
// bq2415x charger driver
//
// Copyright (C) 2011-2013  Pali Rohár <pali@kernel.org>
//
// This is platform data for bq2415x chip. It contains default board
// voltages and currents which can be also later configured via sysfs. If
// value is -1 then default chip value (specified in datasheet) will be
// used.
//
// Value resistor_sense is needed for configuring charge and
// termination current. If it is less or equal to zero, configuring charge
// and termination current will not be possible.
//
// For automode support is needed to provide name of power supply device
// in value notify_device. Device driver must immediately report property
// POWER_SUPPLY_PROP_CURRENT_MAX when current changed.
//
// Supported modes with maximal current limit
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bq2415x_mode {
    BQ2415X_MODE_OFF,		/* offline mode (charger disabled) */
    BQ2415X_MODE_NONE,		/* unknown charger (100mA) */
    BQ2415X_MODE_HOST_CHARGER,	/* usb host/hub charger (500mA) */
    BQ2415X_MODE_DEDICATED_CHARGER, /* dedicated charger (unlimited) */
    BQ2415X_MODE_BOOST,		/* boost mode (charging disabled) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq2415x_platform_data {
    pub /: *mut *mut int current_limit; / mA,
    pub /: *mut *mut int weak_battery_voltage; / mV,
    pub /: *mut *mut int battery_regulation_voltage; / mV,
    pub /: *mut *mut int charge_current; / mA,
    pub /: *mut *mut int termination_current; / mA,
    pub /: *mut *mut int resistor_sense; / m ohm,
    pub /: *const *const *const char notify_device; / name,
}
