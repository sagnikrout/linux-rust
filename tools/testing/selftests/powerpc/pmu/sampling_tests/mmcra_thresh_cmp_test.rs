//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/mmcra_thresh_cmp_test.c
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
// Primary PMU event used here is PM_MRK_INST_CMPL (0x401e0)
// Threshold event selection used is issue to complete for cycles
// Sampling criteria is Load only sampling
//
pub const p9_EventCode: c_uint = 0x13E35340401e0;
pub const p10_EventCode: c_uint = 0x35340401e0;
    extern void thirty_two_instruction_loop_with_ll_sc(u64 loops, u64 *ll_sc_target);
// A perf sampling test to test mmcra fields
#[no_mangle]
unsafe extern "C" fn mmcra_thresh_cmp() -> c_int {
    static int mmcra_thresh_cmp(void)
    {
    struct event event;
    u64 *intr_regs;
    u64 dummy;
// Check for platform support for the test
    SKIP_IF(check_pvr_for_sampling_tests());
// Skip for comapt mode
    SKIP_IF(check_for_compat_mode());
// Init the event for the sampling test
    if (!have_hwcap2(PPC_FEATURE2_ARCH_3_1)) {
    event_init_sampling(&event, p9_EventCode);
    } else {
    event_init_sampling(&event, p10_EventCode);
    event.attr.config1 = 1000;
    }
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF(event_open(&event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);
    FAIL_IF(event_enable(&event));
// workload to make the event overflow
    thirty_two_instruction_loop_with_ll_sc(1000000, &dummy);
    FAIL_IF(event_disable(&event));
// Check for sample count
    FAIL_IF(!collect_samples(event.mmap_buffer));
    intr_regs = get_intr_regs(&event, event.mmap_buffer);
// Check for intr_regs
    FAIL_IF(!intr_regs);
// Verify that thresh cmp match with the corresponding event code fields
    FAIL_IF(get_thresh_cmp_val(event) !=
    get_mmcra_thd_cmp(get_reg_value(intr_regs, "MMCRA"), 4));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    FAIL_IF(test_harness(mmcra_thresh_cmp, "mmcra_thresh_cmp"));
    }
