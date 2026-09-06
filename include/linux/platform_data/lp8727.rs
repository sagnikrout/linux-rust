//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lp8727.h
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
// LP8727 Micro/Mini USB IC with integrated charger
//
// Copyright (C) 2011 Texas Instruments
// Copyright (C) 2011 National Semiconductor
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8727_eoc_level {
    LP8727_EOC_5P,
    LP8727_EOC_10P,
    LP8727_EOC_16P,
    LP8727_EOC_20P,
    LP8727_EOC_25P,
    LP8727_EOC_33P,
    LP8727_EOC_50P,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8727_ichg {
    LP8727_ICHG_90mA,
    LP8727_ICHG_100mA,
    LP8727_ICHG_400mA,
    LP8727_ICHG_450mA,
    LP8727_ICHG_500mA,
    LP8727_ICHG_600mA,
    LP8727_ICHG_700mA,
    LP8727_ICHG_800mA,
    LP8727_ICHG_900mA,
    LP8727_ICHG_1000mA,
}

//
// struct lp8727_chg_param
// @eoc_level : end of charge level setting
// @ichg      : charging current
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8727_chg_param {
    pub eoc_level: lp8727_eoc_level,
    pub ichg: lp8727_ichg,
}

//
// struct lp8727_platform_data
// @get_batt_present  : check battery status - exists or not
// @get_batt_level    : get battery voltage (mV)
// @get_batt_capacity : get battery capacity (%)
// @get_batt_temp     : get battery temperature
// @ac                : charging parameters for AC type charger
// @usb               : charging parameters for USB type charger
// @debounce_msec     : interrupt debounce time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8727_platform_data {
    pub (*get_batt_present)(void): *mut u8,
    pub (*get_batt_level)(void): *mut u16,
    pub (*get_batt_capacity)(void): *mut u8,
    pub (*get_batt_temp)(void): *mut u8,
    pub ac: *mut lp8727_chg_param,
    pub usb: *mut lp8727_chg_param,
    pub debounce_msec: c_uint,
}
