//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_bpf_nop.c
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

    static struct ctx {
    struct bpf_nop_bench *skel;
    struct bpf_bench_timing timing;
    int prog_fd;
    } ctx;
#[no_mangle]
unsafe extern "C" fn nop_validate() {
    static void nop_validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "benchmark doesn't support consumers\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn nop_run_once(__always_unused: *mut *mut void unused) {
    static void nop_run_once(void *unused __always_unused)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    bpf_prog_test_run_opts(ctx.prog_fd, &topts);
    }
#[no_mangle]
unsafe extern "C" fn nop_setup() {
    static void nop_setup(void)
    {
    struct bpf_nop_bench *skel;
    int err;
    setup_libbpf();
    skel = bpf_nop_bench__open();
    if (!skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    err = bpf_nop_bench__load(skel);
    if (err) {
    fprintf(stderr, "failed to load skeleton: %s\n", strerror(-err));
    bpf_nop_bench__destroy(skel);
    exit(1);
    }
    ctx.skel = skel;
    ctx.prog_fd = bpf_program__fd(skel.progs.bench_nop);
    BENCH_TIMING_INIT(&ctx.timing, skel, 0);
    bpf_bench_calibrate(&ctx.timing, nop_run_once, core::ptr::null_mut());
    env.duration_sec = 600;
    }
    static void *nop_producer(void *input)
    {
    while (true)
    nop_run_once(core::ptr::null_mut());
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nop_measure(res: *mut bench_res) {
    static void nop_measure(struct bench_res *res)
    {
    bpf_bench_timing_measure(&ctx.timing, res);
    }
#[no_mangle]
unsafe extern "C" fn nop_report_final(res[]: bench_res, res_cnt: c_int) {
    static void nop_report_final(struct bench_res res[], int res_cnt)
    {
    bpf_bench_timing_report(&ctx.timing, "bpf-nop", core::ptr::null_mut());
    }
    const struct bench bench_bpf_nop = {
    .name		= "bpf-nop",
    .validate	= nop_validate,
    .setup		= nop_setup,
    .producer_thread = nop_producer,
    .measure	= nop_measure,
    .report_final	= nop_report_final,
    };
