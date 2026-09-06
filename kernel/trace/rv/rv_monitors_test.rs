//! Automatically rewritten from C to Rust
//! Source: kernel/trace/rv/rv_monitors_test.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2026-2029 Red Hat, Inc. Gabriele Monaco <gmonaco@redhat.com>
//
// RV monitor kunit tests:
// Tests the RV monitors by triggering fake events to verify monitor
// behavior and reactions. Tests start from the first defined event and
// trigger events in order to verify error detection.
//

//
// An easy way to pass the context is to use kunit_get_current_test()->priv,
// but this doesn't always work (e.g. a reactor running from another context
// like softirq). Store the current value here whenever a test is running.
//
    static struct rv_kunit_ctx *active_ctx;
    __printf(1, 0)
#[no_mangle]
unsafe extern "C" fn rv_kunit_mock_react(msg: *const c_char, args: va_list) {
    static void rv_kunit_mock_react(const char *msg, va_list args)
    {
    if (active_ctx)
    ++active_ctx.reactions;
    }
//
// teardown_test - Disable the monitor for a kunit test
//
// Since per-task monitors are special, make sure we reset all the ones we
// started manually here, if required.
//
#[no_mangle]
pub unsafe extern "C" fn teardown_test(arg: *mut c_void) {
    void teardown_test(void *arg)
    {
    const struct rv_kunit_mon *mon = arg;
    struct kunit *test = kunit_get_current_test();
    if (test) {
    struct rv_kunit_ctx *ctx = test.priv;
    RV_KUNIT_EXPECT_NO_REACTION(test, ctx);
    if (mon.is_per_task && mon.task_reset) {
    for (int i = 0; i < ctx.mock_task_count; i++)
    mon.task_reset(ctx.mock_tasks[i]);
    synchronize_rcu();
    }
    }
    mon.rv_this.enabled = 0;
    if (mon.rv_this.reactor)
    mon.rv_this.react = mon.rv_this.reactor.react;
    else
    mon.rv_this.react = core::ptr::null_mut();
    active_ctx = core::ptr::null_mut();
    rv_mock_current(core::ptr::null_mut());
    if (mon.is_per_task)
// mon->task_slot = RV_PER_TASK_MONITOR_INIT;
    else
    mon.monitor_destroy();
    }
//
// prepare_test - Enable the monitor for a kunit test
//
// Do the bare minimum to set up the monitor, per-task monitors are special as
// "real" initialisation/destruction iterates over real tasks, and may register
// handlers. All we need is to select the right slot in the task_struct.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_test(test: *mut kunit, mon: *const rv_kunit_mon) {
    void prepare_test(struct kunit *test, const struct rv_kunit_mon *mon)
    {
    KUNIT_ASSERT_FALSE(test, mon.rv_this.enabled);
    active_ctx = test.priv;
    mon.rv_this.react = rv_kunit_mock_react;
    if (mon.is_per_task)
// mon->task_slot = 0;
    else
    KUNIT_ASSERT_EQ(test, mon.monitor_init(), 0);
    mon.rv_this.enabled = 1;
    KUNIT_ASSERT_EQ(test, 0,
    kunit_add_action_or_reset(test, teardown_test, (void *)mon));
    }
    struct task_struct *rv_kunit_alloc_mock_task(struct kunit *test)
    {
    struct rv_kunit_ctx *ctx = test.priv;
    struct task_struct *tsk;
    KUNIT_ASSERT_LT(test, ctx.mock_task_count, RV_KUNIT_MAX_MOCK_TASKS);
    tsk = kunit_kzalloc(test, sizeof(struct task_struct), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tsk);
    if (!IS_ENABLED(CONFIG_THREAD_INFO_IN_TASK)) {
    tsk.stack = kunit_kzalloc(test, sizeof(struct thread_info), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tsk.stack);
    }
    ctx.mock_tasks[ctx.mock_task_count++] = tsk;
    return tsk;
    }
#[no_mangle]
unsafe extern "C" fn rv_mon_test_init(test: *mut kunit) -> c_int {
    static int rv_mon_test_init(struct kunit *test)
    {
    struct rv_kunit_ctx *ctx;
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    test.priv = ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rv_test_stub(test: *mut kunit) -> void __maybe_unused {
    static void __maybe_unused rv_test_stub(struct kunit *test)
    {
    kunit_skip(test, "Monitor not enabled\n");
    }
//
// rv_test_dummy - test reactions work as expected
//
#[no_mangle]
unsafe extern "C" fn rv_test_dummy(test: *mut kunit) {
    static void rv_test_dummy(struct kunit *test)
    {
    struct rv_kunit_ctx *ctx = test.priv;
    static struct rv_monitor dummy_monitor = {
    .name = "dummy",
    .react = rv_kunit_mock_react,
    };
    active_ctx = ctx;
    RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
    rv_react(&dummy_monitor, "dummy");
    RV_KUNIT_EXPECT_NO_REACTION(test, ctx);
    active_ctx = core::ptr::null_mut();
    }

    static struct kunit_case rv_mon_test_cases[] = {
    KUNIT_CASE(rv_test_dummy),
    KUNIT_CASE(rv_test_sco),
    KUNIT_CASE(rv_test_sssw),
    KUNIT_CASE(rv_test_sts),
    KUNIT_CASE(rv_test_opid),
    KUNIT_CASE(rv_test_nomiss),
    KUNIT_CASE(rv_test_pagefault),
    KUNIT_CASE(rv_test_sleep),
    {}
    };
    static struct kunit_suite rv_mon_test_suite = {
    .name = "rv_mon",
    .suite_init = rv_set_testing,
    .suite_exit = rv_clear_testing,
    .init = rv_mon_test_init,
    .test_cases = rv_mon_test_cases,
    };
    kunit_test_suites(&rv_mon_test_suite);
    MODULE_AUTHOR("Gabriele Monaco <gmonaco@redhat.com>");
    MODULE_DESCRIPTION("RV monitor kunit tests: test monitors by triggering reactions");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
