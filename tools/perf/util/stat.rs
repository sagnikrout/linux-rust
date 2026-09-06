//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/stat.h
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
#[derive(Copy, Clone)]
pub struct stats {
    pub M2: double n, mean,,
    pub min: u64 max,,
}

// hold aggregated event info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_stat_aggr {
// aggregated values
    pub counts: perf_counts_values,
// number of entries (CPUs) aggregated
    pub nr: c_int,
// whether any entry has failed to read/process event
    pub failed: bool,
// to mark this data is processed already
    pub used: bool,
}

// per-evsel event stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_stat_evsel {
// used for repeated runs
    pub res_stats: stats,
// number of allocated 'aggr'
    pub nr_aggr: c_int,
// aggregated event values
    pub aggr: *mut perf_stat_aggr,
// used for group read
    pub group_data: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aggr_mode {
    AGGR_NONE,
    AGGR_GLOBAL,
    AGGR_SOCKET,
    AGGR_DIE,
    AGGR_CLUSTER,
    AGGR_CACHE,
    AGGR_CORE,
    AGGR_THREAD,
    AGGR_UNSET,
    AGGR_NODE,
    AGGR_MAX
}

extern "C" {
    pub fn aggr_cpu_id(config: *mut *mut aggr_get_id_t)(struct perf_stat_config, cpu: perf_cpu) -> typedef struct;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_stat_config {
    pub aggr_mode: aggr_mode,
    pub aggr_level: u32,
    pub scale: bool,
    pub no_inherit: bool,
    pub identifier: bool,
    pub csv_output: bool,
    pub json_output: bool,
    pub interval_clear: bool,
    pub metric_only: bool,
    pub null_run: bool,
    pub hide_zero: bool,
    pub ru_display: bool,
    pub big_num: bool,
    pub hybrid_merge: bool,
    pub walltime_run_table: bool,
    pub all_kernel: bool,
    pub all_user: bool,
    pub percore_show_thread: bool,
    pub summary: bool,
    pub no_csv_summary: bool,
    pub metric_no_group: bool,
    pub metric_no_merge: bool,
    pub metric_no_threshold: bool,
    pub hardware_aware_grouping: bool,
    pub stop_read_counter: bool,
    pub iostat_run: bool,
    pub user_requested_cpu_list: *mut c_char,
    pub system_wide: bool,
    pub output: *mut FILE,
    pub interval: c_uint,
    pub timeout: c_uint,
    pub unit_width: c_uint,
    pub metric_only_len: c_uint,
    pub times: c_int,
    pub run_count: c_int,
    pub print_free_counters_hint: c_int,
    pub csv_sep: *const c_char,
    pub walltime_nsecs_stats: *mut stats,
    pub ru_data: rusage,
    pub aggr_map: *mut cpu_aggr_map,
    pub aggr_get_id: aggr_get_id_t,
    pub cpus_aggr_map: *mut cpu_aggr_map,
    pub walltime_run: *mut u64,
    pub ctl_fd: c_int,
    pub ctl_fd_ack: c_int,
    pub ctl_fd_close: bool,
    pub cgroup_list: *const c_char,
    pub topdown_level: c_uint,
}

extern "C" {
    pub fn perf_stat__set_big_num(set: c_int);
}
extern "C" {
    pub fn update_stats(stats: *mut stats, val: u64);
}
extern "C" {
    pub fn avg_stats(stats: *mut stats) -> double;
}
extern "C" {
    pub fn stddev_stats(stats: *mut stats) -> double;
}
extern "C" {
    pub fn rel_stddev_stats(stddev: double, avg: double) -> double;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metric_threshold_classify {
    METRIC_THRESHOLD_UNKNOWN,
    METRIC_THRESHOLD_BAD,
    METRIC_THRESHOLD_NEARLY_BAD,
    METRIC_THRESHOLD_LESS_GOOD,
    METRIC_THRESHOLD_GOOD,
}

extern "C" {
    pub fn void(config: *mut *mut new_line_t)(struct perf_stat_config, ctx: *mut c_void) -> typedef;
}
// Used to print the display name of the Default metricgroup for now.
extern "C" {
    pub fn perf_stat__reset_shadow_stats();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_stat_output_ctx {
    pub ctx: *mut c_void,
    pub print_metric: print_metric_t,
    pub new_line: new_line_t,
    pub print_metricgroup_header: print_metricgroup_header_t,
    pub force_header: bool,
}

extern "C" {
    pub fn perf_stat__skip_metric_event(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evlist__free_stats(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__reset_stats(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__reset_prev_raw_counts(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__copy_prev_raw_counts(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__save_aggr_prev_raw_counts(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__alloc_aggr_stats(evlist: *mut evlist, nr_aggr: c_int) -> c_int;
}
extern "C" {
    pub fn evlist__reset_aggr_stats(evlist: *mut evlist);
}
extern "C" {
    pub fn evlist__copy_res_stats(config: *mut perf_stat_config, evlist: *mut evlist);
}
extern "C" {
    pub fn perf_stat_merge_counters(config: *mut perf_stat_config, evlist: *mut evlist);
}
extern "C" {
    pub fn perf_stat_process_percore(config: *mut perf_stat_config, evlist: *mut evlist);
}
extern "C" {
    pub fn perf_event__fprintf_stat(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_stat_round(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn perf_event__fprintf_stat_config(event: *mut perf_event, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn test_generic_metric(mexp: *mut metric_expr, aggr_idx: c_int) -> double;
}
