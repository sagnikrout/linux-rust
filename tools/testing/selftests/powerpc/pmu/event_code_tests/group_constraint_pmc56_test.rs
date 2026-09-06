//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_pmc56_test.c
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
// Testcase for checking constraint checks for
// Performance Monitor Counter 5 (PMC5) and also
// Performance Monitor Counter 6 (PMC6). Events using
// PMC5/PMC6 shouldn't have other fields in event
// code like cache bits, thresholding or marked bit.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_pmc56() -> c_int {
    static int group_constraint_pmc56(void)
    {
    struct event event;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// Events using PMC5 and PMC6 with cache bit
// set in event code is expected to fail.
//
    event_init(&event, 0x2500fa);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x2600f4);
    FAIL_IF(!event_open(&event));
//
// PMC5 and PMC6 only supports base events:
// ie 500fa and 600f4. Other combinations
// should fail.
//
    event_init(&event, 0x501e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x6001e);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x501fa);
    FAIL_IF(!event_open(&event));
//
// Events using PMC5 and PMC6 with random
// sampling bits set in event code should fail
// to schedule.
//
    event_init(&event, 0x35340500fa);
    FAIL_IF(!event_open(&event));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_pmc56, "group_constraint_pmc56");
    }
