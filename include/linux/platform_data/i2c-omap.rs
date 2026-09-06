//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-omap.h
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


// SPDX-License-Identifier: GPL-2.0

//
// Version 2 of the I2C peripheral unit has a different register
// layout and extra registers.  The ID register in the V2 peripheral
// unit on the OMAP4430 reports the same ID as the V1 peripheral
// unit on the OMAP3530, so we must inform the driver which IP
// version we know it is running on from platform / cpu-specific
// code using these constants in the hwmod class definition.
//
pub const OMAP_I2C_IP_VERSION_1: c_int = 1;
pub const OMAP_I2C_IP_VERSION_2: c_int = 2;
// struct omap_i2c_bus_platform_data .flags meanings

// how the CPU address bus must be translated for I2C unit access
pub const OMAP_I2C_FLAG_BUS_SHIFT_NONE: c_int = 0;

pub const OMAP_I2C_FLAG_BUS_SHIFT__SHIFT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_i2c_bus_platform_data {
    pub clkrate: u32,
    pub rev: u32,
    pub flags: u32,
    pub set): *mut *mut *mut void (set_mpu_wkup_lat)(struct device dev, long,
}
