//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max8925.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Maxim8925 Interface
//
// Copyright (C) 2009 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

// Unified sub device IDs for MAX8925
//
// Charging current threshold trigger going from fast charge
// to TOPOFF charge. From 5% to 20% of fasting charging current.
//
// Fast charging current
// Charger registers

// GPM registers

// Touch registers

// RTC registers

// WLED registers

// MAX8925 Registers

// bit definitions

// IRQ definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_chip {
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub adc: *mut i2c_client,
    pub rtc: *mut i2c_client,
    pub io_lock: mutex,
    pub irq_lock: mutex,
    pub irq_base: c_int,
    pub core_irq: c_int,
    pub tsc_irq: c_int,
    pub wakeup_flag: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_backlight_pdata {
    pub /: *mut *mut int lxw_scl; / 0/1 -- 0.8Ohm/0.4Ohm,
    pub /: *mut *mut int lxw_freq; / 700KHz ~ 1400KHz,
    pub /: *mut *mut int dual_string; / 0/1 -- single/dual string,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_touch_pdata {
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_power_pdata {
    pub (*set_charger)(int): *mut c_int,
    pub batt_detect:1: unsigned,
    pub topoff_threshold:2: unsigned,
    pub /: *mut *mut unsigned fast_charge:3; / charge current,
    pub /: *mut *mut unsigned no_temp_support:1; / set if no temperature detect,
    pub /: *mut *mut unsigned no_insert_detect:1; / set if no ac insert detect,
    pub supplied_to: *mut c_char,
    pub num_supplicants: c_int,
}

//
// irq_base: stores IRQ base number of MAX8925 in platform
// tsc_irq: stores IRQ number of MAX8925 TSC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_platform_data {
    pub backlight: *mut max8925_backlight_pdata,
    pub touch: *mut max8925_touch_pdata,
    pub power: *mut max8925_power_pdata,
    pub sd1: *mut regulator_init_data,
    pub sd2: *mut regulator_init_data,
    pub sd3: *mut regulator_init_data,
    pub ldo1: *mut regulator_init_data,
    pub ldo2: *mut regulator_init_data,
    pub ldo3: *mut regulator_init_data,
    pub ldo4: *mut regulator_init_data,
    pub ldo5: *mut regulator_init_data,
    pub ldo6: *mut regulator_init_data,
    pub ldo7: *mut regulator_init_data,
    pub ldo8: *mut regulator_init_data,
    pub ldo9: *mut regulator_init_data,
    pub ldo10: *mut regulator_init_data,
    pub ldo11: *mut regulator_init_data,
    pub ldo12: *mut regulator_init_data,
    pub ldo13: *mut regulator_init_data,
    pub ldo14: *mut regulator_init_data,
    pub ldo15: *mut regulator_init_data,
    pub ldo16: *mut regulator_init_data,
    pub ldo17: *mut regulator_init_data,
    pub ldo18: *mut regulator_init_data,
    pub ldo19: *mut regulator_init_data,
    pub ldo20: *mut regulator_init_data,
    pub irq_base: c_int,
    pub tsc_irq: c_int,
}

extern "C" {
    pub fn max8925_reg_read(: *mut i2c_client, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn max8925_reg_write(: *mut i2c_client, _arg: c_int, char: unsigned) -> c_int;
}
extern "C" {
    pub fn max8925_bulk_read(: *mut i2c_client, _arg: c_int, _arg: c_int, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn max8925_bulk_write(: *mut i2c_client, _arg: c_int, _arg: c_int, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn max8925_device_exit(: *mut max8925_chip);
}
