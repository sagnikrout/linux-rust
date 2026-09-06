//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/bcm590xx.h
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
// Broadcom BCM590xx PMU
//
// Copyright 2014 Linaro Limited
// Author: Matt Porter <mporter@linaro.org>
//

// PMU ID register values; also used as device type
pub const BCM590XX_PMUID_BCM59054: c_uint = 0x54;
pub const BCM590XX_PMUID_BCM59056: c_uint = 0x56;
// Known chip revision IDs
pub const BCM59054_REV_DIGITAL_A1: c_int = 1;
pub const BCM59054_REV_ANALOG_A1: c_int = 2;
pub const BCM59056_REV_DIGITAL_A0: c_int = 1;
pub const BCM59056_REV_ANALOG_A0: c_int = 1;
pub const BCM59056_REV_DIGITAL_B0: c_int = 2;
pub const BCM59056_REV_ANALOG_B0: c_int = 2;
// regmap types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm590xx_regmap_type {
    BCM590XX_REGMAP_PRI,
    BCM590XX_REGMAP_SEC,
}

// max register address
pub const BCM590XX_MAX_REGISTER_PRI: c_uint = 0xe7;
pub const BCM590XX_MAX_REGISTER_SEC: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm590xx {
    pub dev: *mut device,
    pub i2c_pri: *mut i2c_client,
    pub i2c_sec: *mut i2c_client,
    pub regmap_pri: *mut regmap,
    pub regmap_sec: *mut regmap,
// PMU ID value; also used as device type
    pub pmu_id: u8,
// Chip revision, read from PMUREV reg
    pub rev_digital: u8,
    pub rev_analog: u8,
}
