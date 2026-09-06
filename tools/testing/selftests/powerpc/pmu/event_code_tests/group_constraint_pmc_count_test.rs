//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_pmc_count_test.c
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
// Copyright 2022, Athira Rajeev, IBM Corp.
//

//
// Testcase for number of counters in use.
// The number of programmable counters is from
// performance monitor counter 1 to performance
// monitor counter 4 (PMC1-PMC4). If number of
// counters in use exceeds the limit, next event
// should fail to schedule.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_pmc_count() -> c_int {
    static int group_constraint_pmc_count(void)
    {
    struct event *e, events[5];
    int i;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// Test for number of counters in use.
// Use PMC1 to PMC4 for leader and 3 sibling
// events. Trying to open fourth event should
// fail here.
//
    e = &events[0];
    event_init(e, 0x1001a);
    e = &events[1];
    event_init(e, 0x200fc);
    e = &events[2];
    event_init(e, 0x30080);
    e = &events[3];
    event_init(e, 0x40054);
    e = &events[4];
    event_init(e, 0x0002c);
    FAIL_IF(event_open(&events[0]));
//
// The event_open will fail on event 4 if constraint
// check fails
//
    for (i = 1; i < 5; i++) {
    if (i == 4)
    FAIL_IF(!event_open_with_group(&events[i], events[0].fd));
    else
    FAIL_IF(event_open_with_group(&events[i], events[0].fd));
    }
    for (i = 1; i < 4; i++)
    event_close(&events[i]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_pmc_count, "group_constraint_pmc_count");
    }
