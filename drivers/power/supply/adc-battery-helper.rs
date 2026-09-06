//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/adc-battery-helper.h
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
// Helper for batteries with accurate current and voltage measurement, but
// without temperature measurement or without a "resistance-temp-table".
// Copyright (c) 2021-2025 Hans de Goede <hansg@kernel.org>
//

pub const ADC_BAT_HELPER_MOV_AVG_WINDOW_SIZE: c_int = 8;
//
// The adc battery helper code needs voltage- and current-now to be sampled as
// close to each other (in sample-time) as possible. A single getter function is
// used to allow the battery driver to handle this in the best way possible.
//
extern "C" {
    pub fn int(psy: *mut *mut adc_battery_helper_get_func)(struct power_supply, volt: *mut c_int, curr: *mut c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc_battery_helper {
    pub psy: *mut power_supply,
    pub charge_finished: *mut gpio_desc,
    pub work: delayed_work,
    pub lock: mutex,
    pub get_voltage_and_current_now: adc_battery_helper_get_func,
    pub /: *mut *mut int ocv_uv[ADC_BAT_HELPER_MOV_AVG_WINDOW_SIZE]; / micro-volt,
    pub /: *mut *mut int intern_res_mohm[ADC_BAT_HELPER_MOV_AVG_WINDOW_SIZE]; / milli-ohm,
    pub poll_count: c_int,
    pub ocv_avg_index: c_int,
    pub /: *mut *mut int ocv_avg_uv; / micro-volt,
    pub intern_res_poll_count: c_int,
    pub intern_res_avg_index: c_int,
    pub /: *mut *mut int intern_res_avg_mohm; / milli-ohm,
    pub /: *mut *mut int volt_uv; / micro-volt,
    pub /: *mut *mut int curr_ua; / micro-ampere,
    pub /: *mut *mut int capacity; / percent,
    pub status: c_int,
    pub supplied: bool,
}

// Must be const cannot be an external. Asserted in adc-battery-helper.c
pub const ADC_HELPER_NUM_PROPERTIES: c_int = 7;
//
// The below functions can be directly used as power-supply / suspend-resume
// callbacks. They cast the power_supply_get_drvdata() / dev_get_drvdata() data
// directly to struct adc_battery_helper. Therefor struct adc_battery_helper
// MUST be the first member of the battery driver's data struct.
//
extern "C" {
    pub fn adc_battery_helper_external_power_changed(psy: *mut power_supply);
}
extern "C" {
    pub fn adc_battery_helper_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn adc_battery_helper_resume(dev: *mut device) -> c_int;
}
