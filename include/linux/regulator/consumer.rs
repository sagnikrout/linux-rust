//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/consumer.h
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
// consumer.h -- SoC Regulator consumer support.
//
// Copyright (C) 2007, 2008 Wolfson Microelectronics PLC.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//
// Regulator Consumer Interface.
//
// A Power Management Regulator framework for SoC based devices.
// Features:-
// o Voltage and current level control.
// o Operating mode control.
// o Regulator status.
// o sysfs entries for showing client devices and status
//
// EXPERIMENTAL FEATURES:
// Dynamic Regulator operating Mode Switching (DRMS) - allows regulators
// to use most efficient operating mode depending upon voltage and load and
// is transparent to client drivers.
//
// e.g. Devices x,y,z share regulator r. Device x and y draw 20mA each during
// IO and 1mA at idle. Device z draws 100mA when under load and 5mA when
// idling. Regulator r has > 90% efficiency in NORMAL mode at loads > 100mA
// but this drops rapidly to 60% when below 100mA. Regulator r has > 90%
// efficiency in IDLE mode at loads < 10mA. Thus regulator r will operate
// in normal mode for loads > 10mA and in IDLE mode for load <= 10mA.
//

//
// Regulator operating modes.
//
// Regulators can run in a variety of different operating modes depending on
// output load. This allows further system power savings by selecting the
// best (and most efficient) regulator mode for a desired load.
//
// Most drivers will only care about NORMAL. The modes below are generic and
// will probably not match the naming convention of your regulator data sheet
// but should match the use cases in the datasheet.
//
// In order of power efficiency (least efficient at top).
//
// Mode       Description
// FAST       Regulator can handle fast changes in it's load.
// e.g. useful in CPU voltage & frequency scaling where
// load can quickly increase with CPU frequency increases.
//
// NORMAL     Normal regulator power supply mode. Most drivers will
// use this mode.
//
// IDLE       Regulator runs in a more efficient mode for light
// loads. Can be used for devices that have a low power
// requirement during periods of inactivity. This mode
// may be more noisy than NORMAL and may not be able
// to handle fast load switching.
//
// STANDBY    Regulator runs in the most efficient mode for very
// light loads. Can be used by devices when they are
// in a sleep/standby state. This mode is likely to be
// the most noisy and may not be able to handle fast load
// switching.
//
// NOTE: Most regulators will only support a subset of these modes. Some
// will only just support NORMAL.
//
// These modes can be OR'ed together to make up a mask of valid register modes.
//
pub const REGULATOR_MODE_INVALID: c_uint = 0x0;
pub const REGULATOR_MODE_FAST: c_uint = 0x1;
pub const REGULATOR_MODE_NORMAL: c_uint = 0x2;
pub const REGULATOR_MODE_IDLE: c_uint = 0x4;
pub const REGULATOR_MODE_STANDBY: c_uint = 0x8;
//
// Regulator errors that can be queried using regulator_get_error_flags
//
// UNDER_VOLTAGE  Regulator output is under voltage.
// OVER_CURRENT   Regulator output current is too high.
// REGULATION_OUT Regulator output is out of regulation.
// FAIL           Regulator output has failed.
// OVER_TEMP      Regulator over temp.
//
// NOTE: These errors can be OR'ed together.
//

//
// struct pre_voltage_change_data - Data sent with PRE_VOLTAGE_CHANGE event
//
// @old_uV: Current voltage before change.
// @min_uV: Min voltage we'll change to.
// @max_uV: Max voltage we'll change to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pre_voltage_change_data {
    pub old_uV: c_ulong,
    pub min_uV: c_ulong,
    pub max_uV: c_ulong,
}

//
// struct regulator_bulk_data - Data used for bulk regulator operations.
//
// @supply:       The name of the supply.  Initialised by the user before
// using the bulk regulator APIs.
// @consumer:     The regulator consumer for the supply.  This will be managed
// by the bulk API.
// @init_load_uA: After getting the regulator, regulator_set_load() will be
// called with this load.  Initialised by the user before
// using the bulk regulator APIs.
//
// The regulator APIs provide a series of regulator_bulk_() API calls as
// a convenience to consumers which require multiple supplies.  This
// structure is used to manage data for these calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut regulator,
    pub init_load_uA: c_int,
// private: Internal use
    pub ret: c_int,
}

// regulator get and put
extern "C" {
    pub fn devm_regulator_get_enable(dev: *mut device, id: *const c_char) -> c_int;
}
extern "C" {
    pub fn devm_regulator_get_enable_optional(dev: *mut device, id: *const c_char) -> c_int;
}
extern "C" {
    pub fn devm_regulator_get_enable_read_voltage(dev: *mut device, id: *const c_char) -> c_int;
}
extern "C" {
    pub fn regulator_put(regulator: *mut regulator);
}
extern "C" {
    pub fn devm_regulator_put(regulator: *mut regulator);
}
extern "C" {
    pub fn regulator_unregister_supply_alias(dev: *mut device, id: *const c_char);
}
// regulator output control and status
extern "C" {
    pub fn regulator_enable(regulator: *mut regulator) -> int __must_check;
}
extern "C" {
    pub fn regulator_disable(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_force_disable(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_is_enabled(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_disable_deferred(regulator: *mut regulator, ms: c_int) -> c_int;
}
extern "C" {
    pub fn devm_regulator_bulk_put(consumers: *mut regulator_bulk_data);
}
extern "C" {
    pub fn regulator_count_voltages(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_list_voltage(regulator: *mut regulator, selector: unsigned) -> c_int;
}
extern "C" {
    pub fn regulator_get_linear_step(regulator: *mut regulator) -> c_uint;
}
extern "C" {
    pub fn regulator_set_voltage(regulator: *mut regulator, min_uV: c_int, max_uV: c_int) -> c_int;
}
extern "C" {
    pub fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_sync_voltage(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_get_current_limit(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_get_unclaimed_power_budget(regulator: *mut regulator) -> c_int;
}
extern "C" {
    pub fn regulator_set_mode(regulator: *mut regulator, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn regulator_get_mode(regulator: *mut regulator) -> c_uint;
}
extern "C" {
    pub fn regulator_set_load(regulator: *mut regulator, load_uA: c_int) -> c_int;
}
extern "C" {
    pub fn regulator_allow_bypass(regulator: *mut regulator, allow: bool) -> c_int;
}
extern "C" {
    pub fn regulator_hardware_enable(regulator: *mut regulator, enable: bool) -> c_int;
}
// regulator notifier block
// regulator suspend
// driver data - core doesn't touch
extern "C" {
    pub fn regulator_set_drvdata(regulator: *mut regulator, data: *mut c_void);
}
// misc helpers
extern "C" {
    pub fn regulator_is_equal(reg1: *mut regulator, reg2: *mut regulator) -> bool;
}

//
// Make sure client drivers will still build on systems with no software
// controllable voltage or current regulators.
//
// Nothing except the stubbed out regulator API should be
// looking at the value except to check if it is an error
// value. Drivers are free to handle NULL specifically by
// skipping all regulator API calls, but they don't have to.
// Drivers which don't, should make sure they properly handle
// corner cases of the API, such as regulator_get_voltage()
// returning 0.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn regulator_set_voltage(_arg: regulator, _arg: min_uV, _arg: max_uV) -> return;
}
