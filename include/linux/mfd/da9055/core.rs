//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9055/core.h
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
// da9055 declarations for DA9055 PMICs.
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

//
// PMIC IRQ
//
pub const DA9055_IRQ_ALARM: c_uint = 0x01;
pub const DA9055_IRQ_TICK: c_uint = 0x02;
pub const DA9055_IRQ_NONKEY: c_uint = 0x00;
pub const DA9055_IRQ_REGULATOR: c_uint = 0x0B;
pub const DA9055_IRQ_HWMON: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9055 {
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub irq_base: c_int,
    pub chip_irq: c_int,
}

// Device I/O
extern "C" {
    pub fn regmap_write(_arg: da9055->regmap, _arg: reg, _arg: val) -> return;
}
extern "C" {
    pub fn regmap_bulk_read(_arg: da9055->regmap, _arg: reg, _arg: val, _arg: reg_cnt) -> return;
}
extern "C" {
    pub fn regmap_raw_write(_arg: da9055->regmap, _arg: reg, _arg: val, _arg: reg_cnt) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: da9055->regmap, _arg: reg, _arg: bit_mask, _arg: reg_val) -> return;
}
// Generic Device API
extern "C" {
    pub fn da9055_device_init(da9055: *mut da9055) -> c_int;
}
extern "C" {
    pub fn da9055_device_exit(da9055: *mut da9055);
}
