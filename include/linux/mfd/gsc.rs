//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/gsc.h
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
// Copyright (C) 2020 Gateworks Corporation
//

// Device Addresses
pub const GSC_MISC: c_uint = 0x20;
pub const GSC_UPDATE: c_uint = 0x21;
pub const GSC_GPIO: c_uint = 0x23;
pub const GSC_HWMON: c_uint = 0x29;
pub const GSC_EEPROM0: c_uint = 0x50;
pub const GSC_EEPROM1: c_uint = 0x51;
pub const GSC_EEPROM2: c_uint = 0x52;
pub const GSC_EEPROM3: c_uint = 0x53;
pub const GSC_RTC: c_uint = 0x68;
// Register offsets
// Bit definitions
pub const GSC_CTRL_0_PB_HARD_RESET: c_int = 0;
pub const GSC_CTRL_0_PB_CLEAR_SECURE_KEY: c_int = 1;
pub const GSC_CTRL_0_PB_SOFT_POWER_DOWN: c_int = 2;
pub const GSC_CTRL_0_PB_BOOT_ALTERNATE: c_int = 3;
pub const GSC_CTRL_0_PERFORM_CRC: c_int = 4;
pub const GSC_CTRL_0_TAMPER_DETECT: c_int = 5;
pub const GSC_CTRL_0_SWITCH_HOLD: c_int = 6;
pub const GSC_CTRL_1_SLEEP_ENABLE: c_int = 0;
pub const GSC_CTRL_1_SLEEP_ACTIVATE: c_int = 1;
pub const GSC_CTRL_1_SLEEP_ADD: c_int = 2;
pub const GSC_CTRL_1_SLEEP_NOWAKEPB: c_int = 3;
pub const GSC_CTRL_1_WDT_TIME: c_int = 4;
pub const GSC_CTRL_1_WDT_ENABLE: c_int = 5;
pub const GSC_CTRL_1_SWITCH_BOOT_ENABLE: c_int = 6;
pub const GSC_CTRL_1_SWITCH_BOOT_CLEAR: c_int = 7;
pub const GSC_IRQ_PB: c_int = 0;
pub const GSC_IRQ_KEY_ERASED: c_int = 1;
pub const GSC_IRQ_EEPROM_WP: c_int = 2;
pub const GSC_IRQ_RESV: c_int = 3;
pub const GSC_IRQ_GPIO: c_int = 4;
pub const GSC_IRQ_TAMPER: c_int = 5;
pub const GSC_IRQ_WDT_TIMEOUT: c_int = 6;
pub const GSC_IRQ_SWITCH_HOLD: c_int = 7;
extern "C" {
    pub fn gsc_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn gsc_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_dev {
    pub dev: *mut device,
    pub /: *mut *mut *mut i2c_client i2c; / 0x20: interrupt controller, WDT,
    pub /: *mut *mut *mut i2c_client i2c_hwmon; / 0x29: hwmon, fan controller,
    pub regmap: *mut regmap,
    pub fwver: c_uint,
    pub fwcrc: c_ushort,
}
