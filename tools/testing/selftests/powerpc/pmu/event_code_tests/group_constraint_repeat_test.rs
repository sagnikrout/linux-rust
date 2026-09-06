//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_repeat_test.c
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

// The processor's L1 data cache was reloaded
pub const EventCode1: c_uint = 0x21C040;
pub const EventCode2: c_uint = 0x22C040;
//
// Testcase for group constraint check
// when using events with same PMC.
// Multiple events in a group shouldn't
// ask for same PMC. If so it should fail.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_repeat() -> c_int {
    static int group_constraint_repeat(void)
    {
    struct event event, leader;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// Two events in a group using same PMC
// should fail to get scheduled. Usei same PMC2
// for leader and sibling event which is expected
// to fail.
//
    event_init(&leader, EventCode1);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode1);
// Expected to fail since sibling event is requesting same PMC as leader
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_init(&event, EventCode2);
// Expected to pass since sibling event is requesting different PMC
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_repeat, "group_constraint_repeat");
    }
