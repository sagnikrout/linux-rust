//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_rename.c
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
// Copyright (c) 2020 Facebook

// BPF triggering benchmarks
    static struct ctx {
    struct test_overhead *skel;
    struct counter hits;
    int fd;
    } ctx;
#[no_mangle]
unsafe extern "C" fn validate() {
    static void validate(void)
    {
    if (env.producer_cnt != 1) {
    fprintf(stderr, "benchmark doesn't support multi-producer!\n");
    exit(1);
    }
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "benchmark doesn't support consumer!\n");
    exit(1);
    }
    }
    static void *producer(void *input)
    {
    char buf[] = "test_overhead";
    int err;
    while (true) {
    err = write(ctx.fd, buf, sizeof(buf));
    if (err < 0) {
    fprintf(stderr, "write failed\n");
    exit(1);
    }
    atomic_inc(&ctx.hits.value);
    }
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    res.hits = atomic_swap(&ctx.hits.value, 0);
    }
#[no_mangle]
unsafe extern "C" fn setup_ctx() {
    static void setup_ctx(void)
    {
    setup_libbpf();
    ctx.skel = test_overhead__open_and_load();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    ctx.fd = open("/proc/self/comm", O_WRONLY|O_TRUNC);
    if (ctx.fd < 0) {
    fprintf(stderr, "failed to open /proc/self/comm: %d\n", -errno);
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn attach_bpf(prog: *mut bpf_program) {
    static void attach_bpf(struct bpf_program *prog)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_base() {
    static void setup_base(void)
    {
    setup_ctx();
    }
#[no_mangle]
unsafe extern "C" fn setup_kprobe() {
    static void setup_kprobe(void)
    {
    setup_ctx();
    attach_bpf(ctx.skel.progs.prog1);
    }
#[no_mangle]
unsafe extern "C" fn setup_kretprobe() {
    static void setup_kretprobe(void)
    {
    setup_ctx();
    attach_bpf(ctx.skel.progs.prog2);
    }
#[no_mangle]
unsafe extern "C" fn setup_rawtp() {
    static void setup_rawtp(void)
    {
    setup_ctx();
    attach_bpf(ctx.skel.progs.prog3);
    }
#[no_mangle]
unsafe extern "C" fn setup_fentry() {
    static void setup_fentry(void)
    {
    setup_ctx();
    attach_bpf(ctx.skel.progs.prog4);
    }
#[no_mangle]
unsafe extern "C" fn setup_fexit() {
    static void setup_fexit(void)
    {
    setup_ctx();
    attach_bpf(ctx.skel.progs.prog5);
    }
    const struct bench bench_rename_base = {
    .name = "rename-base",
    .validate = validate,
    .setup = setup_base,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_rename_kprobe = {
    .name = "rename-kprobe",
    .validate = validate,
    .setup = setup_kprobe,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_rename_kretprobe = {
    .name = "rename-kretprobe",
    .validate = validate,
    .setup = setup_kretprobe,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_rename_rawtp = {
    .name = "rename-rawtp",
    .validate = validate,
    .setup = setup_rawtp,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_rename_fentry = {
    .name = "rename-fentry",
    .validate = validate,
    .setup = setup_fentry,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_rename_fexit = {
    .name = "rename-fexit",
    .validate = validate,
    .setup = setup_fexit,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
