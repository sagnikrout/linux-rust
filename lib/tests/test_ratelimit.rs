//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_ratelimit.c
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

// a simple boot-time regression test

    static DEFINE_RATELIMIT_STATE(testrl, TESTRL_INTERVAL, 3);

    KUNIT_ASSERT_EQ(test, ___ratelimit(&testrl, "test_ratelimit_smoke"), (expected))
#[no_mangle]
unsafe extern "C" fn test_ratelimit_smoke(test: *mut kunit) {
    static void test_ratelimit_smoke(struct kunit *test)
    {
// Check settings.
    KUNIT_ASSERT_GE(test, TESTRL_INTERVAL, 100);
// Test normal operation.
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);
    schedule_timeout_idle(TESTRL_INTERVAL / 2);
    test_ratelimited(test, false);
    schedule_timeout_idle(TESTRL_INTERVAL * 3 / 4);
    test_ratelimited(test, true);
    schedule_timeout_idle(2 * TESTRL_INTERVAL);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    schedule_timeout_idle(TESTRL_INTERVAL / 2 );
    test_ratelimited(test, true);
    schedule_timeout_idle(TESTRL_INTERVAL * 3 / 4);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);
// Test disabling.
    testrl.burst = 0;
    test_ratelimited(test, false);
    testrl.burst = 2;
    testrl.interval = 0;
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
// Testing re-enabling.
    testrl.interval = TESTRL_INTERVAL;
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);
    test_ratelimited(test, false);
    }
    static struct ratelimit_state stressrl = RATELIMIT_STATE_INIT_FLAGS("stressrl", HZ / 10, 3,
    RATELIMIT_MSG_ON_RELEASE);
    let mut stress_duration: static int = 2 * HZ;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stress_kthread {
    pub nattempts: c_ulong,
    pub nunlimited: c_ulong,
    pub nlimited: c_ulong,
    pub nmissed: c_ulong,
    pub tp: *mut task_struct,
}

#[no_mangle]
unsafe extern "C" fn test_ratelimit_stress_child(arg: *mut c_void) -> c_int {
    static int test_ratelimit_stress_child(void *arg)
    {
    struct stress_kthread *sktp = arg;
    set_user_nice(current, MAX_NICE);
    while (!kthread_should_stop()) {
    sktp.nattempts++;
    if (___ratelimit(&stressrl, __func__))
    sktp.nunlimited++;
    else
    sktp.nlimited++;
    cond_resched();
    }
    sktp.nmissed = ratelimit_state_reset_miss(&stressrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_ratelimit_stress(test: *mut kunit) {
    static void test_ratelimit_stress(struct kunit *test)
    {
    int i;
    let mut n_stress_kthread: c_int = cpumask_weight(cpu_online_mask);
    let mut skt: stress_kthread = { 0 };
    struct stress_kthread *sktp = kzalloc_objs(*sktp, n_stress_kthread);
    let mut n_started: c_int = 0;
    KUNIT_ASSERT_NOT_NULL_MSG(test, sktp, "Memory allocation failure");
    for (i = 0; i < n_stress_kthread; i++) {
    sktp[i].tp = kthread_run(test_ratelimit_stress_child, &sktp[i], "%s/%i",
    "test_ratelimit_stress_child", i);
    if (IS_ERR(sktp[i].tp)) {
    KUNIT_FAIL(test, "kthread_run failed: %ld", PTR_ERR(sktp[i].tp));
    goto out_stop;
    }
    n_started++;
    pr_alert("Spawned test_ratelimit_stress_child %d\n", i);
    }
    schedule_timeout_idle(stress_duration);
    out_stop:
    for (i = 0; i < n_started; i++) {
    kthread_stop(sktp[i].tp);
    skt.nattempts += sktp[i].nattempts;
    skt.nunlimited += sktp[i].nunlimited;
    skt.nlimited += sktp[i].nlimited;
    skt.nmissed += sktp[i].nmissed;
    }
    if (n_started == n_stress_kthread) {
    KUNIT_ASSERT_EQ_MSG(test, skt.nunlimited + skt.nlimited, skt.nattempts,
    "Outcomes not equal to attempts");
    KUNIT_ASSERT_EQ_MSG(test, skt.nlimited, skt.nmissed,
    "Misses not equal to limits");
    }
    kfree(sktp);
    }
    static struct kunit_case ratelimit_test_cases[] = {
    KUNIT_CASE_SLOW(test_ratelimit_smoke),
    KUNIT_CASE_SLOW(test_ratelimit_stress),
    {}
    };
    static struct kunit_suite ratelimit_test_suite = {
    .name = "lib_ratelimit",
    .test_cases = ratelimit_test_cases,
    };
    kunit_test_suites(&ratelimit_test_suite);
    MODULE_DESCRIPTION("___ratelimit() KUnit test suite");
    MODULE_LICENSE("GPL");
