//! Automatically rewritten from C Header to Rust Module
//! Source: tools/thermal/tmon/tmon.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// tmon.h contains data structures and constants used by TMON
//
// Copyright (C) 2012 Intel Corporation. All rights reserved.
//
// Author Name Jacob Pan <jacob.jun.pan@linux.intel.com>
//
pub const MAX_DISP_TEMP: c_int = 125;
pub const MAX_CTRL_TEMP: c_int = 105;
pub const MIN_CTRL_TEMP: c_int = 40;
pub const MAX_NR_TZONE: c_int = 16;
pub const MAX_NR_CDEV: c_int = 32;
pub const MAX_NR_TRIP: c_int = 16;

// to a thermal zone trip.
//
pub const MAX_TEMP_KC: c_int = 140000;
// starting char position to draw sensor data, such as tz names
// trip point list, etc.
//
pub const DATA_LEFT_ALIGN: c_int = 10;
pub const NR_LINES_TZDATA: c_int = 1;

// use fixed size record to simplify data processing and transfer
// TBD: more info to be added, e.g. programmable trip point data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_data_record {
    pub tv: timeval,
    pub temp: [c_ulong; MAX_NR_TZONE],
    pub pid_out_pct: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdev_info {
    pub type: [c_char; 64],
    pub instance: c_int,
    pub max_state: c_ulong,
    pub cur_state: c_ulong,
    pub flag: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trip_type {
    THERMAL_TRIP_CRITICAL,
    THERMAL_TRIP_HOT,
    THERMAL_TRIP_PASSIVE,
    THERMAL_TRIP_ACTIVE,
    NR_THERMAL_TRIP_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trip_point {
    pub type: trip_type,
    pub temp: c_ulong,
    pub hysteresis: c_ulong,
    pub /: *mut *mut int attribute; / programmability etc.,
}

// thermal zone configuration information, binding with cooling devices could
// change at runtime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tz_info {
    pub /: *mut *mut char type[256]; / e.g. acpitz,
    pub instance: c_int,
    pub /: *mut *mut int passive; / active zone has passive node to force passive mode,
    pub /: *mut *mut int nr_cdev; / number of cooling device binded,
    pub nr_trip_pts: c_int,
    pub tp: [trip_point; MAX_NR_TRIP],
    pub /: *mut *mut unsigned long cdev_binding; / bitmap for attached cdevs,
// cdev bind trip points, allow one cdev bind to multiple trips
    pub trip_binding: [c_ulong; MAX_NR_CDEV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmon_platform_data {
    pub nr_tz_sensor: c_int,
    pub nr_cooling_dev: c_int,
// keep track of instance ids since there might be gaps
    pub max_tz_instance: c_int,
    pub max_cdev_instance: c_int,
    pub tzi: *mut tz_info,
    pub cdi: *mut cdev_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_ops {
    pub ratio): *mut *mut void (set_ratio)(unsigned long,
    pub ratio): *mut *mut unsigned long (get_ratio)(unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdev_types {
    CDEV_TYPE_PROC,
    CDEV_TYPE_FAN,
    CDEV_TYPE_MEM,
    CDEV_TYPE_NR,
}

// REVISIT: the idea is to group sensors if possible, e.g. on intel mid
// we have "skin0", "skin1", "sys", "msicdie"
// on DPTF enabled systems, we might have PCH, TSKN, TAMB, etc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tzone_types {
    TZONE_TYPE_ACPI,
    TZONE_TYPE_PCH,
    TZONE_TYPE_NR,
}

// limit the output of PID controller adjustment

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pid_params {
    pub /: *mut *mut double kp; / Controller gain from Dialog Box,
    pub /: *mut *mut double ki; / Time-constant for I action from Dialog Box,
    pub /: *mut *mut double kd; / Time-constant for D action from Dialog Box,
    pub ts: double,
    pub k_lpf: double,
    pub t_target: double,
    pub y_k: double,
}

extern "C" {
    pub fn init_thermal_controller() -> c_int;
}
extern "C" {
    pub fn controller_handler(xk: double, yk: *mut double);
}
extern "C" {
    pub fn initialize_curses();
}
extern "C" {
    pub fn show_controller_stats(line: *mut c_char);
}
extern "C" {
    pub fn show_title_bar();
}
extern "C" {
    pub fn setup_windows();
}
extern "C" {
    pub fn disable_tui();
}
extern "C" {
    pub fn show_sensors_w();
}
extern "C" {
    pub fn show_data_w();
}
extern "C" {
    pub fn write_status_bar(x: c_int, line: *mut c_char);
}
extern "C" {
    pub fn show_control_w();
}
extern "C" {
    pub fn show_cooling_device();
}
extern "C" {
    pub fn show_dialogue();
}
extern "C" {
    pub fn update_thermal_data() -> c_int;
}
extern "C" {
    pub fn probe_thermal_sysfs() -> c_int;
}
extern "C" {
    pub fn free_thermal_data();
}
extern "C" {
    pub fn resize_handler(sig: c_int);
}
extern "C" {
    pub fn set_ctrl_state(state: c_ulong);
}
extern "C" {
    pub fn get_ctrl_state(state: *mut c_ulong);
}
extern "C" {
    pub fn sysfs_set_ulong(path: *mut c_char, filename: *mut c_char, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn zone_instance_to_index(zone_inst: c_int) -> c_int;
}
extern "C" {
    pub fn close_windows();
}
pub const PT_COLOR_DEFAULT: c_int = 1;
pub const PT_COLOR_HEADER_BAR: c_int = 2;
pub const PT_COLOR_ERROR: c_int = 3;
pub const PT_COLOR_RED: c_int = 4;
pub const PT_COLOR_YELLOW: c_int = 5;
pub const PT_COLOR_GREEN: c_int = 6;
pub const PT_COLOR_BRIGHT: c_int = 7;
pub const PT_COLOR_BLUE: c_int = 8;
// each thermal zone uses 12 chars, 8 for name, 2 for instance, 2 space
// also used to list trip points in forms of AAAC, which represents
// A: Active
// C: Critical
//
pub const TZONE_RECORD_SIZE: c_int = 12;
pub const TZ_LEFT_ALIGN: c_int = 32;
pub const CDEV_NAME_SIZE: c_int = 20;

// dialogue box starts
pub const DIAG_X: c_int = 48;
pub const DIAG_Y: c_int = 8;

pub const TDATA_LEFT: c_int = 16;
