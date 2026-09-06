//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/multi_ebb_procs_test.c
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
// Test running multiple EBB using processes at once on a single CPU. They
// should all run happily without interfering with each other.
//
    static bool child_should_exit;
#[no_mangle]
unsafe extern "C" fn sigint_handler(signal: c_int) {
    static void sigint_handler(int signal)
    {
    child_should_exit = true;
    }
    struct sigaction sigint_action = {
    .sa_handler = sigint_handler,
    };
#[no_mangle]
unsafe extern "C" fn cycles_child() -> c_int {
    static int cycles_child(void)
    {
    struct event event;
    if (sigaction(SIGINT, &sigint_action, core::ptr::null_mut())) {
    perror("sigaction");
    return 1;
    }
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
    while (!child_should_exit) {
    FAIL_IF(core_busy_loop());
    FAIL_IF(ebb_check_mmcr0());
    }
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_summary_ebb_state();
    event_close(&event);
    FAIL_IF(ebb_state.stats.ebb_count == 0);
    return 0;
    }
pub const NR_CHILDREN: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn multi_ebb_procs() -> c_int {
    int multi_ebb_procs(void)
    {
    pid_t pids[NR_CHILDREN];
    int rc, i;
    SKIP_IF(!ebb_is_supported());
    FAIL_IF(bind_to_cpu(BIND_CPU_ANY) < 0);
    for (i = 0; i < NR_CHILDREN; i++) {
    pids[i] = fork();
    if (pids[i] == 0)
    exit(cycles_child());
    }
// Have them all run for "a while"
    sleep(10);
    rc = 0;
    for (i = 0; i < NR_CHILDREN; i++) {
// Tell them to stop
    kill(pids[i], SIGINT);
// And wait
    rc |= wait_for_child(pids[i]);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(multi_ebb_procs, "multi_ebb_procs");
    }
