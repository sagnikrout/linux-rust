//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/thermal.h
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
// thermal.h  ($Revision: 0 $)
//
// Copyright (C) 2008  Intel Corp
// Copyright (C) 2008  Zhang Rui <rui.zhang@intel.com>
// Copyright (C) 2008  Sujith Thomas <sujith.thomas@intel.com>
//

// invalid cooling state

// No upper/lower limit requirement

// Default weight of a bound cooling device
pub const THERMAL_WEIGHT_DEFAULT: c_int = 0;
// use value, which < 0K, to indicate an invalid/uninitialized temperature

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_trend {
    THERMAL_TREND_STABLE, /* temperature is stable */
    THERMAL_TREND_RAISING, /* temperature is raising */
    THERMAL_TREND_DROPPING, /* temperature is dropping */
}

// Thermal notification reason
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum thermal_notify_event {
    THERMAL_EVENT_UNSPECIFIED, /* Unspecified event */
    THERMAL_EVENT_TEMP_SAMPLE, /* New Temperature sample */
    THERMAL_TRIP_VIOLATED, /* TRIP Point violation */
    THERMAL_TRIP_CHANGED, /* TRIP Point temperature changed */
    THERMAL_DEVICE_DOWN, /* Thermal device is down */
    THERMAL_DEVICE_UP, /* Thermal device is up after a down event */
    THERMAL_DEVICE_POWER_CAPABILITY_CHANGED, /* power capability changed */
    THERMAL_TABLE_CHANGED, /* Thermal table(s) changed */
    THERMAL_EVENT_KEEP_ALIVE, /* Request for user space handler to respond */
    THERMAL_TZ_BIND_CDEV, /* Cooling dev is bind to the thermal zone */
    THERMAL_TZ_UNBIND_CDEV, /* Cooling dev is unbind from the thermal zone */
    THERMAL_INSTANCE_WEIGHT_CHANGED, /* Thermal instance weight changed */
    THERMAL_TZ_RESUME, /* Thermal zone is resuming after system sleep */
    THERMAL_TZ_ADD_THRESHOLD, /* Threshold added */
    THERMAL_TZ_DEL_THRESHOLD, /* Threshold deleted */
    THERMAL_TZ_FLUSH_THRESHOLDS, /* All thresholds deleted */
}

//
// struct thermal_trip - representation of a point in temperature domain
// @temperature: temperature value in miliCelsius
// @hysteresis: relative hysteresis in miliCelsius
// @type: trip point type
// @priv: pointer to driver data associated with this trip
// @flags: flags representing binary properties of the trip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_trip {
    pub temperature: c_int,
    pub hysteresis: c_int,
    pub type: thermal_trip_type,
    pub flags: u8,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cooling_spec {
    pub /: *mut *mut unsigned long upper; / Highest cooling state,
    pub /: *mut *mut unsigned long lower; / Lowest cooling state,
    pub /: *mut *mut unsigned int weight; / Cooling device weight,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_zone_device_ops {
    pub ): *mut cooling_spec,
    pub ): *mut *mut *mut int (get_temp) (struct thermal_zone_device , int,
    pub int): *mut *mut *mut int (set_trips) (struct thermal_zone_device , int,,
    pub thermal_device_mode): enum,
    pub int): *const *const thermal_trip ,,
    pub ): *mut *mut *mut int (get_crit_temp) (struct thermal_zone_device , int,
    pub int): *mut *mut *mut int (set_emul_temp) (struct thermal_zone_device ,,
    pub ): *const *const thermal_trip , enum thermal_trend,
    pub ): *mut *mut void (hot)(struct thermal_zone_device,
    pub ): *mut *mut void (critical)(struct thermal_zone_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_cooling_device_ops {
    pub ): *mut *mut *mut int (get_max_state) (struct thermal_cooling_device , unsigned long,
    pub ): *mut *mut *mut int (get_cur_state) (struct thermal_cooling_device , unsigned long,
    pub long): *mut *mut *mut int (set_cur_state) (struct thermal_cooling_device , unsigned,
    pub ): *mut *mut *mut int (get_requested_power)(struct thermal_cooling_device , u32,
    pub ): *mut *mut *mut int (state2power)(struct thermal_cooling_device , unsigned long, u32,
    pub ): *mut *mut *mut int (power2state)(struct thermal_cooling_device , u32, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_cooling_device {
    pub id: c_int,
    pub type: *const c_char,
    pub max_state: c_ulong,
    pub device: device,
    pub devdata: *mut c_void,
    pub stats: *mut c_void,
    pub ops: *const thermal_cooling_device_ops,
    pub /: *mut *mut bool updated; / true if the cooling device does not need update,
    pub /: *mut *mut mutex lock; / protect thermal_instances list,
    pub thermal_instances: list_head,
    pub node: list_head,

    pub np: *mut device_node,
    pub cdev_id: u32,

    pub debugfs: *mut thermal_debugfs,

}

// Structure to define Thermal Zone parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_zone_params {
    pub governor_name: *const c_char,
//
// a boolean to indicate if the thermal to hwmon sysfs interface
// is required. when no_hwmon == false, a hwmon sysfs interface
// will be created. when no_hwmon == true, nothing will be done
//
    pub no_hwmon: bool,
//
// Sustainable power (heat) that this thermal zone can dissipate in
// mW
//
    pub sustainable_power: u32,
//
// Proportional parameter of the PID controller when
// overshooting (i.e., when temperature is below the target)
//
    pub k_po: i32,
//
// Proportional parameter of the PID controller when
// undershooting
//
    pub k_pu: i32,
// Integral parameter of the PID controller
    pub k_i: i32,
// Derivative parameter of the PID controller
    pub k_d: i32,
// threshold below which the error is no longer accumulated
    pub integral_cutoff: i32,
//
// @slope:	slope of a linear temperature adjustment curve.
// Used by thermal zone drivers.
//
    pub slope: c_int,
//
// @offset:	offset of a linear temperature adjustment curve.
// Used by thermal zone drivers (default 0).
//
    pub offset: c_int,
}

// Function declarations

extern "C" {
    pub fn devm_thermal_of_zone_unregister(dev: *mut device, tz: *mut thermal_zone_device);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
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
    pub fn thermal_zone_get_crit_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int;
}

extern "C" {
    pub fn thermal_zone_device_unregister(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_zone_device_id(tzd: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_cooling_device_update(: *mut thermal_cooling_device);
}
extern "C" {
    pub fn thermal_cooling_device_unregister(: *mut thermal_cooling_device);
}
extern "C" {
    pub fn thermal_zone_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn thermal_zone_get_slope(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_zone_get_offset(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_zone_device_enable(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_zone_device_disable(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_zone_device_critical(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_pm_prepare();
}
extern "C" {
    pub fn thermal_pm_complete();
}

