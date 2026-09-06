//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/count_instructions.c
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
// Copyright 2013, Michael Ellerman, IBM Corp.
//
// Macro flag: #define _GNU_SOURCE

    extern void thirty_two_instruction_loop(u64 loops);
#[no_mangle]
unsafe extern "C" fn setup_event(e: *mut event, config: u64, name: *mut c_char) {
    static void setup_event(struct event *e, u64 config, char *name)
    {
    event_init_opts(e, config, PERF_TYPE_HARDWARE, name);
    e.attr.disabled = 1;
    e.attr.exclude_kernel = 1;
    e.attr.exclude_hv = 1;
    e.attr.exclude_idle = 1;
    }
    static int do_count_loop(struct event *events, u64 instructions,
    u64 overhead, bool report)
    {
    s64 difference, expected;
    double percentage;
    prctl(PR_TASK_PERF_EVENTS_ENABLE);
// Run for 1M instructions
    thirty_two_instruction_loop(instructions >> 5);
    prctl(PR_TASK_PERF_EVENTS_DISABLE);
    event_read(&events[0]);
    event_read(&events[1]);
    expected = instructions + overhead;
    difference = events[0].result.value - expected;
    percentage = (double)difference / events[0].result.value * 100;
    if (report) {
    event_report(&events[0]);
    event_report(&events[1]);
    printf("Looped for %llu instructions, overhead %llu\n", instructions, overhead);
    printf("Expected %llu\n", expected);
    printf("Actual   %llu\n", events[0].result.value);
    printf("Delta    %lld, %f%%\n", difference, percentage);
    }
    event_reset(&events[0]);
    event_reset(&events[1]);
    if (difference < 0)
    difference = -difference;
// Tolerate a difference below 0.0001 %
    difference *= 10000 * 100;
    if (difference / events[0].result.value)
    return -1;
    return 0;
    }
// Count how many instructions it takes to do a null loop
#[no_mangle]
unsafe extern "C" fn determine_overhead(events: *mut event) -> u64 {
    static u64 determine_overhead(struct event *events)
    {
    u64 current, overhead;
    int i;
    do_count_loop(events, 0, 0, false);
    overhead = events[0].result.value;
    for (i = 0; i < 100; i++) {
    do_count_loop(events, 0, 0, false);
    current = events[0].result.value;
    if (current < overhead) {
    printf("Replacing overhead %llu with %llu\n", overhead, current);
    overhead = current;
    }
    }
    return overhead;
    }
#[no_mangle]
unsafe extern "C" fn test_body() -> c_int {
    static int test_body(void)
    {
    struct event events[2];
    u64 overhead;
    setup_event(&events[0], PERF_COUNT_HW_INSTRUCTIONS, "instructions");
    setup_event(&events[1], PERF_COUNT_HW_CPU_CYCLES, "cycles");
    if (event_open(&events[0])) {
    perror("perf_event_open");
    return -1;
    }
    if (event_open_with_group(&events[1], events[0].fd)) {
    perror("perf_event_open");
    return -1;
    }
    overhead = determine_overhead(events);
    printf("Overhead of null loop: %llu instructions\n", overhead);
// Run for 1Mi instructions
    FAIL_IF(do_count_loop(events, 1000000, overhead, true));
// Run for 10Mi instructions
    FAIL_IF(do_count_loop(events, 10000000, overhead, true));
// Run for 100Mi instructions
    FAIL_IF(do_count_loop(events, 100000000, overhead, true));
// Run for 1Bi instructions
    FAIL_IF(do_count_loop(events, 1000000000, overhead, true));
// Run for 16Bi instructions
    FAIL_IF(do_count_loop(events, 16000000000, overhead, true));
// Run for 64Bi instructions
    FAIL_IF(do_count_loop(events, 64000000000, overhead, true));
    event_close(&events[0]);
    event_close(&events[1]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn count_instructions() -> c_int {
    static int count_instructions(void)
    {
    return eat_cpu(test_body);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(count_instructions, "count_instructions");
    }
