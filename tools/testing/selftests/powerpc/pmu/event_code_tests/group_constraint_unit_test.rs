//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_unit_test.c
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
// Copyright 2022, Kajol Jain, IBM Corp.
//

// All successful D-side store dispatches for this thread with PMC 2
pub const EventCode_1: c_uint = 0x26080;
// All successful D-side store dispatches for this thread with PMC 4
pub const EventCode_2: c_uint = 0x46080;
// All successful D-side store dispatches for this thread that were L2 Miss with PMC 3
pub const EventCode_3: c_uint = 0x36880;
//
// Testcase for group constraint check of unit and pmc bits which is
// used to program corresponding unit and pmc field in Monitor Mode
// Control Register 1 (MMCR1)
// One of the event in the group should use PMC 4 incase units field
// value is within 6 to 9 otherwise event_open for the group will fail.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_unit() -> c_int {
    static int group_constraint_unit(void)
    {
    struct event *e, events[3];
//
// Check for platform support for the test.
// Constraint to use PMC4 with one of the event in group,
// when the unit is within 6 to 9 is only applicable on
// power9.
//
    SKIP_IF(platform_check_for_tests());
    SKIP_IF(have_hwcap2(PPC_FEATURE2_ARCH_3_1));
// Init the events for the group contraint check for unit bits
    e = &events[0];
    event_init(e, EventCode_1);
// Expected to fail as PMC 4 is not used with unit field value 6 to 9
    FAIL_IF(!event_open(&events[0]));
// Init the events for the group contraint check for unit bits
    e = &events[1];
    event_init(e, EventCode_2);
// Expected to pass as PMC 4 is used with unit field value 6 to 9
    FAIL_IF(event_open(&events[1]));
// Init the event for the group contraint unit test
    e = &events[2];
    event_init(e, EventCode_3);
// Expected to fail as PMC4 is not being used
    FAIL_IF(!event_open_with_group(&events[2], events[0].fd));
// Expected to succeed as event using PMC4
    FAIL_IF(event_open_with_group(&events[2], events[1].fd));
    event_close(&events[0]);
    event_close(&events[1]);
    event_close(&events[2]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_unit, "group_constraint_unit");
    }
