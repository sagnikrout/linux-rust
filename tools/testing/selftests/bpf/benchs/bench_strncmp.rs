//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_strncmp.c
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
// Copyright (C) 2021. Huawei Technologies Co., Ltd

    static struct strncmp_ctx {
    struct strncmp_bench *skel;
    } ctx;
    static struct strncmp_args {
    u32 cmp_str_len;
    } args = {
    .cmp_str_len = 32,
    };
    enum {
    ARG_CMP_STR_LEN = 5000,
    };
    static const struct argp_option opts[] = {
    { "cmp-str-len", ARG_CMP_STR_LEN, "CMP_STR_LEN", 0,
    "Set the length of compared string" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn strncmp_parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t strncmp_parse_arg(int key, char *arg, struct argp_state *state)
    {
    switch (key) {
    case ARG_CMP_STR_LEN:
    args.cmp_str_len = strtoul(arg, core::ptr::null_mut(), 10);
    if (!args.cmp_str_len ||
    args.cmp_str_len >= sizeof(ctx.skel.bss.str)) {
    fprintf(stderr, "Invalid cmp str len (limit %zu)\n",
    sizeof(ctx.skel.bss.str));
    argp_usage(state);
    }
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
    const struct argp bench_strncmp_argp = {
    .options = opts,
    .parser = strncmp_parse_arg,
    };
#[no_mangle]
unsafe extern "C" fn strncmp_validate() {
    static void strncmp_validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "strncmp benchmark doesn't support consumer!\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn strncmp_setup() {
    static void strncmp_setup(void)
    {
    int err;
    char *target;
    size_t i, sz;
    sz = sizeof(ctx.skel.rodata.target);
    if (!sz || sz < sizeof(ctx.skel.bss.str)) {
    fprintf(stderr, "invalid string size (target %zu, src %zu)\n",
    sz, sizeof(ctx.skel.bss.str));
    exit(1);
    }
    setup_libbpf();
    ctx.skel = strncmp_bench__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    srandom(time(core::ptr::null_mut()));
    target = ctx.skel.rodata.target;
    for (i = 0; i < sz - 1; i++)
    target[i] = '1' + random() % 9;
    target[sz - 1] = '\0';
    ctx.skel.rodata.cmp_str_len = args.cmp_str_len;
    memcpy(ctx.skel.bss.str, target, args.cmp_str_len);
    ctx.skel.bss.str[args.cmp_str_len] = '\0';
// Make bss->str < rodata->target
    ctx.skel.bss.str[args.cmp_str_len - 1] -= 1;
    err = strncmp_bench__load(ctx.skel);
    if (err) {
    fprintf(stderr, "failed to load skeleton\n");
    strncmp_bench__destroy(ctx.skel);
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn strncmp_attach_prog(prog: *mut bpf_program) {
    static void strncmp_attach_prog(struct bpf_program *prog)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn strncmp_no_helper_setup() {
    static void strncmp_no_helper_setup(void)
    {
    strncmp_setup();
    strncmp_attach_prog(ctx.skel.progs.strncmp_no_helper);
    }
#[no_mangle]
unsafe extern "C" fn strncmp_helper_setup() {
    static void strncmp_helper_setup(void)
    {
    strncmp_setup();
    strncmp_attach_prog(ctx.skel.progs.strncmp_helper);
    }
    static void *strncmp_producer(void *ctx)
    {
    while (true)
    (void)syscall(__NR_getpgid);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn strncmp_measure(res: *mut bench_res) {
    static void strncmp_measure(struct bench_res *res)
    {
    res.hits = atomic_swap(&ctx.skel.bss.hits, 0);
    }
    const struct bench bench_strncmp_no_helper = {
    .name = "strncmp-no-helper",
    .argp = &bench_strncmp_argp,
    .validate = strncmp_validate,
    .setup = strncmp_no_helper_setup,
    .producer_thread = strncmp_producer,
    .measure = strncmp_measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_strncmp_helper = {
    .name = "strncmp-helper",
    .argp = &bench_strncmp_argp,
    .validate = strncmp_validate,
    .setup = strncmp_helper_setup,
    .producer_thread = strncmp_producer,
    .measure = strncmp_measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
