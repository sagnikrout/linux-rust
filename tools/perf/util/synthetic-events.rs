//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/synthetic-events.h
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
pub enum perf_record_synth {
    PERF_SYNTH_TASK		= 1 << 0,
    PERF_SYNTH_MMAP		= 1 << 1,
    PERF_SYNTH_CGROUP	= 1 << 2,

// last element
    PERF_SYNTH_MAX		= 1 << 3,
}

extern "C" {
    pub fn parse_synth_opt(str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_attrs(tool: *const perf_tool, evlist: *mut evlist, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_attr(tool: *const perf_tool, attr: *mut perf_event_attr, ids: u32, id: *mut u64, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_cpu_map(tool: *const perf_tool, cpus: *const perf_cpu_map, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_event_update_cpus(tool: *const perf_tool, evsel: *mut evsel, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_event_update_name(tool: *const perf_tool, evsel: *mut evsel, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_event_update_scale(tool: *const perf_tool, evsel: *mut evsel, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_event_update_unit(tool: *const perf_tool, evsel: *mut evsel, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_extra_attr(tool: *const perf_tool, evsel_list: *mut evlist, process: perf_event__handler_t, is_pipe: bool) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_extra_kmaps(tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_features(tool: *const perf_tool, session: *mut perf_session, evlist: *mut evlist, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_id_index(tool: *const perf_tool, process: perf_event__handler_t, evlist: *mut evlist, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn __perf_event__synthesize_id_index(tool: *const perf_tool, process: perf_event__handler_t, evlist: *mut evlist, machine: *mut machine, from: usize) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_id_sample(array: *mut __u64, type: u64, sample: *const perf_sample) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_kernel_mmap(tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_mmap_events(tool: *const perf_tool, event: *mut perf_event, pid: pid_t, tgid: pid_t, process: perf_event__handler_t, machine: *mut machine, mmap_data: bool) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_modules(tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_namespaces(tool: *const perf_tool, event: *mut perf_event, pid: pid_t, tgid: pid_t, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_cgroups(tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_stat_config(tool: *const perf_tool, config: *mut perf_stat_config, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_stat_events(config: *mut perf_stat_config, tool: *const perf_tool, evlist: *mut evlist, process: perf_event__handler_t, attrs: bool) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_stat_round(tool: *const perf_tool, time: u64, type: u64, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_stat(tool: *const perf_tool, cpu: perf_cpu, thread: u32, id: u64, count: *mut perf_counts_values, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_thread_map2(tool: *const perf_tool, threads: *mut perf_thread_map, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_thread_map(tool: *const perf_tool, threads: *mut perf_thread_map, process: perf_event__handler_t, machine: *mut machine, needs_mmap: bool, mmap_data: bool) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_threads(tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine, needs_mmap: bool, mmap_data: bool, nr_threads_synthesize: c_uint) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_tracing_data(tool: *const perf_tool, fd: c_int, evlist: *mut evlist, process: perf_event__handler_t) -> c_int;
}
extern "C" {
    pub fn perf_event__synth_time_conv(pc: *const perf_event_mmap_page, tool: *const perf_tool, process: perf_event__handler_t, machine: *mut machine) -> c_int;
}
extern "C" {
    pub fn perf_event__synthesize_comm(tool: *const perf_tool, event: *mut perf_event, pid: pid_t, process: perf_event__handler_t, machine: *mut machine) -> pid_t;
}
extern "C" {
    pub fn perf_tool__process_synth_event(tool: *const perf_tool, event: *mut perf_event, machine: *mut machine, process: perf_event__handler_t) -> c_int;
}

