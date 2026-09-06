//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_pmc56_exclude_constraints_test.c
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
// Testcase for group constraint check for
// Performance Monitor Counter 5 (PMC5) and also
// Performance Monitor Counter 6 (PMC6).
// Test that pmc5/6 is excluded from constraint
// check when scheduled along with group of events.
//
#[no_mangle]
unsafe extern "C" fn group_pmc56_exclude_constraints() -> c_int {
    static int group_pmc56_exclude_constraints(void)
    {
    struct event *e, events[3];
    int i;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// PMC5/6 is excluded from constraint bit
// check along with group of events. Use
// group of events with PMC5, PMC6 and also
// event with cache bit (dc_ic) set. Test expects
// this set of events to go in as a group.
//
    e = &events[0];
    event_init(e, 0x500fa);
    e = &events[1];
    event_init(e, 0x600f4);
    e = &events[2];
    event_init(e, 0x22C040);
    FAIL_IF(event_open(&events[0]));
//
// The event_open will fail if constraint check fails.
// Since we are asking for events in a group and since
// PMC5/PMC6 is excluded from group constraints, even_open
// should pass.
//
    for (i = 1; i < 3; i++)
    FAIL_IF(event_open_with_group(&events[i], events[0].fd));
    for (i = 0; i < 3; i++)
    event_close(&events[i]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_pmc56_exclude_constraints, "group_pmc56_exclude_constraints");
    }
