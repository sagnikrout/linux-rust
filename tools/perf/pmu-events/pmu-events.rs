//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/pmu-events/pmu-events.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aggr_mode_class {
    PerChip = 1,
    PerCore
}

//
// enum metric_event_groups - How events within a pmu_metric should be grouped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metric_event_groups {
//
// @MetricGroupEvents: Default, group events within the metric.
//
    MetricGroupEvents = 0,
//
// @MetricNoGroupEvents: Don't group events for the metric.
//
    MetricNoGroupEvents = 1,
//
// @MetricNoGroupEventsNmi:
// Don't group events for the metric if the NMI watchdog is enabled.
//
    MetricNoGroupEventsNmi = 2,
//
// @MetricNoGroupEventsSmt:
// Don't group events for the metric if SMT is enabled.
//
    MetricNoGroupEventsSmt = 3,
//
// @MetricNoGroupEventsThresholdAndNmi:
// Don't group events for the metric thresholds and if the NMI watchdog
// is enabled.
//
    MetricNoGroupEventsThresholdAndNmi = 4,
}

//
// Describe each PMU event. Each CPU has a table of PMU events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_event {
    pub name: *const c_char,
    pub compat: *const c_char,
    pub event: *const c_char,
    pub desc: *const c_char,
    pub topic: *const c_char,
    pub long_desc: *const c_char,
    pub pmu: *const c_char,
    pub unit: *const c_char,
    pub retirement_latency_mean: *const c_char,
    pub retirement_latency_min: *const c_char,
    pub retirement_latency_max: *const c_char,
    pub perpkg: bool,
    pub deprecated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_metric {
    pub pmu: *const c_char,
    pub metric_name: *const c_char,
    pub metric_group: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub unit: *const c_char,
    pub compat: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub metricgroup_no_group: *const c_char,
    pub default_metricgroup_name: *const c_char,
    pub aggr_mode: aggr_mode_class,
    pub event_grouping: metric_event_groups,
    pub default_show_events: bool,
}

//
// Search for a table and entry matching with pmu__name_wildcard_match or any
// tables if pmu is NULL. Each matching event has fn called on it. 0 implies to
// success/continue the search while non-zero means to terminate. The special
// value PMU_EVENTS__NOT_FOUND is used to indicate no event was found in one of
// the tables which doesn't terminate the search of all tables.
//
extern "C" {
    pub fn pmu_metrics_table__iterate_tables(fn: pmu_metrics_table_iter_t, data: *mut c_void) -> c_int;
}
//
// Search for a table and entry matching with pmu__name_wildcard_match or any
// tables if pmu is NULL. Each matching metric has fn called on it. 0 implies to
// success/continue the search while non-zero means to terminate. The special
// value PMU_METRICS__NOT_FOUND is used to indicate no metric was found in one
// of the tables which doesn't terminate the search of all tables.
//
extern "C" {
    pub fn pmu_for_each_core_event(fn: pmu_event_iter_fn, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pmu_for_each_core_metric(fn: pmu_metric_iter_fn, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pmu_for_each_sys_event(fn: pmu_event_iter_fn, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pmu_for_each_sys_metric(fn: pmu_metric_iter_fn, data: *mut c_void) -> c_int;
}
