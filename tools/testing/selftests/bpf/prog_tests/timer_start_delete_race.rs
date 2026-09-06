//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/timer_start_delete_race.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Macro flag: #define _GNU_SOURCE

//
// Test for race between bpf_timer_start() and map element deletion.
//
// The race scenario:
// - CPU 1: bpf_timer_start() proceeds to bpf_async_process() and is about
// to call hrtimer_start() but hasn't yet
// - CPU 2: map_delete_elem() calls __bpf_async_cancel_and_free(), since
// timer is not scheduled yet hrtimer_try_to_cancel() is a nop,
// then calls bpf_async_refcount_put() dropping refcnt to zero
// and scheduling call_rcu_tasks_trace()
// - CPU 1: continues and calls hrtimer_start()
// - After RCU tasks trace grace period: memory is freed
// - Timer callback fires on freed memory: UAF!
//
// This test stresses this race by having two threads:
// - Thread 1: repeatedly starts timers
// - Thread 2: repeatedly deletes map elements
//
// KASAN should detect use-after-free.
//
pub const ITERATIONS: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctx {
    pub skel: *mut timer_start_delete_race,
    pub start: volatile bool,
    pub stop: volatile bool,
    pub errors: c_int,
}

    static void *start_timer_thread(void *arg)
    {
    struct ctx *ctx = arg;
    cpu_set_t cpuset;
    int fd, i;
    CPU_ZERO(&cpuset);
    CPU_SET(0, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
    while (!ctx.start && !ctx.stop)
    usleep(1);
    if (ctx.stop)
    return core::ptr::null_mut();
    fd = bpf_program__fd(ctx.skel.progs.start_timer);
    for (i = 0; i < ITERATIONS && !ctx.stop; i++) {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    int err;
    err = bpf_prog_test_run_opts(fd, &opts);
    if (err || opts.retval) {
    ctx.errors++;
    break;
    }
    }
    return core::ptr::null_mut();
    }
    static void *delete_elem_thread(void *arg)
    {
    struct ctx *ctx = arg;
    cpu_set_t cpuset;
    int fd, i;
    CPU_ZERO(&cpuset);
    CPU_SET(1, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
    while (!ctx.start && !ctx.stop)
    usleep(1);
    if (ctx.stop)
    return core::ptr::null_mut();
    fd = bpf_program__fd(ctx.skel.progs.delete_elem);
    for (i = 0; i < ITERATIONS && !ctx.stop; i++) {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    int err;
    err = bpf_prog_test_run_opts(fd, &opts);
    if (err || opts.retval) {
    ctx.errors++;
    break;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_timer_start_delete_race() {
    void test_timer_start_delete_race(void)
    {
    struct timer_start_delete_race *skel;
    pthread_t threads[2];
    let mut ctx: ctx = {};
    int err;
    skel = timer_start_delete_race__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    ctx.skel = skel;
    err = pthread_create(&threads[0], core::ptr::null_mut(), start_timer_thread, &ctx);
    if (!ASSERT_OK(err, "create start_timer_thread")) {
    ctx.stop = true;
    goto cleanup;
    }
    err = pthread_create(&threads[1], core::ptr::null_mut(), delete_elem_thread, &ctx);
    if (!ASSERT_OK(err, "create delete_elem_thread")) {
    ctx.stop = true;
    pthread_join(threads[0], core::ptr::null_mut());
    goto cleanup;
    }
    ctx.start = true;
    pthread_join(threads[0], core::ptr::null_mut());
    pthread_join(threads[1], core::ptr::null_mut());
    ASSERT_EQ(ctx.errors, 0, "thread_errors");
// Either KASAN will catch UAF or kernel will crash or nothing happens
    cleanup:
    timer_start_delete_race__destroy(skel);
    }
