//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/evsel.h
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
pub const __PERF_EVSEL_H: c_int = 1;

extern "C" {
    pub fn int(event: *mut evsel__sb_cb_t)(union perf_event, data: *mut c_void) -> typedef;
}
// struct evsel - event selector
//
// @evlist - evlist this evsel is in, if it is in one.
// @core - libperf evsel object
// @name - Can be set to retain the original event name passed by the user,
// so that when showing results in tools such as 'perf stat', we
// show the name used, not some alias.
// @id_pos: the position of the event id (PERF_SAMPLE_ID or
// PERF_SAMPLE_IDENTIFIER) in a sample event i.e. in the array of
// struct perf_record_sample
// @is_pos: the position (counting backwards) of the event id (PERF_SAMPLE_ID or
// PERF_SAMPLE_IDENTIFIER) in a non-sample event i.e. if sample_id_all
// is used there is an id sample appended to non-sample events
// @priv:   And what is in its containing unnamed union are tool specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evsel {
    pub core: perf_evsel,
    pub evlist: *mut evlist,
    pub refcnt: refcount_t,
    pub id_offset: off_t,
    pub id_pos: c_int,
    pub is_pos: c_int,
    pub sample_size: c_uint,
//
// These fields can be set in the parse-events code or similar.
// Please check evsel__clone() to copy them properly so that
// they can be released properly.
//
    pub name: *mut c_char,
    pub group_name: *mut c_char,
    pub group_pmu_name: *const c_char,

    pub tp_sys: *mut c_char,
    pub tp_name: *mut c_char,
    pub tp_format: *mut tep_event,

    pub filter: *mut c_char,
    pub max_events: c_ulong,
    pub scale: double,
    pub unit: *const c_char,
    pub cgrp: *mut cgroup,
    pub metric_id: *const c_char,
// The PMU the event is from. Used for missing_features, PMU name, etc.
    pub pmu: *mut perf_pmu,
//
// This point to the first evsel with the same name, intended to store the
// aggregated counts in aggregation mode.
//
    pub first_wildcard_match: *mut evsel,
// parse modifier helper
    pub exclude_GH: c_int,
    pub sample_read: c_int,
    pub snapshot: bool,
    pub per_pkg: bool,
    pub percore: bool,
    pub precise_max: bool,
    pub is_libpfm_event: bool,
    pub collect_stat: bool,
    pub weak_group: bool,
    pub bpf_counter: bool,
    pub use_config_name: bool,
    pub skippable: bool,
    pub retire_lat: bool,
    pub dont_regroup: bool,
    pub /: *mut *mut bool default_metricgroup; / A member of the Default metricgroup,
    pub /: *mut *mut bool default_show_events; / If a default group member, show the event,
    pub config_terms: list_head,
    pub alternate_hw_config: u64,
}

//
// metric fields are similar, but needs more care as they can have
// references to other metric (evsel).
//
// For reporting purposes, an evsel sample can have a callchain
// synthesized from AUX area data. Keep track of synthesized sample
// types here. Note, the recorded sample_type cannot be changed because
// it is needed to continue to parse events.
// See also evsel__has_callchain().
//
// Store the branch counter related information.
// br_cntr_idx: The idx of the branch counter event in the evlist
// br_cntr_nr:  The number of the branch counter event in the group
// (Only available for the leader event)
// abbr_name:   The abbreviation name assigned to an event which is
// logged by the branch counter.
// The abbr name is from A to Z9. NA is applied if out
// of the range.
//
// bpf_counter_ops serves two use cases:
// 1. perf-stat -b          counting events used byBPF programs
// 2. perf-stat --use-bpf   use BPF programs to aggregate counts
//
// for perf-stat --use-bpf
// For tool events
// Beginning time subtracted when the counter is read.
// Defaults for retirement latency events.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _retirement_latency {
    pub mean: double,
    pub min: double,
    pub max: double,
    pub retirement_latency: },
// duration_time is a single global time.
    pub start_time: __u64,
    pub accumulated_time: __u64,
    pub duration_time: },
//
// user_time and system_time read an initial value potentially
// per-CPU or per-pid.
//
    pub start_times: *mut xyarray,
    pub accumulated_times: *mut xyarray,
    pub process_time: },
}

// Is the tool's fd for /proc/pid/stat or /proc/stat.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_missing_features {
    pub sample_id_all: bool,
    pub exclude_guest: bool,
    pub mmap2: bool,
    pub cloexec: bool,
    pub clockid: bool,
    pub clockid_wrong: bool,
    pub lbr_flags: bool,
    pub write_backward: bool,
    pub group_read: bool,
    pub ksymbol: bool,
    pub bpf: bool,
    pub aux_output: bool,
    pub branch_hw_idx: bool,
    pub cgroup: bool,
    pub data_page_size: bool,
    pub code_page_size: bool,
    pub weight_struct: bool,
    pub read_lost: bool,
    pub branch_counters: bool,
    pub aux_action: bool,
    pub inherit_sample_read: bool,
    pub defer_callchain: bool,
}

extern "C" {
    pub fn perf_evsel__cpus(_arg: &evsel->core) -> return;
}
extern "C" {
    pub fn perf_cpu_map__nr(_arg: evsel__cpus(evsel)) -> return;
}
extern "C" {
    pub fn evsel__is_aux_event(evsel: *const evsel) -> bool;
}
extern "C" {
    pub fn evsel__is_probe(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__is_kprobe(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__is_uprobe(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__new_idx(_arg: attr, _arg: 0) -> return;
}
extern "C" {
    pub fn copy_config_terms(dst: *mut list_head, src: *mut list_head) -> c_int;
}
extern "C" {
    pub fn free_config_terms(config_terms: *mut list_head);
}
//
// Returns pointer with encoded error via <linux/err.h> interface.
//
extern "C" {
    pub fn evsel__newtp_idx(_arg: sys, _arg: name, _arg: 0, _arg: true) -> return;
}
extern "C" {
    pub fn evsel__put(evsel: *mut evsel);
}

extern "C" {
    pub fn evsel__set_priv_destructor(priv): *mut *mut void (destructor)(void);
}
extern "C" {
    pub fn __evsel__sample_size(sample_type: u64) -> c_int;
}
extern "C" {
    pub fn evsel__calc_id_pos(evsel: *mut evsel);
}
extern "C" {
    pub fn evsel__is_cache_op_valid(type: u8, op: u8) -> bool;
}
pub const EVSEL__MAX_ALIASES: c_int = 8;
extern "C" {
    pub fn evsel__match_bpf_counter_events(name: *const c_char) -> bool;
}
extern "C" {
    pub fn arch_evsel__hw_name(evsel: *mut evsel, bf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn __evsel__hw_cache_type_op_res_name(type: u8, op: u8, result: u8, bf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
}
extern "C" {
    pub fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn __evsel__set_sample_bit(evsel: *mut evsel, bit: perf_event_sample_format);
}
extern "C" {
    pub fn __evsel__reset_sample_bit(evsel: *mut evsel, bit: perf_event_sample_format);
}

extern "C" {
    pub fn evsel__set_sample_id(evsel: *mut evsel, use_sample_identifier: bool);
}
extern "C" {
    pub fn arch_evsel__set_sample_weight(evsel: *mut evsel);
}
extern "C" {
    pub fn arch__post_evsel_config(evsel: *mut evsel, attr: *mut perf_event_attr);
}
extern "C" {
    pub fn arch_evsel__open_strerror(evsel: *mut evsel, err: c_int, msg: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn arch_evsel__apply_ratio_to_prev(evsel: *mut evsel, attr: *mut perf_event_attr);
}
extern "C" {
    pub fn evsel__set_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn evsel__append_tp_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn evsel__append_addr_filter(evsel: *mut evsel, filter: *const c_char) -> c_int;
}
extern "C" {
    pub fn evsel__enable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
}
extern "C" {
    pub fn evsel__enable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn evsel__disable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn evsel__disable_cpu(evsel: *mut evsel, cpu_map_idx: c_int) -> c_int;
}
extern "C" {
    pub fn evsel__open_per_cpu(evsel: *mut evsel, cpus: *mut perf_cpu_map, cpu_map_idx: c_int) -> c_int;
}
extern "C" {
    pub fn evsel__open_per_thread(evsel: *mut evsel, threads: *mut perf_thread_map) -> c_int;
}
extern "C" {
    pub fn evsel__close(evsel: *mut evsel);
}
extern "C" {
    pub fn evsel__precise_ip_fallback(evsel: *mut evsel) -> bool;
}

extern "C" {
    pub fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
}
extern "C" {
    pub fn perf_sample__intval_common(sample: *mut perf_sample, name: *const c_char) -> u64;
}
extern "C" {
    pub fn perf_sample__taskstate(sample: *mut perf_sample, name: *const c_char) -> c_char;
}
extern "C" {
    pub fn perf_sample__rawptr(_arg: sample, _arg: name) -> return;
}

extern "C" {
    pub fn format_field__intval(field: *mut tep_format_field, sample: *mut perf_sample, needs_swap: bool) -> u64;
}

extern "C" {
    pub fn __evsel__match(evsel: *const evsel, type: u32, config: u64) -> bool;
}

extern "C" {
    pub fn evsel__read_counter(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
}
extern "C" {
    pub fn __evsel__read_on_cpu(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int, scale: bool) -> c_int;
}
//
// evsel__read_on_cpu - Read out the results on a CPU and thread
//
// @evsel - event selector to read value
// @cpu_map_idx - CPU of interest
// @thread - thread of interest
//
extern "C" {
    pub fn __evsel__read_on_cpu(_arg: evsel, _arg: cpu_map_idx, _arg: thread, _arg: false) -> return;
}
//
// evsel__read_on_cpu_scaled - Read out the results on a CPU and thread, scaled
//
// @evsel - event selector to read value
// @cpu_map_idx - CPU of interest
// @thread - thread of interest
//
extern "C" {
    pub fn __evsel__read_on_cpu(_arg: evsel, _arg: cpu_map_idx, _arg: thread, _arg: true) -> return;
}
extern "C" {
    pub fn __evsel__parse_sample(_arg: evsel, _arg: event, _arg: data, _arg: evsel->needs_swap) -> return;
}
extern "C" {
    pub fn evsel__id_hdr_size(evsel: *const evsel) -> u16;
}
extern "C" {
    pub fn list_entry(_arg: evsel->core.node.next, evsel: struct, _arg: core.node) -> return;
}
extern "C" {
    pub fn list_entry(_arg: evsel->core.node.prev, evsel: struct, _arg: core.node) -> return;
}
//
// evsel__is_group_leader - Return whether given evsel is a leader event
//
// @evsel - evsel selector to be tested
//
// Return %true if @evsel is a group leader or a stand-alone event
//
// evsel__is_group_event - Return whether given evsel is a group event
//
// @evsel - evsel selector to be tested
//
// Return %true iff event group view is enabled and @evsel is a actual group
// leader which has other members in the group
//
extern "C" {
    pub fn evsel__is_non_perf_event_open_pmu(evsel: *const evsel) -> bool;
}
extern "C" {
    pub fn evsel__is_function_event(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__match(_arg: evsel, _arg: SOFTWARE, _arg: SW_BPF_OUTPUT) -> return;
}
// Iterates group WITHOUT the leader.

// Iterates group WITH the leader.

//
// For reporting purposes, an evsel sample can have a recorded callchain
// or a callchain synthesized from AUX area data.
//
// For reporting purposes, an evsel sample can have a recorded branch
// stack or a branch stack synthesized from AUX area data.
//
extern "C" {
    pub fn evsel__e_machine(evsel: *mut evsel, e_flags: *mut u32) -> u16;
}
extern "C" {
    pub fn evsel__store_ids(evsel: *mut evsel, evlist: *mut evlist) -> c_int;
}
extern "C" {
    pub fn evsel__zero_per_pkg(evsel: *mut evsel);
}
extern "C" {
    pub fn evsel__is_hybrid(evsel: *const evsel) -> bool;
}
extern "C" {
    pub fn evsel__has_leader(evsel: *mut evsel, leader: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__is_leader(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
}
extern "C" {
    pub fn evsel__source_count(evsel: *const evsel) -> c_int;
}
extern "C" {
    pub fn evsel__remove_from_group(evsel: *mut evsel, leader: *mut evsel);
}
extern "C" {
    pub fn arch_evsel__must_be_in_group(evsel: *const evsel) -> bool;
}
extern "C" {
    pub fn evsel__set_needs_uniquify(counter: *mut evsel, config: *const perf_stat_config) -> bool;
}
extern "C" {
    pub fn evsel__uniquify_counter(counter: *mut evsel);
}
//
// Macro to swap the bit-field postition and size.
// Used when,
// - dont need to swap the entire u64 &&
// - when u64 has variable bit-field sizes &&
// - when presented in a host endian which is different
// than the source endian of the perf.data file
//

extern "C" {
    pub fn evsel__bitfield_swap_branch_flags(value: u64) -> u64;
}
extern "C" {
    pub fn evsel__config_exists(evsel: *const evsel, config_name: *const c_char) -> bool;
}
extern "C" {
    pub fn evsel__is_offcpu_event(evsel: *mut evsel) -> bool;
}
extern "C" {
    pub fn evsel__warn_user_requested_cpus(evsel: *mut evsel, user_requested_cpus: *mut perf_cpu_map);
}
