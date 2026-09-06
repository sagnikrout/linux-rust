//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/bhrb_no_crash_wo_pmu_test.c
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
// A perf sampling test for making sure
// enabling branch stack doesn't crash in any
// environment, say:
// - With generic compat PMU
// - without any PMU registered
// - With platform specific PMU
// A fix for bhrb sampling crash was added in kernel
// via commit: b460b512417a ("powerpc/perf: Fix crashes
// with generic_compat_pmu & BHRB")
//
// This testcase exercises this code by doing branch
// stack enable for software event. s/w event is used
// since software event will work even in platform
// without PMU.
//
#[no_mangle]
unsafe extern "C" fn bhrb_no_crash_wo_pmu_test() -> c_int {
    static int bhrb_no_crash_wo_pmu_test(void)
    {
    struct event event;
//
// Init the event for the sampling test.
// This uses software event which works on
// any platform.
//
    event_init_opts(&event, 0, PERF_TYPE_SOFTWARE, "cycles");
    event.attr.sample_period = 1000;
    event.attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    event.attr.disabled = 1;
//
// Return code of event_open is not
// considered since test just expects no crash from
// using PERF_SAMPLE_BRANCH_STACK. Also for environment
// like generic compat PMU, branch stack is unsupported.
//
    event_open(&event);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(bhrb_no_crash_wo_pmu_test, "bhrb_no_crash_wo_pmu_test");
    }
