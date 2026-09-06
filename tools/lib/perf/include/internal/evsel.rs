//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/internal/evsel.h
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
// The per-thread accumulated period storage node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_id_period {
    pub node: list_head,
    pub hnode: hlist_node,
// Holds total ID period value for PERF_SAMPLE_READ processing.
    pub period: u64,
// The TID that the values belongs to
    pub tid: u32,
}

//
// perf_evsel_for_each_per_thread_period_safe - safely iterate thru all the
// per_stream_periods
// @evlist:perf_evsel instance to iterate
// @item: struct perf_sample_id_period iterator
// @tmp: struct perf_sample_id_period temp iterator
//

pub const PERF_SAMPLE_ID__HLIST_BITS: c_int = 4;

//
// Per fd, to map back from PERF_SAMPLE_ID to evsel, only used when there are
// more than one entry in the evlist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_id {
    pub node: hlist_node,
    pub id: u64,
    pub evsel: *mut perf_evsel,
//
// 'idx' will be used for AUX area sampling. A sample will have AUX area
// data that will be queued for decoding, where there are separate
// queues for each CPU (per-cpu tracing) or task (per-thread tracing).
// The sample ID can be used to lookup 'idx' which is effectively the
// queue number.
//
    pub idx: c_int,
    pub cpu: perf_cpu,
    pub tid: pid_t,
// Guest machine pid and VCPU, valid only if machine_pid is non-zero
    pub machine_pid: pid_t,
    pub vcpu: perf_cpu,
//
// Per-thread, and global event counts are mutually exclusive:
// Whilst it is possible to combine events into a group with differing
// values of PERF_SAMPLE_READ, it is not valid to have inconsistent
// values for `inherit`. Therefore it is not possible to have a
// situation where a per-thread event is sampled as a global event;
// all !inherit groups are global, and all groups where the sampling
// event is inherit + PERF_SAMPLE_READ will be per-thread. Any event
// that is part of such a group that is inherit but not PERF_SAMPLE_READ
// will be read as per-thread. If such an event can also trigger a
// sample (such as with sample_period > 0) then it will not cause
// `read_format` to be included in its PERF_RECORD_SAMPLE, and
// therefore will not expose the per-thread group members as global.
//
// Holds total ID period value for PERF_SAMPLE_READ processing
// (when period is not per-thread).
//
    pub period: u64,
//
// Holds total ID period value for PERF_SAMPLE_READ processing
// (when period is per-thread).
//
    pub periods: [hlist_head; PERF_SAMPLE_ID__HLIST_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_evsel {
    pub node: list_head,
    pub attr: perf_event_attr,
// The commonly used cpu map of CPUs the event should be opened upon, etc.
    pub cpus: *mut perf_cpu_map,
//
// The cpu map read from the PMU. For core PMUs this is the list of all
// CPUs the event can be opened upon. For other PMUs this is the default
// cpu map for opening the event on, for example, the first CPU on a
// socket for an uncore event.
//
    pub pmu_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub fd: *mut xyarray,
    pub mmap: *mut xyarray,
    pub sample_id: *mut xyarray,
    pub id: *mut u64,
    pub ids: u32,
    pub leader: *mut perf_evsel,
// For events where the read_format value is per-thread rather than
// global, stores the per-thread cumulative period
    pub per_stream_periods: list_head,
// parse modifier helper
    pub nr_members: c_int,
//
// system_wide is for events that need to be on every CPU, irrespective
// of user requested CPUs or threads. Tha main example of this is the
// dummy event. Map propagation will set cpus for this event to all CPUs
// as software PMU events like dummy, have a CPU map that is empty.
//
    pub system_wide: bool,
//
// Some events, for example uncore events, require a CPU.
// i.e. it cannot be the 'any CPU' value of -1.
//
    pub requires_cpu: bool,
// Is the PMU for the event a core one? Effects the handling of own_cpus.
    pub is_pmu_core: bool,
// Does the evsel on read on the first CPU index such as tool time events?
    pub reads_only_on_cpu_idx0: bool,
    pub idx: c_int,
}

extern "C" {
    pub fn perf_evsel__exit(evsel: *mut perf_evsel);
}
extern "C" {
    pub fn perf_evsel__alloc_fd(evsel: *mut perf_evsel, ncpus: c_int, nthreads: c_int) -> c_int;
}
extern "C" {
    pub fn perf_evsel__close_fd(evsel: *mut perf_evsel);
}
extern "C" {
    pub fn perf_evsel__free_fd(evsel: *mut perf_evsel);
}
extern "C" {
    pub fn perf_evsel__read_size(evsel: *mut perf_evsel) -> c_int;
}
extern "C" {
    pub fn perf_evsel__apply_filter(evsel: *mut perf_evsel, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_evsel__alloc_id(evsel: *mut perf_evsel, ncpus: c_int, nthreads: c_int) -> c_int;
}
extern "C" {
    pub fn perf_evsel__free_id(evsel: *mut perf_evsel);
}
extern "C" {
    pub fn perf_evsel__attr_has_per_thread_sample_period(evsel: *mut perf_evsel) -> bool;
}
