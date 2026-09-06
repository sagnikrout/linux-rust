//! Automatically rewritten from C to Rust
//! Source: kernel/time/clocksource-wdtest.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Unit test for the clocksource watchdog.
//
// Copyright (C) 2021 Facebook, Inc.
// Copyright (C) 2026 Intel Corp.
//
// Author: Paul E. McKenney <paulmck@kernel.org>
// Author: Thomas Gleixner <tglx@kernel.org>
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Clocksource watchdog unit test");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@kernel.org>");
    MODULE_AUTHOR("Thomas Gleixner <tglx@kernel.org>");
    enum wdtest_states {
    WDTEST_INJECT_NONE,
    WDTEST_INJECT_DELAY,
    WDTEST_INJECT_POSITIVE,
    WDTEST_INJECT_NEGATIVE,
    WDTEST_INJECT_PERCPU	= 0x100,
    };
    static enum wdtest_states wdtest_state;
    static unsigned long wdtest_test_count;
    static ktime_t wdtest_last_ts, wdtest_offset;
pub const SHIFT_4000PPM: c_int = 8;
#[no_mangle]
unsafe extern "C" fn wdtest_get_offset(cs: *mut clocksource) -> ktime_t {
    static ktime_t wdtest_get_offset(struct clocksource *cs)
    {
    if (wdtest_state < WDTEST_INJECT_PERCPU)
    return wdtest_test_count & 0x1 ? 0 : wdtest_offset >> SHIFT_4000PPM;
// Only affect the readout of the "remote" CPU
    return cs.wd_cpu == smp_processor_id() ? 0 : NSEC_PER_MSEC;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_ktime_read(cs: *mut clocksource) -> u64 {
    static u64 wdtest_ktime_read(struct clocksource *cs)
    {
    let mut now: ktime_t = ktime_get_raw_fast_ns();
    let mut intv: ktime_t = now - wdtest_last_ts;
//
// Only increment the test counter once per watchdog interval and
// store the interval for the offset calculation of this step. This
// guarantees a consistent behaviour even if the other side needs
// to repeat due to a watchdog read timeout.
//
    if (intv > (NSEC_PER_SEC / 4)) {
    WRITE_ONCE(wdtest_test_count, wdtest_test_count + 1);
    wdtest_last_ts = now;
    wdtest_offset = intv;
    }
    switch (wdtest_state & ~WDTEST_INJECT_PERCPU) {
    case WDTEST_INJECT_POSITIVE:
    return now + wdtest_get_offset(cs);
    case WDTEST_INJECT_NEGATIVE:
    return now - wdtest_get_offset(cs);
    case WDTEST_INJECT_DELAY:
    udelay(500);
    return now;
    default:
    return now;
    }
    }

    CLOCK_SOURCE_CALIBRATED |		\
    CLOCK_SOURCE_MUST_VERIFY |		\
    CLOCK_SOURCE_WDTEST)
    static struct clocksource clocksource_wdtest_ktime = {
    .name			= "wdtest-ktime",
    .rating			= 10,
    .read			= wdtest_ktime_read,
    .mask			= CLOCKSOURCE_MASK(64),
    .flags			= KTIME_FLAGS,
    .list			= LIST_HEAD_INIT(clocksource_wdtest_ktime.list),
    };
#[no_mangle]
unsafe extern "C" fn wdtest_clocksource_reset(which: enum wdtest_states, percpu: bool) {
    static void wdtest_clocksource_reset(enum wdtest_states which, bool percpu)
    {
    clocksource_unregister(&clocksource_wdtest_ktime);
    pr_info("Test: State %d percpu %d\n", which, percpu);
    wdtest_state = which;
    if (percpu)
    wdtest_state |= WDTEST_INJECT_PERCPU;
    wdtest_test_count = 0;
    wdtest_last_ts = 0;
    clocksource_wdtest_ktime.rating = 10;
    clocksource_wdtest_ktime.flags = KTIME_FLAGS;
    if (percpu)
    clocksource_wdtest_ktime.flags |= CLOCK_SOURCE_WDTEST_PERCPU;
    clocksource_register_khz(&clocksource_wdtest_ktime, 1000 * 1000);
    }
    static bool wdtest_execute(enum wdtest_states which, bool percpu, unsigned int expect,
    unsigned long calls)
    {
    wdtest_clocksource_reset(which, percpu);
    for (; READ_ONCE(wdtest_test_count) < calls; msleep(100)) {
    let mut flags: c_uint = READ_ONCE(clocksource_wdtest_ktime.flags);
    if (kthread_should_stop())
    return false;
    if (flags & CLOCK_SOURCE_UNSTABLE) {
    if (expect & CLOCK_SOURCE_UNSTABLE)
    return true;
    pr_warn("Fail: Unexpected unstable\n");
    return false;
    }
    if (flags & CLOCK_SOURCE_VALID_FOR_HRES) {
    if (expect & CLOCK_SOURCE_VALID_FOR_HRES)
    return true;
    pr_warn("Fail: Unexpected valid for highres\n");
    return false;
    }
    }
    if (!expect)
    return true;
    pr_warn("Fail: Timed out\n");
    return false;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_run(percpu: bool) -> bool {
    static bool wdtest_run(bool percpu)
    {
    if (!wdtest_execute(WDTEST_INJECT_NONE, percpu, CLOCK_SOURCE_VALID_FOR_HRES, 8))
    return false;
    if (!wdtest_execute(WDTEST_INJECT_DELAY, percpu, 0, 4))
    return false;
    if (!wdtest_execute(WDTEST_INJECT_POSITIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8))
    return false;
    if (!wdtest_execute(WDTEST_INJECT_NEGATIVE, percpu, CLOCK_SOURCE_UNSTABLE, 8))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn wdtest_func(arg: *mut c_void) -> c_int {
    static int wdtest_func(void *arg)
    {
    clocksource_register_khz(&clocksource_wdtest_ktime, 1000 * 1000);
    if (wdtest_run(false)) {
    if (wdtest_run(true))
    pr_info("Success: All tests passed\n");
    }
    clocksource_unregister(&clocksource_wdtest_ktime);
    if (!IS_MODULE(CONFIG_TEST_CLOCKSOURCE_WATCHDOG))
    return 0;
    while (!kthread_should_stop())
    schedule_timeout_interruptible(3600 * HZ);
    return 0;
    }
    static struct task_struct *wdtest_thread;
#[no_mangle]
unsafe extern "C" fn clocksource_wdtest_init() -> int __init {
    static int __init clocksource_wdtest_init(void)
    {
    struct task_struct *t = kthread_run(wdtest_func, core::ptr::null_mut(), "wdtest");
    if (IS_ERR(t)) {
    pr_warn("Failed to create wdtest kthread.\n");
    return PTR_ERR(t);
    }
    wdtest_thread = t;
    return 0;
    }
    module_init(clocksource_wdtest_init);
#[no_mangle]
unsafe extern "C" fn clocksource_wdtest_cleanup() {
    static void clocksource_wdtest_cleanup(void)
    {
    if (wdtest_thread)
    kthread_stop(wdtest_thread);
    }
    module_exit(clocksource_wdtest_cleanup);
