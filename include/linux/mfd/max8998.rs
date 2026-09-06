//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max8998.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// max8998.h - Voltage regulator driver for the Maxim 8998
//
// Copyright (C) 2009-2010 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
// Marek Szyprowski <m.szyprowski@samsung.com>
//

// MAX 8998 regulator ids
//
// max8998_regulator_data - regulator data
// @id: regulator id
// @initdata: regulator init data (contraints, supplies, ...)
// @reg_node: DT node of regulator (unused on non-DT platforms)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8998_regulator_data {
    pub id: c_int,
    pub initdata: *mut regulator_init_data,
    pub reg_node: *mut device_node,
}

//
// struct max8998_board - packages regulator init data
// @regulators: array of defined regulators
// @num_regulators: number of regulators used
// @irq_base: base IRQ number for max8998, required for IRQs
// @ono: power onoff IRQ number for max8998
// @buck_voltage_lock: Do NOT change the values of the following six
// registers set by buck?_voltage?. The voltage of BUCK1/2 cannot
// be other than the preset values.
// @buck1_voltage: BUCK1 DVS mode 1 voltage registers
// @buck2_voltage: BUCK2 DVS mode 2 voltage registers
// @buck1_default_idx: Default for BUCK1 gpio pin 1, 2
// @buck2_default_idx: Default for BUCK2 gpio pin.
// @wakeup: Allow to wake up from suspend
// @rtc_delay: LP3974 RTC chip bug that requires delay after a register
// write before reading it.
// @eoc: End of Charge Level in percent: 10% ~ 45% by 5% step
// If it equals 0, leave it unchanged.
// Otherwise, it is a invalid value.
// @restart: Restart Level in mV: 100, 150, 200, and -1 for disable.
// If it equals 0, leave it unchanged.
// Otherwise, it is a invalid value.
// @timeout: Full Timeout in hours: 5, 6, 7, and -1 for disable.
// If it equals 0, leave it unchanged.
// Otherwise, leave it unchanged.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8998_platform_data {
    pub regulators: *mut max8998_regulator_data,
    pub num_regulators: c_int,
    pub irq_base: c_uint,
    pub ono: c_int,
    pub buck_voltage_lock: bool,
    pub buck1_voltage: [c_int; 4],
    pub buck2_voltage: [c_int; 2],
    pub buck1_default_idx: c_int,
    pub buck2_default_idx: c_int,
    pub wakeup: bool,
    pub rtc_delay: bool,
    pub eoc: c_int,
    pub restart: c_int,
    pub timeout: c_int,
}
