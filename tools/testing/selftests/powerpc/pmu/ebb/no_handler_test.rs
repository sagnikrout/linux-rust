//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/no_handler_test.c
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

// Test that things work sanely if we have no handler
#[no_mangle]
unsafe extern "C" fn no_handler_test() -> c_int {
    static int no_handler_test(void)
    {
    struct event event;
    u64 val;
    int i;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open(&event));
    FAIL_IF(ebb_event_enable(&event));
    val = mfspr(SPRN_EBBHR);
    FAIL_IF(val != 0);
// Make sure it overflows quickly
    sample_period = 1000;
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
// Spin to make sure the event has time to overflow
    for (i = 0; i < 1000; i++)
    mb();
    dump_ebb_state();
// We expect to see the PMU frozen & PMAO set
    val = mfspr(SPRN_MMCR0);
    FAIL_IF(val != 0x0000000080000080);
    event_close(&event);
// The real test is that we never took an EBB at 0x0
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(no_handler_test,"no_handler_test");
    }
