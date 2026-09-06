//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/tool_pmu.h
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
pub enum tool_pmu_event {
    TOOL_PMU__EVENT_NONE = 0,
    TOOL_PMU__EVENT_DURATION_TIME,
    TOOL_PMU__EVENT_USER_TIME,
    TOOL_PMU__EVENT_SYSTEM_TIME,
    TOOL_PMU__EVENT_HAS_PMEM,
    TOOL_PMU__EVENT_NUM_CORES,
    TOOL_PMU__EVENT_NUM_CPUS,
    TOOL_PMU__EVENT_NUM_CPUS_ONLINE,
    TOOL_PMU__EVENT_NUM_DIES,
    TOOL_PMU__EVENT_NUM_PACKAGES,
    TOOL_PMU__EVENT_SLOTS,
    TOOL_PMU__EVENT_SMT_ON,
    TOOL_PMU__EVENT_SYSTEM_TSC_FREQ,
    TOOL_PMU__EVENT_CORE_WIDE,
    TOOL_PMU__EVENT_TARGET_CPU,

    TOOL_PMU__EVENT_MAX,
}

extern "C" {
    pub fn tool_pmu__str_to_event(str: *const c_char) -> tool_pmu_event;
}
extern "C" {
    pub fn tool_pmu__skip_event(name: *const c_char) -> bool;
}
extern "C" {
    pub fn tool_pmu__num_skip_events() -> c_int;
}
extern "C" {
    pub fn tool_pmu__cpu_slots_per_cycle() -> u64;
}
extern "C" {
    pub fn perf_pmu__is_tool(pmu: *const perf_pmu) -> bool;
}
extern "C" {
    pub fn evsel__is_tool(evsel: *const evsel) -> bool;
}
extern "C" {
    pub fn evsel__tool_event(evsel: *const evsel) -> tool_pmu_event;
}
extern "C" {
    pub fn evsel__tool_pmu_enable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
}
extern "C" {
    pub fn evsel__tool_pmu_enable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn evsel__tool_pmu_disable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
}
extern "C" {
    pub fn evsel__tool_pmu_disable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn evsel__tool_pmu_read(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
}
