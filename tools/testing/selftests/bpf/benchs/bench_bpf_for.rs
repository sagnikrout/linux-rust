//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_bpf_for.c
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

// BPF triggering benchmarks
    static struct ctx {
    struct bpf_for_bench *skel;
    } ctx;
    static struct {
    __u32 nr_loops;
    } args = {
//
// Default to a large loop count so the per-iteration bpf_iter_num_next() cost dominates
// the one-time bpf_iter_num_new()/destroy() setup and teardown.
//
    .nr_loops = 1000,
    };
    enum {
    ARG_NR_LOOPS = 4000,
    };
    static const struct argp_option opts[] = {
    { "nr_loops", ARG_NR_LOOPS, "nr_loops", 0,
    "Set number of iterations for the bpf_for() loop"},
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t parse_arg(int key, char *arg, struct argp_state *state)
    {
    switch (key) {
    case ARG_NR_LOOPS:
    args.nr_loops = strtol(arg, core::ptr::null_mut(), 10);
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
// exported into benchmark runner
    const struct argp bench_bpf_for_argp = {
    .options = opts,
    .parser = parse_arg,
    };
#[no_mangle]
unsafe extern "C" fn validate() {
    static void validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "benchmark doesn't support consumer!\n");
    exit(1);
    }
    }
    static void *producer(void *input)
    {
    while (true)
// trigger the bpf program
    syscall(__NR_getpgid);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    res.hits = atomic_swap(&ctx.skel.bss.hits, 0);
    }
#[no_mangle]
unsafe extern "C" fn setup() {
    static void setup(void)
    {
    struct bpf_link *link;
    setup_libbpf();
    ctx.skel = bpf_for_bench__open_and_load();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    link = bpf_program__attach(ctx.skel.progs.benchmark);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
    ctx.skel.bss.nr_loops = args.nr_loops;
    }
    const struct bench bench_bpf_for = {
    .name = "bpf-for",
    .argp = &bench_bpf_for_argp,
    .validate = validate,
    .setup = setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = ops_report_progress,
    .report_final = ops_report_final,
    };
