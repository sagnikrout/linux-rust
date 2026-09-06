//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/back_to_back_ebbs_test.c
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

pub const NUMBER_OF_EBBS: c_int = 50;
//
// Test that if we overflow the counter while in the EBB handler, we take
// another EBB on exiting from the handler.
//
// We do this by counting with a stupidly low sample period, causing us to
// overflow the PMU while we're still in the EBB handler, leading to another
// EBB.
//
// We get out of what would otherwise be an infinite loop by leaving the
// counter frozen once we've taken enough EBBs.
//
#[no_mangle]
unsafe extern "C" fn ebb_callee() {
    static void ebb_callee(void)
    {
    uint64_t siar, val;
    val = mfspr(SPRN_BESCR);
    if (!(val & BESCR_PMEO)) {
    ebb_state.stats.spurious++;
    goto out;
    }
    ebb_state.stats.ebb_count++;
    trace_log_counter(ebb_state.trace, ebb_state.stats.ebb_count);
// Resets the PMC
    count_pmc(1, sample_period);
    out:
    if (ebb_state.stats.ebb_count == NUMBER_OF_EBBS)
// Reset but leave counters frozen
    reset_ebb_with_clear_mask(MMCR0_PMAO);
    else
// Unfreezes
    reset_ebb();
// Do some stuff to chew some cycles and pop the counter
    siar = mfspr(SPRN_SIAR);
    trace_log_reg(ebb_state.trace, SPRN_SIAR, siar);
    val = mfspr(SPRN_PMC1);
    trace_log_reg(ebb_state.trace, SPRN_PMC1, val);
    val = mfspr(SPRN_MMCR0);
    trace_log_reg(ebb_state.trace, SPRN_MMCR0, val);
    }
#[no_mangle]
pub unsafe extern "C" fn back_to_back_ebbs() -> c_int {
    int back_to_back_ebbs(void)
    {
    struct event event;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open(&event));
    setup_ebb_handler(ebb_callee);
    FAIL_IF(ebb_event_enable(&event));
    sample_period = 5;
    ebb_freeze_pmcs();
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    ebb_global_enable();
    ebb_unfreeze_pmcs();
    while (ebb_state.stats.ebb_count < NUMBER_OF_EBBS)
    FAIL_IF(core_busy_loop());
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_ebb_state();
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count != NUMBER_OF_EBBS);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(back_to_back_ebbs, "back_to_back_ebbs");
    }
