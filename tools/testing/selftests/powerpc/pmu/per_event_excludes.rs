//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/per_event_excludes.c
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
//
// Copyright 2014, Michael Ellerman, IBM Corp.
//
// Macro flag: #define _GNU_SOURCE

//
// Test that per-event excludes work.
//
#[no_mangle]
unsafe extern "C" fn per_event_excludes() -> c_int {
    static int per_event_excludes(void)
    {
    struct event *e, events[4];
    int i;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
//
// We need to create the events disabled, otherwise the running/enabled
// counts don't match up.
//
    e = &events[0];
    event_init_opts(e, PERF_COUNT_HW_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "instructions");
    e.attr.disabled = 1;
    e = &events[1];
    event_init_opts(e, PERF_COUNT_HW_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "instructions(k)");
    e.attr.disabled = 1;
    e.attr.exclude_user = 1;
    e.attr.exclude_hv = 1;
    e = &events[2];
    event_init_opts(e, PERF_COUNT_HW_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "instructions(h)");
    e.attr.disabled = 1;
    e.attr.exclude_user = 1;
    e.attr.exclude_kernel = 1;
    e = &events[3];
    event_init_opts(e, PERF_COUNT_HW_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "instructions(u)");
    e.attr.disabled = 1;
    e.attr.exclude_hv = 1;
    e.attr.exclude_kernel = 1;
    FAIL_IF(event_open(&events[0]));
//
// The open here will fail if we don't have per event exclude support,
// because the second event has an incompatible set of exclude settings
// and we're asking for the events to be in a group.
//
    for (i = 1; i < 4; i++)
    FAIL_IF(event_open_with_group(&events[i], events[0].fd));
//
// Even though the above will fail without per-event excludes we keep
// testing in order to be thorough.
//
    prctl(PR_TASK_PERF_EVENTS_ENABLE);
// Spin for a while
    for (i = 0; i < INT_MAX; i++)
    asm volatile("" : : : "memory");
    prctl(PR_TASK_PERF_EVENTS_DISABLE);
    for (i = 0; i < 4; i++) {
    FAIL_IF(event_read(&events[i]));
    event_report(&events[i]);
    }
//
// We should see that all events have enabled == running. That
// shows that they were all on the PMU at once.
//
    for (i = 0; i < 4; i++)
    FAIL_IF(events[i].result.running != events[i].result.enabled);
//
// We can also check that the result for instructions is >= all the
// other counts. That's because it is counting all instructions while
// the others are counting a subset.
//
    for (i = 1; i < 4; i++)
    FAIL_IF(events[0].result.value < events[i].result.value);
    for (i = 0; i < 4; i++)
    event_close(&events[i]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(per_event_excludes, "per_event_excludes");
    }
