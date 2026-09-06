//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_cache_test.c
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

// All L1 D cache load references counted at finish, gated by reject
pub const EventCode_1: c_uint = 0x1100fc;
// Load Missed L1
pub const EventCode_2: c_uint = 0x23e054;
// Load Missed L1
pub const EventCode_3: c_uint = 0x13e054;
//
// Testcase for group constraint check of data and instructions
// cache qualifier bits which is used to program cache select field in
// Monitor Mode Control Register 1 (MMCR1: 16-17) for l1 cache.
// All events in the group should match cache select bits otherwise
// event_open for the group will fail.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_cache() -> c_int {
    static int group_constraint_cache(void)
    {
    struct event event, leader;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
// Init the events for the group contraint check for l1 cache select bits
    event_init(&leader, EventCode_1);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_2);
// Expected to fail as sibling event doesn't request same l1 cache select bits as leader
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_close(&event);
// Init the event for the group contraint l1 cache select test
    event_init(&event, EventCode_3);
// Expected to succeed as sibling event request same l1 cache select bits as leader
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_cache, "group_constraint_cache");
    }
