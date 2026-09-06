//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/cycles_with_freeze_test.c
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
// Test of counting cycles while using MMCR0_FC (freeze counters) to only count
// parts of the code. This is complicated by the fact that FC is set by the
// hardware when the event overflows. We may take the EBB after we have set FC,
// so we have to be careful about whether we clear FC at the end of the EBB
// handler or not.
//
    let mut counters_frozen: static bool = false;
    let mut ebbs_while_frozen: static int = 0;
#[no_mangle]
unsafe extern "C" fn ebb_callee() {
    static void ebb_callee(void)
    {
    uint64_t mask, val;
    mask = MMCR0_PMAO | MMCR0_FC;
    val = mfspr(SPRN_BESCR);
    if (!(val & BESCR_PMEO)) {
    ebb_state.stats.spurious++;
    goto out;
    }
    ebb_state.stats.ebb_count++;
    trace_log_counter(ebb_state.trace, ebb_state.stats.ebb_count);
    val = mfspr(SPRN_MMCR0);
    trace_log_reg(ebb_state.trace, SPRN_MMCR0, val);
    if (counters_frozen) {
    trace_log_string(ebb_state.trace, "frozen");
    ebbs_while_frozen++;
    mask &= ~MMCR0_FC;
    }
    count_pmc(1, sample_period);
    out:
    reset_ebb_with_clear_mask(mask);
    }
#[no_mangle]
pub unsafe extern "C" fn cycles_with_freeze() -> c_int {
    int cycles_with_freeze(void)
    {
    struct event event;
    uint64_t val;
    bool fc_cleared;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open(&event));
    setup_ebb_handler(ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&event));
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    fc_cleared = false;
// Make sure we loop until we take at least one EBB
    while ((ebb_state.stats.ebb_count < 20 && !fc_cleared) ||
    ebb_state.stats.ebb_count < 1)
    {
    counters_frozen = false;
    mb();
    mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & ~MMCR0_FC);
    FAIL_IF(core_busy_loop());
    counters_frozen = true;
    mb();
    mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) |  MMCR0_FC);
    val = mfspr(SPRN_MMCR0);
    if (! (val & MMCR0_FC)) {
    printf("Outside of loop, FC NOT set MMCR0 0x%lx\n", val);
    fc_cleared = true;
    }
    }
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_ebb_state();
    printf("EBBs while frozen %d\n", ebbs_while_frozen);
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
    FAIL_IF(fc_cleared);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(cycles_with_freeze, "cycles_with_freeze");
    }
