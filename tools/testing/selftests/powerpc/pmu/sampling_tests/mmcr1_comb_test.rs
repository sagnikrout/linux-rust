//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/mmcr1_comb_test.c
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

// All successful D-side store dispatches for this thread that were L2 Miss
pub const EventCode: c_uint = 0x46880;
    extern void thirty_two_instruction_loop_with_ll_sc(u64 loops, u64 *ll_sc_target);
//
// A perf sampling test for mmcr1
// fields : comb.
//
#[no_mangle]
unsafe extern "C" fn mmcr1_comb() -> c_int {
    static int mmcr1_comb(void)
    {
    struct event event;
    u64 *intr_regs;
    u64 dummy;
// Check for platform support for the test
    SKIP_IF(check_pvr_for_sampling_tests());
// Init the event for the sampling test
    event_init_sampling(&event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF(event_open(&event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);
    FAIL_IF(event_enable(&event));
// workload to make the event overflow
    thirty_two_instruction_loop_with_ll_sc(10000000, &dummy);
    FAIL_IF(event_disable(&event));
// Check for sample count
    FAIL_IF(!collect_samples(event.mmap_buffer));
    intr_regs = get_intr_regs(&event, event.mmap_buffer);
// Check for intr_regs
    FAIL_IF(!intr_regs);
//
// Verify that comb field match with
// corresponding event code fields
//
    FAIL_IF(EV_CODE_EXTRACT(event.attr.config, comb) !=
    get_mmcr1_comb(get_reg_value(intr_regs, "MMCR1"), 4));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(mmcr1_comb, "mmcr1_comb");
    }
