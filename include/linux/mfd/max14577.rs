//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max14577.h
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
// max14577.h - Driver for the Maxim 14577/77836
//
// Copyright (C) 2014 Samsung Electronics
// Chanwoo Choi <cw00.choi@samsung.com>
// Krzysztof Kozlowski <krzk@kernel.org>
//
// This driver is based on max8997.h
//
// MAX14577 has MUIC, Charger devices.
// The devices share the same I2C bus and interrupt line
// included in this mfd driver.
//
// MAX77836 has additional PMIC and Fuel-Gauge on different I2C slave
// addresses.
//

// MAX14577 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max14577_regulators {
    MAX14577_SAFEOUT = 0,
    MAX14577_CHARGER,

    MAX14577_REGULATOR_NUM,
}

// MAX77836 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77836_regulators {
    MAX77836_SAFEOUT = 0,
    MAX77836_CHARGER,
    MAX77836_LDO1,
    MAX77836_LDO2,

    MAX77836_REGULATOR_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14577_regulator_platform_data {
    pub id: c_int,
    pub initdata: *mut regulator_init_data,
    pub of_node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14577_charger_platform_data {
    pub constant_uvolt: u32,
    pub fast_charge_uamp: u32,
    pub eoc_uamp: u32,
    pub ovp_uvolt: u32,
}

//
// MAX14577 MFD platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max14577_platform_data {
// IRQ
    pub irq_base: c_int,
// current control GPIOs
    pub gpio_pogo_vbatt_en: c_int,
    pub gpio_pogo_vbus_en: c_int,
// current control GPIO control function
    pub gpio_val): *mut *mut int (set_gpio_pogo_vbatt_en) (int,
    pub gpio_val): *mut *mut int (set_gpio_pogo_vbus_en) (int,
    pub new_dev): *mut *mut int (set_gpio_pogo_cb) (int,
    pub regulators: *mut max14577_regulator_platform_data,
}

//
// Valid limits of current for max14577 and max77836 chargers.
// They must correspond to MBCICHWRCL and MBCICHWRCH fields in CHGCTRL4
// register for given chipset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maxim_charger_current {
// Minimal current, set in CHGCTRL4/MBCICHWRCL, uA
    pub min: c_uint,
//
// Minimal current when high setting is active,
// set in CHGCTRL4/MBCICHWRCH, uA
//
    pub high_start: c_uint,
// Value of one step in high setting, uA
    pub high_step: c_uint,
// Maximum current of high setting, uA
    pub max: c_uint,
}
