//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_thresh_cmp_test.c
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

//
// Primary PMU events used here is PM_MRK_INST_CMPL (0x401e0) and
// PM_THRESH_MET (0x101ec)
// Threshold event selection used is issue to complete for cycles
// Sampling criteria is Load or Store only sampling
//
pub const p9_EventCode_1: c_uint = 0x13e35340401e0;
pub const p9_EventCode_2: c_uint = 0x17d34340101ec;
pub const p9_EventCode_3: c_uint = 0x13e35340101ec;
pub const p10_EventCode_1: c_uint = 0x35340401e0;
pub const p10_EventCode_2: c_uint = 0x35340101ec;
//
// Testcase for group constraint check of thresh_cmp bits which is
// used to program thresh compare field in Monitor Mode Control Register A
// (MMCRA: 9-18 bits for power9 and MMCRA: 8-18 bits for power10/power11).
// All events in the group should match thresh compare bits otherwise
// event_open for the group will fail.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_thresh_cmp() -> c_int {
    static int group_constraint_thresh_cmp(void)
    {
    struct event event, leader;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
    if (have_hwcap2(PPC_FEATURE2_ARCH_3_1)) {
// Init the events for the group contraint check for thresh_cmp bits
    event_init(&leader, p10_EventCode_1);
// Add the thresh_cmp value for leader in config1
    leader.attr.config1 = 1000;
    FAIL_IF(event_open(&leader));
    event_init(&event, p10_EventCode_2);
// Add the different thresh_cmp value from the leader event in config1
    event.attr.config1 = 2000;
// Expected to fail as sibling and leader event request different thresh_cmp bits
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_close(&event);
// Init the event for the group contraint thresh compare test
    event_init(&event, p10_EventCode_2);
// Add the same thresh_cmp value for leader and sibling event in config1
    event.attr.config1 = 1000;
// Expected to succeed as sibling and leader event request same thresh_cmp bits
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    } else {
// Init the events for the group contraint check for thresh_cmp bits
    event_init(&leader, p9_EventCode_1);
    FAIL_IF(event_open(&leader));
    event_init(&event, p9_EventCode_2);
// Expected to fail as sibling and leader event request different thresh_cmp bits
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_close(&event);
// Init the event for the group contraint thresh compare test
    event_init(&event, p9_EventCode_3);
// Expected to succeed as sibling and leader event request same thresh_cmp bits
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_thresh_cmp, "group_constraint_thresh_cmp");
    }
