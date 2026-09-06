//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77693-common.h
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
// Common data shared between Maxim 77693, 77705 and 77843 drivers
//
// Copyright (C) 2015 Samsung Electronics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_types {
    TYPE_MAX77693_UNKNOWN,
    TYPE_MAX77693,
    TYPE_MAX77705,
    TYPE_MAX77843,

    TYPE_MAX77693_NUM,
}

//
// Shared also with max77843.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77693_dev {
    pub dev: *mut device,
    pub /: *mut *mut *mut i2c_client i2c; / 0xCC , PMIC, Charger, Flash LED,
    pub /: *mut *mut *mut i2c_client i2c_muic; / 0x4A , MUIC,
    pub /: *mut *mut *mut i2c_client i2c_haptic; / MAX77693: 0x90 , Haptic,
    pub /: *mut *mut *mut i2c_client i2c_chg; / MAX77843: 0xD2, Charger,
    pub type: max77693_types,
    pub regmap: *mut regmap,
    pub regmap_muic: *mut regmap,
    pub /: *mut *mut *mut regmap regmap_haptic; / Only MAX77693,
    pub /: *mut *mut *mut regmap regmap_chg; / Only MAX77843,
    pub /: *mut *mut *mut regmap regmap_leds; / Only MAX77705,
    pub irq_data_led: *mut regmap_irq_chip_data,
    pub irq_data_topsys: *mut regmap_irq_chip_data,
    pub /: *mut *mut *mut regmap_irq_chip_data irq_data_chg; / Only MAX77693,
    pub irq_data_muic: *mut regmap_irq_chip_data,
    pub irq: c_int,
}
