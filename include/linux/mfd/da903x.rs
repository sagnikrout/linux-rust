//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da903x.h
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
// Unified sub device IDs for DA9030/DA9034/DA9035
//
// DA9030/DA9034 LEDs sub-devices uses generic "struct led_info"
// as the platform_data
//
// DA9030 flags for "struct led_info"
//

// DA9034 flags for "struct led_info"

// DA9034 touch screen platform data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9034_touch_pdata {
    pub /: *mut *mut int interval_ms; / sampling interval while pen down,
    pub x_inverted: c_int,
    pub y_inverted: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9034_backlight_pdata {
    pub /: *mut *mut int output_current; / output current of WLED, from 0-31 (in mA),
}

// DA9030 battery charger data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9030_battery_info {
// battery parameters
    pub battery_info: *mut power_supply_info,
// current and voltage to use for battery charging
    pub charge_milliamp: c_uint,
    pub charge_millivolt: c_uint,
// voltage thresholds (in millivolts)
    pub vbat_low: c_int,
    pub vbat_crit: c_int,
    pub vbat_charge_start: c_int,
    pub vbat_charge_stop: c_int,
    pub vbat_charge_restart: c_int,
// battery nominal minimal and maximal voltages in millivolts
    pub vcharge_min: c_int,
    pub vcharge_max: c_int,
// Temperature thresholds. These are DA9030 register values
    pub tbat_low: c_int,
    pub tbat_high: c_int,
    pub tbat_restart: c_int,
// battery monitor interval (seconds)
    pub batmon_interval: c_uint,
// platform callbacks for battery low and critical events
    pub (*battery_low)(void): *mut c_void,
    pub (*battery_critical)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_subdev_info {
    pub id: c_int,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_platform_data {
    pub num_subdevs: c_int,
    pub subdevs: *mut da903x_subdev_info,
}

// bit definitions for DA9030 events

// bit definitions for DA9034 events

// Status Query Interface

extern "C" {
    pub fn da903x_query_status(dev: *mut device, status: c_uint) -> c_int;
}
// NOTE: the functions below are not intended for use outside
// of the DA903x sub-device drivers
//
extern "C" {
    pub fn da903x_write(dev: *mut device, reg: c_int, val: u8) -> c_int;
}
extern "C" {
    pub fn da903x_writes(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn da903x_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn da903x_reads(dev: *mut device, reg: c_int, len: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn da903x_update(dev: *mut device, reg: c_int, val: u8, mask: u8) -> c_int;
}
extern "C" {
    pub fn da903x_set_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
extern "C" {
    pub fn da903x_clr_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
