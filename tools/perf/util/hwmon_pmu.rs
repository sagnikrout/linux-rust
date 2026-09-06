//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/hwmon_pmu.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

//
// enum hwmon_type:
//
// As described in Documentation/hwmon/sysfs-interface.rst hwmon events are
// defined over multiple files of the form <type><num>_<item>. This enum
// captures potential <type> values.
//
// This enum is exposed for testing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_type {
    HWMON_TYPE_NONE,

    HWMON_TYPE_CPU,
    HWMON_TYPE_CURR,
    HWMON_TYPE_ENERGY,
    HWMON_TYPE_FAN,
    HWMON_TYPE_HUMIDITY,
    HWMON_TYPE_IN,
    HWMON_TYPE_INTRUSION,
    HWMON_TYPE_POWER,
    HWMON_TYPE_PWM,
    HWMON_TYPE_TEMP,

    HWMON_TYPE_MAX
}

//
// enum hwmon_item:
//
// Similar to enum hwmon_type but describes the item part of a sysfs filename.
//
// This enum is exposed for testing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_item {
    HWMON_ITEM_NONE,

    HWMON_ITEM_ACCURACY,
    HWMON_ITEM_ALARM,
    HWMON_ITEM_AUTO_CHANNELS_TEMP,
    HWMON_ITEM_AVERAGE,
    HWMON_ITEM_AVERAGE_HIGHEST,
    HWMON_ITEM_AVERAGE_INTERVAL,
    HWMON_ITEM_AVERAGE_INTERVAL_MAX,
    HWMON_ITEM_AVERAGE_INTERVAL_MIN,
    HWMON_ITEM_AVERAGE_LOWEST,
    HWMON_ITEM_AVERAGE_MAX,
    HWMON_ITEM_AVERAGE_MIN,
    HWMON_ITEM_BEEP,
    HWMON_ITEM_CAP,
    HWMON_ITEM_CAP_HYST,
    HWMON_ITEM_CAP_MAX,
    HWMON_ITEM_CAP_MIN,
    HWMON_ITEM_CRIT,
    HWMON_ITEM_CRIT_HYST,
    HWMON_ITEM_DIV,
    HWMON_ITEM_EMERGENCY,
    HWMON_ITEM_EMERGENCY_HIST,
    HWMON_ITEM_ENABLE,
    HWMON_ITEM_FAULT,
    HWMON_ITEM_FREQ,
    HWMON_ITEM_HIGHEST,
    HWMON_ITEM_INPUT,
    HWMON_ITEM_LABEL,
    HWMON_ITEM_LCRIT,
    HWMON_ITEM_LCRIT_HYST,
    HWMON_ITEM_LOWEST,
    HWMON_ITEM_MAX,
    HWMON_ITEM_MAX_HYST,
    HWMON_ITEM_MIN,
    HWMON_ITEM_MIN_HYST,
    HWMON_ITEM_MOD,
    HWMON_ITEM_OFFSET,
    HWMON_ITEM_PULSES,
    HWMON_ITEM_RATED_MAX,
    HWMON_ITEM_RATED_MIN,
    HWMON_ITEM_RESET_HISTORY,
    HWMON_ITEM_TARGET,
    HWMON_ITEM_TYPE,
    HWMON_ITEM_VID,

    HWMON_ITEM__MAX,
}

//
// union hwmon_pmu_event_key: Key for hwmon_pmu->events as such each key
// represents an event.
// union is exposed for testing to ensure problems are avoided on big
// endian machines.
//
// Related hwmon files start <type><number> that this key represents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union hwmon_pmu_event_key {
    pub type_and_num: c_long,
    pub :16: int num,
    pub :8: hwmon_type type,
}

extern "C" {
    pub fn perf_pmu__is_hwmon(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn evsel__is_hwmon(evsel: *const evsel) -> bool;
}
//
// parse_hwmon_filename() - Parse filename into constituent parts.
//
// @filename: To be parsed, of the form <type><number>_<item>.
// @type: The type defined from the parsed file name.
// @number: The number of the type, for example there may be more than 1 fan.
// @item: A hwmon <type><number> may have multiple associated items.
// @alarm: Is the filename for an alarm value?
//
// An example of a hwmon filename is "temp1_input". The type is temp for a
// temperature value. The number is 1. The item within the file is an input
// value - the temperature itself. This file doesn't contain an alarm value.
//
// Exposed for testing.
//
// hwmon_pmu__new() - Allocate and construct a hwmon PMU.
//
// @pmus: The list of PMUs to be added to.
// @hwmon_dir: The path to a hwmon directory.
// @sysfs_name: Name of the hwmon sysfs directory like hwmon0.
// @name: The contents of the "name" file in the hwmon directory.
//
// Exposed for testing. Regular construction should happen via
// perf_pmus__read_hwmon_pmus.
//
extern "C" {
    pub fn hwmon_pmu__exit(pmu: *mut perf_pmu);
}
extern "C" {
    pub fn hwmon_pmu__for_each_event(pmu: *mut perf_pmu, state: *mut c_void, cb: pmu_event_callback) -> c_int;
}
extern "C" {
    pub fn hwmon_pmu__num_events(pmu: *mut perf_pmu) -> usize;
}
extern "C" {
    pub fn hwmon_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmus__read_hwmon_pmus(pmus: *mut list_head) -> c_int;
}
extern "C" {
    pub fn evsel__hwmon_pmu_read(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
}
