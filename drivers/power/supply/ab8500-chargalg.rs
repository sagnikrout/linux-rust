//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/ab8500-chargalg.h
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
// Copyright (C) ST-Ericsson SA 2012
// Author: Johan Gardsmark <johan.gardsmark@stericsson.com> for ST-Ericsson.
//

//
// Valid only for supplies of type:
// - POWER_SUPPLY_TYPE_MAINS,
// - POWER_SUPPLY_TYPE_USB,
// because only them store as drv_data pointer to struct ux500_charger.
//

// Forward declaration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_charger_ops {
    pub int): *mut *mut *mut int (enable) (struct ux500_charger , int, int,,
    pub int): *mut *mut *mut int (check_enable) (struct ux500_charger , int,,
    pub ): *mut *mut int (kick_wd) (struct ux500_charger,
    pub int): *mut *mut *mut int (update_curr) (struct ux500_charger ,,
}

//
// struct ux500_charger - power supply ux500 charger sub class
// @psy			power supply base class
// @ops			ux500 charger operations
// @max_out_volt_uv	maximum output charger voltage in uV
// @max_out_curr_ua	maximum output charger current in uA
// @enabled		indicates if this charger is used or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_charger {
    pub psy: *mut power_supply,
    pub ops: ux500_charger_ops,
    pub max_out_volt_uv: c_int,
    pub max_out_curr_ua: c_int,
    pub wdt_refresh: c_int,
    pub enabled: bool,
}
