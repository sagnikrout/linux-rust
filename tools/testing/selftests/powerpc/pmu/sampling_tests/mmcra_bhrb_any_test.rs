//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/mmcra_bhrb_any_test.c
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

    extern void thirty_two_instruction_loop(int loops);
// Instructions
pub const EventCode: c_uint = 0x500fa;
// ifm field for any branch mode
pub const IFM_ANY_BRANCH: c_uint = 0x0;
//
// A perf sampling test for mmcra
// field: ifm for bhrb any call.
//
#[no_mangle]
unsafe extern "C" fn mmcra_bhrb_any_test() -> c_int {
    static int mmcra_bhrb_any_test(void)
    {
    struct event event;
    u64 *intr_regs;
// Check for platform support for the test
    SKIP_IF(check_pvr_for_sampling_tests());
// Init the event for the sampling test
    event_init_sampling(&event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_ANY;
    event.attr.exclude_kernel = 1;
    FAIL_IF(event_open(&event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);
    FAIL_IF(event_enable(&event));
// workload to make the event overflow
    thirty_two_instruction_loop(10000);
    FAIL_IF(event_disable(&event));
    intr_regs = get_intr_regs(&event, event.mmap_buffer);
// Check for intr_regs
    FAIL_IF(!intr_regs);
// Verify that ifm bit is set properly in MMCRA
    FAIL_IF(get_mmcra_ifm(get_reg_value(intr_regs, "MMCRA"), 5) != IFM_ANY_BRANCH);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(mmcra_bhrb_any_test, "mmcra_bhrb_any_test");
    }
