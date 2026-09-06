//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/metricgroup.h
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
pub const METRICGROUP_H: c_int = 1;

//
// A node in a rblist keyed by the evsel. The global rblist of metric events
// generally exists in evlist. The evsel is looked up in the rblist
// yielding a list of metric_expr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metric_event {
    pub nd: rb_node,
    pub evsel: *mut evsel,
    pub /: *mut *mut bool is_default; / the metric evsel from the Default metricgroup,
    pub /: *mut *mut list_head head; / list of metric_expr,
}

//
// A metric referenced by a metric_expr. When parsing a metric expression IDs
// will be looked up, matching either a value (from metric_events) or a
// metric_ref. A metric_ref will then be parsed recursively. The metric_refs and
// metric_events need to be known before parsing so that their values may be
// placed in the parse context for lookup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metric_ref {
    pub metric_name: *const c_char,
    pub metric_expr: *const c_char,
}

//
// One in a list of metric_expr associated with an evsel. The data is used to
// generate a metric value during stat output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metric_expr {
    pub nd: list_head,
// The expression to parse, for example, "instructions/cycles".
    pub metric_expr: *const c_char,
// The name of the meric such as "IPC".
    pub metric_name: *const c_char,
    pub metric_threshold: *const c_char,
//
// The "ScaleUnit" that scales and adds a unit to the metric during
// output. For example, "6.4e-05MiB" means to scale the resulting metric
// by 6.4e-05 (typically converting a unit like cache lines to something
// more human intelligible) and then add "MiB" afterward when displayed.
//
    pub metric_unit: *const c_char,
// Displayed metricgroup name of the Default metricgroup
    pub default_metricgroup_name: *const c_char,
// Null terminated array of events used by the metric.
    pub metric_events: *mut evsel,
// Null terminated array of referenced metrics.
    pub metric_refs: *mut metric_ref,
// A value substituted for '?' during parsing.
    pub runtime: c_int,
}

extern "C" {
    pub fn metricgroup__has_metric_or_groups(pmu: *const c_char, metric_or_groups: *const c_char) -> bool;
}
extern "C" {
    pub fn metricgroups__topdown_max_level() -> c_uint;
}
extern "C" {
    pub fn arch_get_runtimeparam(pm: *const pmu_metric) -> c_int;
}
extern "C" {
    pub fn metricgroup__rblist_init(metric_events: *mut rblist);
}
extern "C" {
    pub fn metricgroup__rblist_exit(metric_events: *mut rblist);
}
