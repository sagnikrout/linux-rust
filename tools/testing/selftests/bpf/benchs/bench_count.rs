//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_count.c
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

// COUNT-GLOBAL benchmark
    static struct count_global_ctx {
    struct counter hits;
    } count_global_ctx;
    static void *count_global_producer(void *input)
    {
    struct count_global_ctx *ctx = &count_global_ctx;
    while (true) {
    atomic_inc(&ctx.hits.value);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn count_global_measure(res: *mut bench_res) {
    static void count_global_measure(struct bench_res *res)
    {
    struct count_global_ctx *ctx = &count_global_ctx;
    res.hits = atomic_swap(&ctx.hits.value, 0);
    }
// COUNT-local benchmark
    static struct count_local_ctx {
    struct counter *hits;
    } count_local_ctx;
#[no_mangle]
unsafe extern "C" fn count_local_setup() {
    static void count_local_setup(void)
    {
    struct count_local_ctx *ctx = &count_local_ctx;
    ctx.hits = calloc(env.producer_cnt, sizeof(*ctx.hits));
    if (!ctx.hits)
    exit(1);
    }
    static void *count_local_producer(void *input)
    {
    struct count_local_ctx *ctx = &count_local_ctx;
    let mut idx: c_int = (long)input;
    while (true) {
    atomic_inc(&ctx.hits[idx].value);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn count_local_measure(res: *mut bench_res) {
    static void count_local_measure(struct bench_res *res)
    {
    struct count_local_ctx *ctx = &count_local_ctx;
    int i;
    for (i = 0; i < env.producer_cnt; i++) {
    res.hits += atomic_swap(&ctx.hits[i].value, 0);
    }
    }
    const struct bench bench_count_global = {
    .name = "count-global",
    .producer_thread = count_global_producer,
    .measure = count_global_measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
    const struct bench bench_count_local = {
    .name = "count-local",
    .setup = count_local_setup,
    .producer_thread = count_local_producer,
    .measure = count_local_measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
