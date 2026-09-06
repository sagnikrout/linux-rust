//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/bhrb_filter_map_test.c
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
// A perf sampling test to check bhrb filter
// map. All the branch filters are not supported
// in powerpc. Supported filters in:
// power10/power11: any, any_call, ind_call, cond
// power9: any, any_call
//
// Testcase checks event open for invalid bhrb filter
// types should fail and valid filter types should pass.
// Testcase does validity check for these branch
// sample types.
//
// Invalid types for powerpc
// Valid bhrb filters in power9/power10/power11
    int bhrb_filter_map_valid_common[] = {
    PERF_SAMPLE_BRANCH_ANY,
    PERF_SAMPLE_BRANCH_ANY_CALL,
    };
// Valid bhrb filters in power10/power11
    int bhrb_filter_map_valid_p10[] = {
    PERF_SAMPLE_BRANCH_IND_CALL,
    PERF_SAMPLE_BRANCH_COND,
    };
pub const EventCode: c_uint = 0x1001e;
#[no_mangle]
unsafe extern "C" fn bhrb_filter_map_test() -> c_int {
    static int bhrb_filter_map_test(void)
    {
    struct event event;
    int i;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// Skip for Generic compat PMU since
// bhrb filters is not supported
//
    SKIP_IF(check_for_generic_compat_pmu());
// Init the event for the sampling test
    event_init(&event, EventCode);
    event.attr.sample_period = 1000;
    event.attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    event.attr.disabled = 1;
// Invalid filter maps which are expected to fail in event_open
    for (i = PERF_SAMPLE_BRANCH_USER_SHIFT; i < PERF_SAMPLE_BRANCH_MAX_SHIFT; i++) {
// Skip the valid branch sample type
    if (i == PERF_SAMPLE_BRANCH_ANY_SHIFT || i == PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT \
    || i == PERF_SAMPLE_BRANCH_IND_CALL_SHIFT || i == PERF_SAMPLE_BRANCH_COND_SHIFT)
    continue;
    event.attr.branch_sample_type = 1U << i;
    FAIL_IF(!event_open(&event));
    }
// valid filter maps for power9/power10/power11 which are expected to pass in event_open
    for (i = 0; i < ARRAY_SIZE(bhrb_filter_map_valid_common); i++) {
    event.attr.branch_sample_type = bhrb_filter_map_valid_common[i];
    FAIL_IF(event_open(&event));
    event_close(&event);
    }
//
// filter maps which are valid in power10/power11 and invalid in power9.
// PVR check is used here since PMU specific data like bhrb filter
// alternative tests is handled by respective PMU driver code and
// using PVR will work correctly for all cases including generic
// compat mode.
//
    switch (PVR_VER(mfspr(SPRN_PVR))) {
    case POWER11:
    case POWER10:
    for (i = 0; i < ARRAY_SIZE(bhrb_filter_map_valid_p10); i++) {
    event.attr.branch_sample_type = bhrb_filter_map_valid_p10[i];
    FAIL_IF(event_open(&event));
    event_close(&event);
    }
    break;
    default:
    for (i = 0; i < ARRAY_SIZE(bhrb_filter_map_valid_p10); i++) {
    event.attr.branch_sample_type = bhrb_filter_map_valid_p10[i];
    FAIL_IF(!event_open(&event));
    }
    }
//
// Combine filter maps which includes a valid branch filter and an invalid branch
// filter. Example: any ( PERF_SAMPLE_BRANCH_ANY) and any_call
// (PERF_SAMPLE_BRANCH_ANY_CALL).
// The perf_event_open should fail in this case.
//
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_ANY | PERF_SAMPLE_BRANCH_ANY_CALL;
    FAIL_IF(!event_open(&event));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(bhrb_filter_map_test, "bhrb_filter_map_test");
    }
