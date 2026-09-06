//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/cycles_with_mmcr2_test.c
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
// Test of counting cycles while manipulating the user accessible bits in MMCR2.
//
// We use two values because the first freezes PMC1 and so we would get no EBBs
pub const MMCR2_EXPECTED_1: c_uint = 0x4020100804020000UL /* (FC1P|FC2P|FC3P|FC4P|FC5P|FC6P) */;
pub const MMCR2_EXPECTED_2: c_uint = 0x0020100804020000UL /* (     FC2P|FC3P|FC4P|FC5P|FC6P) */;
#[no_mangle]
pub unsafe extern "C" fn cycles_with_mmcr2() -> c_int {
    int cycles_with_mmcr2(void)
    {
    struct event event;
    uint64_t val, expected[2], actual;
    int i;
    bool bad_mmcr2;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open(&event));
    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&event));
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
// XXX Set of MMCR2 must be after enable
    expected[0] = MMCR2_EXPECTED_1;
    expected[1] = MMCR2_EXPECTED_2;
    i = 0;
    bad_mmcr2 = false;
    actual = 0;
// Make sure we loop until we take at least one EBB
    while ((ebb_state.stats.ebb_count < 20 && !bad_mmcr2) ||
    ebb_state.stats.ebb_count < 1)
    {
    mtspr(SPRN_MMCR2, expected[i % 2]);
    FAIL_IF(core_busy_loop());
    val = mfspr(SPRN_MMCR2);
    if (val != expected[i % 2]) {
    bad_mmcr2 = true;
    actual = val;
    }
    i++;
    }
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_ebb_state();
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
    if (bad_mmcr2)
    printf("Bad MMCR2 value seen is 0x%lx\n", actual);
    FAIL_IF(bad_mmcr2);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(cycles_with_mmcr2, "cycles_with_mmcr2");
    }
