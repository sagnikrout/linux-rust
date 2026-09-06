//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/multi_counter_test.c
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
// Test counting multiple events using EBBs.
//
#[no_mangle]
pub unsafe extern "C" fn multi_counter() -> c_int {
    int multi_counter(void)
    {
    struct event events[6];
    int i, group_fd;
    SKIP_IF(!ebb_is_supported());
    event_init_named(&events[0], 0x1001C, "PM_CMPLU_STALL_THRD");
    event_init_named(&events[1], 0x2D016, "PM_CMPLU_STALL_FXU");
    event_init_named(&events[2], 0x30006, "PM_CMPLU_STALL_OTHER_CMPL");
    event_init_named(&events[3], 0x4000A, "PM_CMPLU_STALL");
    event_init_named(&events[4], 0x600f4, "PM_RUN_CYC");
    event_init_named(&events[5], 0x500fa, "PM_RUN_INST_CMPL");
    event_leader_ebb_init(&events[0]);
    for (i = 1; i < 6; i++)
    event_ebb_init(&events[i]);
    group_fd = -1;
    for (i = 0; i < 6; i++) {
    events[i].attr.exclude_kernel = 1;
    events[i].attr.exclude_hv = 1;
    events[i].attr.exclude_idle = 1;
    FAIL_IF(event_open_with_group(&events[i], group_fd));
    if (group_fd == -1)
    group_fd = events[0].fd;
    }
    ebb_enable_pmc_counting(1);
    ebb_enable_pmc_counting(2);
    ebb_enable_pmc_counting(3);
    ebb_enable_pmc_counting(4);
    ebb_enable_pmc_counting(5);
    ebb_enable_pmc_counting(6);
    setup_ebb_handler(standard_ebb_callee);
    FAIL_IF(ioctl(events[0].fd, PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP));
    FAIL_IF(event_read(&events[0]));
    ebb_global_enable();
    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    mtspr(SPRN_PMC2, pmc_sample_period(sample_period));
    mtspr(SPRN_PMC3, pmc_sample_period(sample_period));
    mtspr(SPRN_PMC4, pmc_sample_period(sample_period));
    mtspr(SPRN_PMC5, pmc_sample_period(sample_period));
    mtspr(SPRN_PMC6, pmc_sample_period(sample_period));
    while (ebb_state.stats.ebb_count < 50) {
    FAIL_IF(core_busy_loop());
    FAIL_IF(ebb_check_mmcr0());
    }
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_ebb_state();
    for (i = 0; i < 6; i++)
    event_close(&events[i]);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(multi_counter, "multi_counter");
    }
