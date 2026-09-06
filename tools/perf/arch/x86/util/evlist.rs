//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/evlist.c
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

#[no_mangle]
pub unsafe extern "C" fn arch_evlist__cmp(lhs: *const evsel, rhs: *const evsel) -> c_int {
    int arch_evlist__cmp(const struct evsel *lhs, const struct evsel *rhs)
    {
//
// Currently the following topdown events sequence are supported to
// move and regroup correctly.
//
// a. all events in a group
// perf stat -e "{instructions,topdown-retiring,slots}" -C0 sleep 1
// WARNING: events were regrouped to match PMUs
// Performance counter stats for 'CPU(s) 0':
// 15,066,240     slots
// 1,899,760      instructions
// 2,126,998      topdown-retiring
// b. all events not in a group
// perf stat -e "instructions,topdown-retiring,slots" -C0 sleep 1
// WARNING: events were regrouped to match PMUs
// Performance counter stats for 'CPU(s) 0':
// 2,045,561      instructions
// 17,108,370     slots
// 2,281,116      topdown-retiring
// c. slots event in a group but topdown metrics events outside the group
// perf stat -e "{instructions,slots},topdown-retiring" -C0 sleep 1
// WARNING: events were regrouped to match PMUs
// Performance counter stats for 'CPU(s) 0':
// 20,323,878      slots
// 2,634,884      instructions
// 3,028,656      topdown-retiring
// d. slots event and topdown metrics events in two groups
// perf stat -e "{instructions,slots},{topdown-retiring}" -C0 sleep 1
// WARNING: events were regrouped to match PMUs
// Performance counter stats for 'CPU(s) 0':
// 26,319,024      slots
// 2,427,791      instructions
// 2,683,508      topdown-retiring
// e. slots event and metrics event are not in a group and not adjacent
// perf stat -e "{instructions,slots},cycles,topdown-retiring" -C0 sleep 1
// WARNING: events were regrouped to match PMUs
// 68,433,522      slots
// 8,856,102      topdown-retiring
// 7,791,494      instructions
// 11,469,513      cycles
//
    if (topdown_sys_has_perf_metrics() &&
    (arch_evsel__must_be_in_group(lhs) || arch_evsel__must_be_in_group(rhs))) {
// Ensure the topdown slots comes first.
    if (arch_is_topdown_slots(lhs))
    return -1;
    if (arch_is_topdown_slots(rhs))
    return 1;
//
// Move topdown metrics events forward only when topdown metrics
// events are not in same group with previous slots event. If
// topdown metrics events are already in same group with slots
// event, do nothing.
//
    if (lhs.core.leader != rhs.core.leader) {
    let mut lhs_topdown: bool = arch_is_topdown_metrics(lhs);
    let mut rhs_topdown: bool = arch_is_topdown_metrics(rhs);
    if (lhs_topdown && !rhs_topdown)
    return -1;
    if (!lhs_topdown && rhs_topdown)
    return 1;
    }
    }
// Retire latency event should not be group leader
    if (lhs.retire_lat && !rhs.retire_lat)
    return 1;
    if (!lhs.retire_lat && rhs.retire_lat)
    return -1;
// Default ordering by insertion index.
    return lhs.core.idx - rhs.core.idx;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_evlist__add_required_events(list: *mut list_head) -> c_int {
    int arch_evlist__add_required_events(struct list_head *list)
    {
    struct evsel *pos, *metric_event = core::ptr::null_mut();
    let mut idx: c_int = 0;
    if (!topdown_sys_has_perf_metrics())
    return 0;
    list_for_each_entry(pos, list, core.node) {
    if (arch_is_topdown_slots(pos)) {
// Slots event already present, nothing to do.
    return 0;
    }
    if (metric_event == core::ptr::null_mut() && arch_is_topdown_metrics(pos))
    metric_event = pos;
    idx++;
    }
    if (metric_event == core::ptr::null_mut()) {
// No topdown metric events, nothing to do.
    return 0;
    }
    return topdown_insert_slots_event(list, idx + 1, metric_event);
    }
