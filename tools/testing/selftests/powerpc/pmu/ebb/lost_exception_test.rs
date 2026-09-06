//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/lost_exception_test.c
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
// Copyright 2014, Michael Ellerman, IBM Corp.
//

//
// Test that tries to trigger CPU_FTR_PMAO_BUG. Which is a hardware defect
// where an exception triggers but we context switch before it is delivered and
// lose the exception.
//
#[no_mangle]
unsafe extern "C" fn test_body() -> c_int {
    static int test_body(void)
    {
    int i, orig_period, max_period;
    struct event event;
    SKIP_IF(!ebb_is_supported());
// We use PMC4 to make sure the kernel switches all counters correctly
    event_init_named(&event, 0x40002, "instructions");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open(&event));
    ebb_enable_pmc_counting(4);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&event));
//
// We want a low sample period, but we also want to get out of the EBB
// handler without tripping up again.
//
// This value picked after much experimentation.
//
    orig_period = max_period = sample_period = 400;
    mtspr(SPRN_PMC4, pmc_sample_period(sample_period));
    while (ebb_state.stats.ebb_count < 1000000) {
//
// We are trying to get the EBB exception to race exactly with
// us entering the kernel to do the syscall. We then need the
// kernel to decide our timeslice is up and context switch to
// the other thread. When we come back our EBB will have been
// lost and we'll spin in this while loop forever.
//
    for (i = 0; i < 100000; i++)
    sched_yield();
// Change the sample period slightly to try and hit the race
    if (sample_period >= (orig_period + 200))
    sample_period = orig_period;
    else
    sample_period++;
    if (sample_period > max_period)
    max_period = sample_period;
    }
    ebb_freeze_pmcs();
    ebb_global_disable();
    mtspr(SPRN_PMC4, 0xdead);
    dump_summary_ebb_state();
    dump_ebb_hw_state();
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
// We vary our sample period so we need extra fudge here
    FAIL_IF(!ebb_check_count(4, orig_period, 2 * (max_period - orig_period)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lost_exception() -> c_int {
    static int lost_exception(void)
    {
    return eat_cpu(test_body);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_harness_set_timeout(300);
    return test_harness(lost_exception, "lost_exception");
    }
