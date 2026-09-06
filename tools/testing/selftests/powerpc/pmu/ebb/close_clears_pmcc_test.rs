//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/close_clears_pmcc_test.c
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
// Test that closing the EBB event clears MMCR0_PMCC, preventing further access
// by userspace to the PMU hardware.
//
#[no_mangle]
pub unsafe extern "C" fn close_clears_pmcc() -> c_int {
    int close_clears_pmcc(void)
    {
    struct event event;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    FAIL_IF(event_open(&event));
    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&event));
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    while (ebb_state.stats.ebb_count < 1)
    FAIL_IF(core_busy_loop());
    ebb_global_disable();
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
// The real test is here, do we take a SIGILL when writing PMU regs now
// that we have closed the event. We expect that we will.
    FAIL_IF(catch_sigill(write_pmc1));
// We should still be able to read EBB regs though
    mfspr(SPRN_EBBHR);
    mfspr(SPRN_EBBRR);
    mfspr(SPRN_BESCR);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(close_clears_pmcc, "close_clears_pmcc");
    }
