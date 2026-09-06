//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/hw_cache_event_type_test.c
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
// Load Missed L1, for power9 its pointing to PM_LD_MISS_L1_FIN (0x2c04e) and
// for power10 its pointing to PM_LD_MISS_L1 (0x3e054)
//
// Hardware cache level : PERF_COUNT_HW_CACHE_L1D
// Hardware cache event operation type : PERF_COUNT_HW_CACHE_OP_READ
// Hardware cache event result type : PERF_COUNT_HW_CACHE_RESULT_MISS
//
pub const EventCode_1: c_uint = 0x10000;
//
// Hardware cache level : PERF_COUNT_HW_CACHE_L1D
// Hardware cache event operation type : PERF_COUNT_HW_CACHE_OP_WRITE
// Hardware cache event result type : PERF_COUNT_HW_CACHE_RESULT_ACCESS
//
pub const EventCode_2: c_uint = 0x0100;
//
// Hardware cache level : PERF_COUNT_HW_CACHE_DTLB
// Hardware cache event operation type : PERF_COUNT_HW_CACHE_OP_WRITE
// Hardware cache event result type : PERF_COUNT_HW_CACHE_RESULT_ACCESS
//
pub const EventCode_3: c_uint = 0x0103;
//
// Hardware cache level : PERF_COUNT_HW_CACHE_L1D
// Hardware cache event operation type : PERF_COUNT_HW_CACHE_OP_READ
// Hardware cache event result type : Invalid ( > PERF_COUNT_HW_CACHE_RESULT_MAX)
//
pub const EventCode_4: c_uint = 0x030000;
//
// A perf test to check valid hardware cache events.
//
#[no_mangle]
unsafe extern "C" fn hw_cache_event_type_test() -> c_int {
    static int hw_cache_event_type_test(void)
    {
    struct event event;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
// Skip for Generic compat PMU
    SKIP_IF(check_for_generic_compat_pmu());
// Init the event to test hardware cache event
    event_init_opts(&event, EventCode_1, PERF_TYPE_HW_CACHE, "event");
// Expected to success as its pointing to L1 load miss
    FAIL_IF(event_open(&event));
    event_close(&event);
// Init the event to test hardware cache event
    event_init_opts(&event, EventCode_2, PERF_TYPE_HW_CACHE, "event");
// Expected to fail as the corresponding cache event entry have 0 in that index
    FAIL_IF(!event_open(&event));
    event_close(&event);
// Init the event to test hardware cache event
    event_init_opts(&event, EventCode_3, PERF_TYPE_HW_CACHE, "event");
// Expected to fail as the corresponding cache event entry have -1 in that index
    FAIL_IF(!event_open(&event));
    event_close(&event);
// Init the event to test hardware cache event
    event_init_opts(&event, EventCode_4, PERF_TYPE_HW_CACHE, "event");
// Expected to fail as hardware cache event result type is Invalid
    FAIL_IF(!event_open(&event));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(hw_cache_event_type_test, "hw_cache_event_type_test");
    }
