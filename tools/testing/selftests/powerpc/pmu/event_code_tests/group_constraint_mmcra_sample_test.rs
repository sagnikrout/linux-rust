//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_mmcra_sample_test.c
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

pub const EventCode_1: c_uint = 0x35340401e0;
pub const EventCode_2: c_uint = 0x353c0101ec;
pub const EventCode_3: c_uint = 0x35340101ec;
//
// Test that using different sample bits in
// event code cause failure in schedule for
// group of events.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_mmcra_sample() -> c_int {
    static int group_constraint_mmcra_sample(void)
    {
    struct event event, leader;
    SKIP_IF(platform_check_for_tests());
//
// Events with different "sample" field values
// in a group will fail to schedule.
// Use event with load only sampling mode as
// group leader. Use event with store only sampling
// as sibling event.
//
    event_init(&leader, EventCode_1);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_2);
// Expected to fail as sibling event doesn't use same sampling bits as leader
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_init(&event, EventCode_3);
// Expected to pass as sibling event use same sampling bits as leader
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_mmcra_sample, "group_constraint_mmcra_sample");
    }
