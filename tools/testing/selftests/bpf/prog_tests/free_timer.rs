//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/free_timer.c
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
// Copyright (C) 2025. Huawei Technologies Co., Ltd
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct run_ctx {
    pub start_prog: *mut bpf_program,
    pub overwrite_prog: *mut bpf_program,
    pub notify: pthread_barrier_t,
    pub loop: c_int,
    pub start: bool,
    pub stop: bool,
}

#[no_mangle]
unsafe extern "C" fn start_threads(ctx: *mut run_ctx) {
    static void start_threads(struct run_ctx *ctx)
    {
    ctx.start = true;
    }
#[no_mangle]
unsafe extern "C" fn stop_threads(ctx: *mut run_ctx) {
    static void stop_threads(struct run_ctx *ctx)
    {
    ctx.stop = true;
// Guarantee the order between ->stop and ->start
    __atomic_store_n(&ctx.start, true, __ATOMIC_RELEASE);
    }
#[no_mangle]
unsafe extern "C" fn wait_for_start(ctx: *mut run_ctx) -> c_int {
    static int wait_for_start(struct run_ctx *ctx)
    {
    while (!__atomic_load_n(&ctx.start, __ATOMIC_ACQUIRE))
    usleep(10);
    return ctx.stop;
    }
    static void *overwrite_timer_fn(void *arg)
    {
    struct run_ctx *ctx = arg;
    int loop, fd, err;
    cpu_set_t cpuset;
    let mut ret: c_long = 0;
// Pin on CPU 0
    CPU_ZERO(&cpuset);
    CPU_SET(0, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
// Is the thread being stopped ?
    err = wait_for_start(ctx);
    if (err)
    return core::ptr::null_mut();
    fd = bpf_program__fd(ctx.overwrite_prog);
    loop = ctx.loop;
    while (loop-- > 0) {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
// Wait for start thread to complete
    pthread_barrier_wait(&ctx.notify);
// Overwrite timers
    err = bpf_prog_test_run_opts(fd, &opts);
    if (err)
    ret |= 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: opts.retval) -> else {
    else if (opts.retval)
    ret |= 2;
// Notify start thread to start timers
    pthread_barrier_wait(&ctx.notify);
    }
    return (void *)ret;
    }
    static void *start_timer_fn(void *arg)
    {
    struct run_ctx *ctx = arg;
    int loop, fd, err;
    cpu_set_t cpuset;
    let mut ret: c_long = 0;
// Pin on CPU 1
    CPU_ZERO(&cpuset);
    CPU_SET(1, &cpuset);
    pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset);
// Is the thread being stopped ?
    err = wait_for_start(ctx);
    if (err)
    return core::ptr::null_mut();
    fd = bpf_program__fd(ctx.start_prog);
    loop = ctx.loop;
    while (loop-- > 0) {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
// Run the prog to start timer
    err = bpf_prog_test_run_opts(fd, &opts);
    if (err)
    ret |= 4;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: opts.retval) -> else {
    else if (opts.retval)
    ret |= 8;
// Notify overwrite thread to do overwrite
    pthread_barrier_wait(&ctx.notify);
// Wait for overwrite thread to complete
    pthread_barrier_wait(&ctx.notify);
    }
    return (void *)ret;
    }
#[no_mangle]
pub unsafe extern "C" fn test_free_timer() {
    void test_free_timer(void)
    {
    struct free_timer *skel;
    struct bpf_program *prog;
    struct run_ctx ctx;
    pthread_t tid[2];
    void *ret;
    int err;
    skel = free_timer__open_and_load();
    if (!skel && errno == EOPNOTSUPP) {
    test__skip();
    return;
    }
    if (!ASSERT_OK_PTR(skel, "open_load"))
    return;
    memset(&ctx, 0, sizeof(ctx));
    prog = bpf_object__find_program_by_name(skel.obj, "start_timer");
    if (!ASSERT_OK_PTR(prog, "find start prog"))
    goto out;
    ctx.start_prog = prog;
    prog = bpf_object__find_program_by_name(skel.obj, "overwrite_timer");
    if (!ASSERT_OK_PTR(prog, "find overwrite prog"))
    goto out;
    ctx.overwrite_prog = prog;
    pthread_barrier_init(&ctx.notify, core::ptr::null_mut(), 2);
    ctx.loop = 10;
    err = pthread_create(&tid[0], core::ptr::null_mut(), start_timer_fn, &ctx);
    if (!ASSERT_OK(err, "create start_timer"))
    goto out;
    err = pthread_create(&tid[1], core::ptr::null_mut(), overwrite_timer_fn, &ctx);
    if (!ASSERT_OK(err, "create overwrite_timer")) {
    stop_threads(&ctx);
    goto out;
    }
    start_threads(&ctx);
    ret = core::ptr::null_mut();
    err = pthread_join(tid[0], &ret);
    ASSERT_EQ(err | (long)ret, 0, "start_timer");
    ret = core::ptr::null_mut();
    err = pthread_join(tid[1], &ret);
    ASSERT_EQ(err | (long)ret, 0, "overwrite_timer");
    out:
    free_timer__destroy(skel);
    }
