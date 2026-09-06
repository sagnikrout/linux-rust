//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/thermal_core.h
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
// thermal_core.h
//
// Copyright (C) 2012  Intel Corp
// Author: Durgadoss R <durgadoss.r@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_attr {
    pub attr: device_attribute,
    pub name: [c_char; THERMAL_NAME_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_trip_attrs {
    pub type: thermal_attr,
    pub temp: thermal_attr,
    pub hyst: thermal_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_trip_desc {
    pub trip: thermal_trip,
    pub trip_attrs: thermal_trip_attrs,
    pub list_node: list_head,
    pub thermal_instances: list_head,
    pub threshold: c_int,
}

//
// struct thermal_governor - structure that holds thermal governor information
// @name:	name of the governor
// @bind_to_tz: callback called when binding to a thermal zone.  If it
// returns 0, the governor is bound to the thermal zone,
// otherwise it fails.
// @unbind_from_tz:	callback called when a governor is unbound from a
// thermal zone.
// @trip_crossed:	called for trip points that have just been crossed
// @manage:	called on thermal zone temperature updates
// @update_tz:	callback called when thermal zone internals have changed, e.g.
// thermal cooling instance was added/removed
// @governor_list:	node in thermal_governor_list (in thermal_core.c)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_governor {
    pub name: *const c_char,
    pub tz): *mut *mut int (bind_to_tz)(struct thermal_zone_device,
    pub tz): *mut *mut void (unbind_from_tz)(struct thermal_zone_device,
    pub upward): bool,
    pub tz): *mut *mut void (manage)(struct thermal_zone_device,
    pub reason): thermal_notify_event,
    pub governor_list: list_head,
}

pub const TZ_STATE_READY: c_int = 0;
//
// struct thermal_zone_device - structure for a thermal zone
// @id:		unique id number for each thermal zone
// @type:	the thermal zone device type
// @device:	&struct device for this thermal zone
// @removal:	removal completion
// @resume:	resume completion
// @trips_attribute_group: trip point sysfs attributes
// @trips_high:	trips above the current zone temperature
// @trips_reached:	trips below or at the current zone temperature
// @trips_invalid:	trips with invalid temperature
// @mode:		current mode of this thermal zone
// @devdata:	private pointer for device private data
// @num_trips:	number of trip points the thermal zone supports
// @passive_delay_jiffies: number of jiffies to wait between polls when
// performing passive cooling.
// @polling_delay_jiffies: number of jiffies to wait between polls when
// checking whether trip points have been crossed (0 for
// interrupt driven systems)
// @recheck_delay_jiffies: delay after a failed attempt to determine the zone
// temperature before trying again
// @temperature:	current temperature.  This is only for core code,
// drivers should use thermal_zone_get_temp() to get the
// current temperature
// @last_temperature:	previous temperature read
// @emul_temperature:	emulated temperature when using CONFIG_THERMAL_EMULATION
// @passive:		1 if you've crossed a passive trip point, 0 otherwise.
// @prev_low_trip:	the low current temperature if you've crossed a passive
// trip point.
// @prev_high_trip:	the above current temperature if you've crossed a
// passive trip point.
// @ops:	operations this &thermal_zone_device supports
// @tzp:	thermal zone parameters
// @governor:	pointer to the governor for this thermal zone
// @governor_data:	private pointer for governor data
// @ida:	&struct ida to generate unique id for this zone's cooling
// devices
// @lock:	lock to protect thermal_instances list
// @node:	node in thermal_tz_list (in thermal_core.c)
// @poll_queue:	delayed work for polling
// @notify_event: Last notification event
// @state: 	current state of the thermal zone
// @debugfs:	this thermal zone device's thermal zone debug info
// @user_thresholds: list of userspace thresholds for temp. limit notifications
// @trips:	array of struct thermal_trip objects
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_zone_device {
    pub id: c_int,
    pub type: [c_char; THERMAL_NAME_LENGTH],
    pub device: device,
    pub removal: completion,
    pub resume: completion,
    pub trips_attribute_group: attribute_group,
    pub trips_high: list_head,
    pub trips_reached: list_head,
    pub trips_invalid: list_head,
    pub mode: thermal_device_mode,
    pub devdata: *mut c_void,
    pub num_trips: c_int,
    pub passive_delay_jiffies: c_ulong,
    pub polling_delay_jiffies: c_ulong,
    pub recheck_delay_jiffies: c_ulong,
    pub temperature: c_int,
    pub last_temperature: c_int,
    pub emul_temperature: c_int,
    pub passive: c_int,
    pub prev_low_trip: c_int,
    pub prev_high_trip: c_int,
    pub ops: thermal_zone_device_ops,
    pub tzp: *mut thermal_zone_params,
    pub governor: *mut thermal_governor,
    pub governor_data: *mut c_void,
    pub ida: ida,
    pub lock: mutex,
    pub node: list_head,
    pub poll_queue: delayed_work,
    pub notify_event: thermal_notify_event,
    pub state: u8,

    pub debugfs: *mut thermal_debugfs,

    pub user_thresholds: list_head,
    pub __counted_by(num_trips): thermal_trip_desc trips[],
}

// Initial thermal zone temperature.

//
// Default and maximum delay after a failed thermal zone temperature check
// before attempting to check it again (in jiffies).
//

// Default Thermal Governor

// Initial state of a cooling device during binding

// Init section thermal table

extern "C" {
    pub fn thermal_cdev_update(: *mut thermal_cooling_device);
}
extern "C" {
    pub fn thermal_cdev_update_nocheck(cdev: *mut thermal_cooling_device);
}
extern "C" {
    pub fn __thermal_cdev_update(cdev: *mut thermal_cooling_device);
}
extern "C" {
    pub fn get_tz_trend(tz: *mut thermal_zone_device, trip: *const thermal_trip) -> c_int;
}
//
// This structure is used to describe the behavior of
// a certain cooling device on a certain trip point
// in a certain thermal zone
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_instance {
    pub id: c_int,
    pub name: [c_char; THERMAL_NAME_LENGTH],
    pub cdev: *mut thermal_cooling_device,
    pub trip: *const thermal_trip,
    pub initialized: bool,
    pub /: *mut *mut unsigned long upper; / Highest cooling state for this trip point,
    pub /: *mut *mut unsigned long lower; / Lowest cooling state for this trip point,
    pub /: *mut *mut unsigned long target; / expected cooling state,
    pub attr_name: [c_char; THERMAL_NAME_LENGTH],
    pub attr: device_attribute,
    pub weight_attr_name: [c_char; THERMAL_NAME_LENGTH],
    pub weight_attr: device_attribute,
    pub /: *mut *mut list_head trip_node; / node in trip->thermal_instances,
    pub /: *mut *mut list_head cdev_node; / node in cdev->thermal_instances,
    pub /: *mut *mut unsigned int weight; / The weight of the cooling device,
    pub upper_no_limit: bool,
}

extern "C" {
    pub fn thermal_zone_device_set_policy(: *mut thermal_zone_device, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn thermal_build_list_of_policies(buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn thermal_zone_device_critical_reboot(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_zone_device_critical_shutdown(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_cooling_device_add(cdev: *mut thermal_cooling_device, devdata: *mut c_void) -> c_int;
}
// Helpers

extern "C" {
    pub fn thermal_zone_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int);
}
extern "C" {
    pub fn __thermal_zone_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int;
}
// sysfs I/F
extern "C" {
    pub fn thermal_zone_create_device_groups(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_zone_destroy_device_groups(: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_cooling_device_setup_sysfs(: *mut thermal_cooling_device);
}
extern "C" {
    pub fn thermal_cooling_device_destroy_sysfs(cdev: *mut thermal_cooling_device);
}
extern "C" {
    pub fn thermal_cooling_device_stats_reinit(cdev: *mut thermal_cooling_device);
}
// used only at binding time
extern "C" {
    pub fn trip_point_show(: *mut device, : *mut device_attribute, : *mut c_char) -> isize;
}
extern "C" {
    pub fn weight_show(: *mut device, : *mut device_attribute, : *mut c_char) -> isize;
}

