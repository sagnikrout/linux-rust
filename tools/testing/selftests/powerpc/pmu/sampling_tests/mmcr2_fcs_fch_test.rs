//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/sampling_tests/mmcr2_fcs_fch_test.c
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
// Copyright 2022, Madhavan Srinivasan, IBM Corp.
//

    extern void thirty_two_instruction_loop(int loops);
    static bool is_hv;
#[no_mangle]
unsafe extern "C" fn sig_usr2_handler(signum: c_int, info: *mut siginfo_t, data: *mut c_void) {
    static void sig_usr2_handler(int signum, siginfo_t *info, void *data)
    {
    ucontext_t *uctx = data;
    is_hv = !!(uctx.uc_mcontext.gp_regs[PT_MSR] & MSR_HV);
    }
//
// A perf sampling test for mmcr2
// fields : fcs, fch.
//
#[no_mangle]
unsafe extern "C" fn mmcr2_fcs_fch() -> c_int {
    static int mmcr2_fcs_fch(void)
    {
    struct sigaction sigact = {
    .sa_sigaction = sig_usr2_handler,
    .sa_flags = SA_SIGINFO
    };
    struct event event;
    u64 *intr_regs;
    FAIL_IF(sigaction(SIGUSR2, &sigact, core::ptr::null_mut()));
    FAIL_IF(kill(getpid(), SIGUSR2));
// Check for platform support for the test
    SKIP_IF(check_pvr_for_sampling_tests());
// Init the event for the sampling test
    event_init_sampling(&event, 0x1001e);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.exclude_kernel = 1;
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
//
// Verify that fcs and fch field of MMCR2 match
// with corresponding modifier fields.
//
    if (is_hv)
    FAIL_IF(event.attr.exclude_kernel !=
    get_mmcr2_fch(get_reg_value(intr_regs, "MMCR2"), 1));
    else
    FAIL_IF(event.attr.exclude_kernel !=
    get_mmcr2_fcs(get_reg_value(intr_regs, "MMCR2"), 1));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(mmcr2_fcs_fch, "mmcr2_fcs_fch");
    }
