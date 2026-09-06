//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/generic_events_valid_test.c
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
// Testcase to ensure that using invalid event in generic
// event for PERF_TYPE_HARDWARE should fail
//
#[no_mangle]
unsafe extern "C" fn generic_events_valid_test() -> c_int {
    static int generic_events_valid_test(void)
    {
    struct event event;
    let mut pvr: c_int = mfspr(SPRN_PVR);
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
// generic events is different in compat_mode
    SKIP_IF(check_for_generic_compat_pmu());
//
// Invalid generic events in power10:
// - PERF_COUNT_HW_BUS_CYCLES
// - PERF_COUNT_HW_STALLED_CYCLES_FRONTEND
// - PERF_COUNT_HW_STALLED_CYCLES_BACKEND
// - PERF_COUNT_HW_REF_CPU_CYCLES
//
    if ((pvr == POWER10) || (pvr == POWER11)) {
    event_init_opts(&event, PERF_COUNT_HW_CPU_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_CACHE_REFERENCES,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_CACHE_MISSES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BRANCH_MISSES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BUS_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    event_init_opts(&event, PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    event_init_opts(&event, PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    event_init_opts(&event, PERF_COUNT_HW_REF_CPU_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    } else if (PVR_VER(mfspr(SPRN_PVR)) == POWER9) {
//
// Invalid generic events in power9:
// - PERF_COUNT_HW_BUS_CYCLES
// - PERF_COUNT_HW_REF_CPU_CYCLES
//
    event_init_opts(&event, PERF_COUNT_HW_CPU_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_INSTRUCTIONS, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_CACHE_REFERENCES,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_CACHE_MISSES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BRANCH_MISSES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_BUS_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    event_init_opts(&event, PERF_COUNT_HW_STALLED_CYCLES_FRONTEND,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_STALLED_CYCLES_BACKEND,
    PERF_TYPE_HARDWARE, "event");
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init_opts(&event, PERF_COUNT_HW_REF_CPU_CYCLES, PERF_TYPE_HARDWARE, "event");
    FAIL_IF(!event_open(&event));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(generic_events_valid_test, "generic_events_valid_test");
    }
