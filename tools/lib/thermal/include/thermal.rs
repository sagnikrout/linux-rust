//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/thermal/include/thermal.h
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


// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

pub const THERMAL_THRESHOLD_WAY_UP: c_uint = 0x1;

pub const THERMAL_THRESHOLD_WAY_DOWN: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_sampling_ops {
    pub arg): *mut *mut int (tz_temp)(int tz_id, int temp, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_events_ops {
    pub arg): *const *const *const int (tz_create)(char name, int tz_id, void,
    pub arg): *mut *mut int (tz_delete)(int tz_id, void,
    pub arg): *mut *mut int (tz_enable)(int tz_id, void,
    pub arg): *mut *mut int (tz_disable)(int tz_id, void,
    pub arg): *mut *mut int (trip_high)(int tz_id, int trip_id, int temp, void,
    pub arg): *mut *mut int (trip_low)(int tz_id, int trip_id, int temp, void,
    pub arg): *mut *mut int (trip_add)(int tz_id, int trip_id, int type, int temp, int hyst, void,
    pub arg): *mut *mut int (trip_change)(int tz_id, int trip_id, int type, int temp, int hyst, void,
    pub arg): *mut *mut int (trip_delete)(int tz_id, int trip_id, void,
    pub arg): *const *const *const int (cdev_add)(char name, int cdev_id, int max_state, void,
    pub arg): *mut *mut int (cdev_delete)(int cdev_id, void,
    pub arg): *mut *mut int (cdev_update)(int cdev_id, int cur_state, void,
    pub arg): *const *const *const int (gov_change)(int tz_id, char gov_name, void,
    pub arg): *mut *mut int (threshold_add)(int tz_id, int temperature, int direction, void,
    pub arg): *mut *mut int (threshold_delete)(int tz_id, int temperature, int direction, void,
    pub arg): *mut *mut int (threshold_flush)(int tz_id, void,
    pub arg): *mut *mut int (threshold_up)(int tz_id, int temp, int prev_temp, void,
    pub arg): *mut *mut int (threshold_down)(int tz_id, int temp, int prev_temp, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_ops {
    pub sampling: thermal_sampling_ops,
    pub events: thermal_events_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_trip {
    pub id: c_int,
    pub type: c_int,
    pub temp: c_int,
    pub hyst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_threshold {
    pub temperature: c_int,
    pub direction: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_zone {
    pub id: c_int,
    pub temp: c_int,
    pub name: [c_char; THERMAL_NAME_LENGTH],
    pub governor: [c_char; THERMAL_NAME_LENGTH],
    pub trip: *mut thermal_trip,
    pub thresholds: *mut thermal_threshold,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_cdev {
    pub id: c_int,
    pub name: [c_char; THERMAL_NAME_LENGTH],
    pub max_state: c_int,
    pub min_state: c_int,
    pub cur_state: c_int,
}

extern "C" {
    pub fn int(: *mut *mut cb_tz_t)(struct thermal_zone, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut cb_tt_t)(struct thermal_trip, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut cb_tc_t)(struct thermal_cdev, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut cb_th_t)(struct thermal_threshold, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn for_each_thermal_zone(tz: *mut thermal_zone, cb: cb_tz_t, arg: *mut c_void) -> LIBTHERMAL_API int;
}
extern "C" {
    pub fn for_each_thermal_trip(tt: *mut thermal_trip, cb: cb_tt_t, arg: *mut c_void) -> LIBTHERMAL_API int;
}
extern "C" {
    pub fn for_each_thermal_cdev(cdev: *mut thermal_cdev, cb: cb_tc_t, arg: *mut c_void) -> LIBTHERMAL_API int;
}
extern "C" {
    pub fn for_each_thermal_threshold(th: *mut thermal_threshold, cb: cb_th_t, arg: *mut c_void) -> LIBTHERMAL_API int;
}
extern "C" {
    pub fn thermal_exit(th: *mut thermal_handler) -> LIBTHERMAL_API void;
}
//
// Netlink thermal events
//
extern "C" {
    pub fn thermal_events_exit(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_events_init(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_events_handle(th: *mut thermal_handler, arg: *mut c_void) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_events_fd(th: *mut thermal_handler) -> LIBTHERMAL_API int;
}
//
// Netlink thermal commands
//
extern "C" {
    pub fn thermal_cmd_exit(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_cmd_init(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
//
// Netlink thermal samples
//
extern "C" {
    pub fn thermal_sampling_exit(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_sampling_init(th: *mut thermal_handler) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_sampling_handle(th: *mut thermal_handler, arg: *mut c_void) -> LIBTHERMAL_API thermal_error_t;
}
extern "C" {
    pub fn thermal_sampling_fd(th: *mut thermal_handler) -> LIBTHERMAL_API int;
}

