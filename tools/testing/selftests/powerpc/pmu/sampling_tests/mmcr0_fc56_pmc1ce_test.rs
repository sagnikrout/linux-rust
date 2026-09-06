//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/mmcr0_fc56_pmc1ce_test.c
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

    extern void thirty_two_instruction_loop(int loops);
//
// A perf sampling test for mmcr0
// fields: fc56, pmc1ce.
//
#[no_mangle]
unsafe extern "C" fn mmcr0_fc56_pmc1ce() -> c_int {
    static int mmcr0_fc56_pmc1ce(void)
    {
    struct event event;
    u64 *intr_regs;
// Check for platform support for the test
    SKIP_IF(check_pvr_for_sampling_tests());
// Init the event for the sampling test
    event_init_sampling(&event, 0x1001e);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF(event_open(&event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);
    FAIL_IF(event_enable(&event));
// workload to make the event overflow
    thirty_two_instruction_loop(10000);
    FAIL_IF(event_disable(&event));
// Check for sample count
    FAIL_IF(!collect_samples(event.mmap_buffer));
    intr_regs = get_intr_regs(&event, event.mmap_buffer);
// Check for intr_regs
    FAIL_IF(!intr_regs);
// Verify that fc56, pmc1ce fields are set in MMCR0
    FAIL_IF(!get_mmcr0_fc56(get_reg_value(intr_regs, "MMCR0"), 1));
    FAIL_IF(!get_mmcr0_pmc1ce(get_reg_value(intr_regs, "MMCR0"), 1));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(mmcr0_fc56_pmc1ce, "mmcr0_fc56_pmc1ce");
    }
