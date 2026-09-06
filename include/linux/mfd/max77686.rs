//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77686.h
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
// max77686.h - Driver for the Maxim 77686/802
//
// Copyright (C) 2012 Samsung Electronics
// Chiwoong Byun <woong.byun@samsung.com>
//
// This driver is based on max8997.h
//
// MAX77686 has PMIC, RTC devices.
// The devices share the same I2C bus and included in
// this mfd driver.
//

// MAX77686 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77686_regulators {
    MAX77686_LDO1 = 0,
    MAX77686_LDO2,
    MAX77686_LDO3,
    MAX77686_LDO4,
    MAX77686_LDO5,
    MAX77686_LDO6,
    MAX77686_LDO7,
    MAX77686_LDO8,
    MAX77686_LDO9,
    MAX77686_LDO10,
    MAX77686_LDO11,
    MAX77686_LDO12,
    MAX77686_LDO13,
    MAX77686_LDO14,
    MAX77686_LDO15,
    MAX77686_LDO16,
    MAX77686_LDO17,
    MAX77686_LDO18,
    MAX77686_LDO19,
    MAX77686_LDO20,
    MAX77686_LDO21,
    MAX77686_LDO22,
    MAX77686_LDO23,
    MAX77686_LDO24,
    MAX77686_LDO25,
    MAX77686_LDO26,
    MAX77686_BUCK1,
    MAX77686_BUCK2,
    MAX77686_BUCK3,
    MAX77686_BUCK4,
    MAX77686_BUCK5,
    MAX77686_BUCK6,
    MAX77686_BUCK7,
    MAX77686_BUCK8,
    MAX77686_BUCK9,

    MAX77686_REG_MAX,
}

// MAX77802 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77802_regulators {
    MAX77802_BUCK1 = 0,
    MAX77802_BUCK2,
    MAX77802_BUCK3,
    MAX77802_BUCK4,
    MAX77802_BUCK5,
    MAX77802_BUCK6,
    MAX77802_BUCK7,
    MAX77802_BUCK8,
    MAX77802_BUCK9,
    MAX77802_BUCK10,
    MAX77802_LDO1,
    MAX77802_LDO2,
    MAX77802_LDO3,
    MAX77802_LDO4,
    MAX77802_LDO5,
    MAX77802_LDO6,
    MAX77802_LDO7,
    MAX77802_LDO8,
    MAX77802_LDO9,
    MAX77802_LDO10,
    MAX77802_LDO11,
    MAX77802_LDO12,
    MAX77802_LDO13,
    MAX77802_LDO14,
    MAX77802_LDO15,
    MAX77802_LDO17,
    MAX77802_LDO18,
    MAX77802_LDO19,
    MAX77802_LDO20,
    MAX77802_LDO21,
    MAX77802_LDO23,
    MAX77802_LDO24,
    MAX77802_LDO25,
    MAX77802_LDO26,
    MAX77802_LDO27,
    MAX77802_LDO28,
    MAX77802_LDO29,
    MAX77802_LDO30,
    MAX77802_LDO32,
    MAX77802_LDO33,
    MAX77802_LDO34,
    MAX77802_LDO35,

    MAX77802_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77686_opmode {
    MAX77686_OPMODE_NORMAL,
    MAX77686_OPMODE_LP,
    MAX77686_OPMODE_STANDBY,
}
