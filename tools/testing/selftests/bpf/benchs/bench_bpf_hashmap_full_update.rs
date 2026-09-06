//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_bpf_hashmap_full_update.c
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
// Copyright (c) 2022 Bytedance

// BPF triggering benchmarks
    static struct ctx {
    struct bpf_hashmap_full_update_bench *skel;
    } ctx;
pub const MAX_LOOP_NUM: c_int = 10000;
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
    while (true) {
// trigger the bpf program
    syscall(__NR_getpgid);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    }
#[no_mangle]
unsafe extern "C" fn hashmap_full_update_setup(map_type: enum bpf_map_type) {
    static void hashmap_full_update_setup(enum bpf_map_type map_type)
    {
    struct bpf_link *link;
    int map_fd, i, max_entries;
    setup_libbpf();
    ctx.skel = bpf_hashmap_full_update_bench__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    bpf_map__set_type(ctx.skel.maps.hash_map_bench, map_type);
    if (map_type == BPF_MAP_TYPE_RHASH)
    bpf_map__set_map_flags(ctx.skel.maps.hash_map_bench,
    BPF_F_NO_PREALLOC);
    if (bpf_hashmap_full_update_bench__load(ctx.skel)) {
    fprintf(stderr, "failed to load skeleton\n");
    exit(1);
    }
    ctx.skel.bss.nr_loops = MAX_LOOP_NUM;
    link = bpf_program__attach(ctx.skel.progs.benchmark);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
// fill hash_map
    map_fd = bpf_map__fd(ctx.skel.maps.hash_map_bench);
    max_entries = bpf_map__max_entries(ctx.skel.maps.hash_map_bench);
    for (i = 0; i < max_entries; i++)
    bpf_map_update_elem(map_fd, &i, &i, BPF_ANY);
    }
#[no_mangle]
unsafe extern "C" fn setup() {
    static void setup(void)
    {
    hashmap_full_update_setup(BPF_MAP_TYPE_HASH);
    }
#[no_mangle]
unsafe extern "C" fn rhash_setup() {
    static void rhash_setup(void)
    {
    hashmap_full_update_setup(BPF_MAP_TYPE_RHASH);
    }
#[no_mangle]
unsafe extern "C" fn hashmap_report_final(res[]: bench_res, res_cnt: c_int) {
    static void hashmap_report_final(struct bench_res res[], int res_cnt)
    {
    let mut nr_cpus: c_uint = bpf_num_possible_cpus();
    int i;
    for (i = 0; i < nr_cpus; i++) {
    let mut time: u64 = ctx.skel.bss.percpu_time[i];
    if (!time)
    continue;
    printf("%d:hash_map_full_perf %lld events per sec\n",
    i, ctx.skel.bss.nr_loops * 1000000000ll / time);
    }
    }
    const struct bench bench_bpf_hashmap_full_update = {
    .name = "bpf-hashmap-full-update",
    .validate = validate,
    .setup = setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = core::ptr::null_mut(),
    .report_final = hashmap_report_final,
    };
    const struct bench bench_bpf_rhashmap_full_update = {
    .name = "bpf-rhashmap-full-update",
    .validate = validate,
    .setup = rhash_setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = core::ptr::null_mut(),
    .report_final = hashmap_report_final,
    };
