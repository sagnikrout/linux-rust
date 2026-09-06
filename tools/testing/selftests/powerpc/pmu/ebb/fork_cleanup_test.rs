//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/fork_cleanup_test.c
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
// Test that a fork clears the PMU state of the child. eg. BESCR/EBBHR/EBBRR
// are cleared, and MMCR0_PMCC is reset, preventing the child from accessing
// the PMU.
//
    static struct event event;
#[no_mangle]
unsafe extern "C" fn child() -> c_int {
    static int child(void)
    {
// Even though we have EBE=0 we can still see the EBB regs
    FAIL_IF(mfspr(SPRN_BESCR) != 0);
    FAIL_IF(mfspr(SPRN_EBBHR) != 0);
    FAIL_IF(mfspr(SPRN_EBBRR) != 0);
    FAIL_IF(catch_sigill(write_pmc1));
// We can still read from the event, though it is on our parent
    FAIL_IF(event_read(&event));
    return 0;
    }
// Tests that fork clears EBB state
#[no_mangle]
pub unsafe extern "C" fn fork_cleanup() -> c_int {
    int fork_cleanup(void)
    {
    pid_t pid;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    FAIL_IF(event_open(&event));
    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&event));
    mtspr(SPRN_MMCR0, MMCR0_FC);
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
// Don't need to actually take any EBBs
    pid = fork();
    if (pid == 0)
    exit(child());
// Child does the actual testing
    FAIL_IF(wait_for_child(pid));
// After fork
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(fork_cleanup, "fork_cleanup");
    }
