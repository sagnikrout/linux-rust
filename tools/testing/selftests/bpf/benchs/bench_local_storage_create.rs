//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_local_storage_create.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread {
    pub fds: *mut c_int,
    pub pthds: *mut pthread_t,
    pub pthd_results: *mut c_int,
}

    static struct bench_local_storage_create *skel;
    static struct thread *threads;
    static long create_owner_errs;
    let mut storage_type: static int = BPF_MAP_TYPE_SK_STORAGE;
    let mut batch_sz: static int = 32;
    enum {
    ARG_BATCH_SZ = 9000,
    ARG_STORAGE_TYPE = 9001,
    };
    static const struct argp_option opts[] = {
    { "batch-size", ARG_BATCH_SZ, "BATCH_SIZE", 0,
    "The number of storage creations in each batch" },
    { "storage-type", ARG_STORAGE_TYPE, "STORAGE_TYPE", 0,
    "The type of local storage to test (socket or task)" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t parse_arg(int key, char *arg, struct argp_state *state)
    {
    int ret;
    switch (key) {
    case ARG_BATCH_SZ:
    ret = atoi(arg);
    if (ret < 1) {
    fprintf(stderr, "invalid batch-size\n");
    argp_usage(state);
    }
    batch_sz = ret;
    break;
    case ARG_STORAGE_TYPE:
    if (!strcmp(arg, "task")) {
    storage_type = BPF_MAP_TYPE_TASK_STORAGE;
    } else if (!strcmp(arg, "socket")) {
    storage_type = BPF_MAP_TYPE_SK_STORAGE;
    } else {
    fprintf(stderr, "invalid storage-type (socket or task)\n");
    argp_usage(state);
    }
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
    const struct argp bench_local_storage_create_argp = {
    .options = opts,
    .parser = parse_arg,
    };
#[no_mangle]
unsafe extern "C" fn validate() {
    static void validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr,
    "local-storage-create benchmark does not need consumer\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn setup() {
    static void setup(void)
    {
    int i;
    skel = bench_local_storage_create__open_and_load();
    if (!skel) {
    fprintf(stderr, "error loading skel\n");
    exit(1);
    }
    skel.bss.bench_pid = getpid();
    if (storage_type == BPF_MAP_TYPE_SK_STORAGE) {
    if (!bpf_program__attach(skel.progs.socket_post_create)) {
    fprintf(stderr, "Error attaching bpf program\n");
    exit(1);
    }
    } else {
    if (!bpf_program__attach(skel.progs.sched_process_fork)) {
    fprintf(stderr, "Error attaching bpf program\n");
    exit(1);
    }
    }
    threads = calloc(env.producer_cnt, sizeof(*threads));
    if (!threads) {
    fprintf(stderr, "cannot alloc thread_res\n");
    exit(1);
    }
    for (i = 0; i < env.producer_cnt; i++) {
    struct thread *t = &threads[i];
    if (storage_type == BPF_MAP_TYPE_SK_STORAGE) {
    t.fds = malloc(batch_sz * sizeof(*t.fds));
    if (!t.fds) {
    fprintf(stderr, "cannot alloc t.fds\n");
    exit(1);
    }
    } else {
    t.pthds = malloc(batch_sz * sizeof(*t.pthds));
    if (!t.pthds) {
    fprintf(stderr, "cannot alloc t.pthds\n");
    exit(1);
    }
    t.pthd_results = malloc(batch_sz * sizeof(*t.pthd_results));
    if (!t.pthd_results) {
    fprintf(stderr, "cannot alloc t.pthd_results\n");
    exit(1);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    res.hits = atomic_swap(&skel.bss.create_cnts, 0);
    }
    static void *sk_producer(void *input)
    {
    struct thread *t = &threads[(long)(input)];
    int *fds = t.fds;
    int i;
    while (true) {
    for (i = 0; i < batch_sz; i++) {
    fds[i] = socket(AF_INET6, SOCK_DGRAM, 0);
    if (fds[i] == -1)
    atomic_inc(&create_owner_errs);
    }
    for (i = 0; i < batch_sz; i++) {
    if (fds[i] != -1)
    close(fds[i]);
    }
    }
    return core::ptr::null_mut();
    }
    static void *thread_func(void *arg)
    {
    return core::ptr::null_mut();
    }
    static void *task_producer(void *input)
    {
    struct thread *t = &threads[(long)(input)];
    pthread_t *pthds = t.pthds;
    int *pthd_results = t.pthd_results;
    int i;
    while (true) {
    for (i = 0; i < batch_sz; i++) {
    pthd_results[i] = pthread_create(&pthds[i], core::ptr::null_mut(), thread_func, core::ptr::null_mut());
    if (pthd_results[i])
    atomic_inc(&create_owner_errs);
    }
    for (i = 0; i < batch_sz; i++) {
    if (!pthd_results[i])
    pthread_join(pthds[i], core::ptr::null_mut());
    }
    }
    return core::ptr::null_mut();
    }
    static void *producer(void *input)
    {
    if (storage_type == BPF_MAP_TYPE_SK_STORAGE)
    return sk_producer(input);
    else
    return task_producer(input);
    }
#[no_mangle]
unsafe extern "C" fn report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    static void report_progress(int iter, struct bench_res *res, long delta_ns)
    {
    double creates_per_sec;
    creates_per_sec = res.hits / 1000.0 / (delta_ns / 1000000000.0);
    printf("Iter %3d (%7.3lfus): ",
    iter, (delta_ns - 1000000000) / 1000.0);
    printf("creates %8.3lfk/s (%7.3lfk/prod)\n",
    creates_per_sec, creates_per_sec / env.producer_cnt);
    }
#[no_mangle]
unsafe extern "C" fn report_final(res[]: bench_res, res_cnt: c_int) {
    static void report_final(struct bench_res res[], int res_cnt)
    {
    let mut creates_mean: double = 0.0, creates_stddev = 0.0;
    let mut total_creates: c_long = 0;
    int i;
    for (i = 0; i < res_cnt; i++) {
    creates_mean += res[i].hits / 1000.0 / (0.0 + res_cnt);
    total_creates += res[i].hits;
    }
    if (res_cnt > 1)  {
    for (i = 0; i < res_cnt; i++)
    creates_stddev += (creates_mean - res[i].hits / 1000.0) *
    (creates_mean - res[i].hits / 1000.0) /
    (res_cnt - 1.0);
    creates_stddev = sqrt(creates_stddev);
    }
    printf("Summary: creates %8.3lf \u00B1 %5.3lfk/s (%7.3lfk/prod), %ld total\n",
    creates_mean, creates_stddev, creates_mean / env.producer_cnt,
    total_creates);
    if (create_owner_errs || skel.bss.create_errs)
    printf("%s() errors %ld create_errs %ld\n",
    storage_type == BPF_MAP_TYPE_SK_STORAGE ?
    "socket" : "pthread_create",
    create_owner_errs,
    skel.bss.create_errs);
    }
// Benchmark performance of creating bpf local storage
    const struct bench bench_local_storage_create = {
    .name = "local-storage-create",
    .argp = &bench_local_storage_create_argp,
    .validate = validate,
    .setup = setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = report_progress,
    .report_final = report_final,
    };
