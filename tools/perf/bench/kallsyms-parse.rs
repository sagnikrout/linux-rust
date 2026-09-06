//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/kallsyms-parse.c
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
// Benchmark of /proc/kallsyms parsing.
//
// Copyright 2020 Google LLC.
//

    let mut iterations: static unsigned int = 100;
    static const struct option options[] = {
    OPT_UINTEGER('i', "iterations", &iterations,
    "Number of iterations used to compute average"),
    OPT_END()
    };
    static const char *const bench_usage[] = {
    "perf bench internals kallsyms-parse <options>",
    core::ptr::null_mut()
    };
    static int bench_process_symbol(void *arg __maybe_unused,
    const char *name __maybe_unused,
    char type __maybe_unused,
    u64 start __maybe_unused)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_kallsyms_parse() -> c_int {
    static int do_kallsyms_parse(void)
    {
    struct timeval start, end, diff;
    u64 runtime_us;
    unsigned int i;
    double time_average, time_stddev;
    int err;
    struct stats time_stats;
    init_stats(&time_stats);
    for (i = 0; i < iterations; i++) {
    gettimeofday(&start, core::ptr::null_mut());
    err = kallsyms__parse("/proc/kallsyms", core::ptr::null_mut(),
    bench_process_symbol);
    if (err)
    return err;
    gettimeofday(&end, core::ptr::null_mut());
    timersub(&end, &start, &diff);
    runtime_us = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    update_stats(&time_stats, runtime_us);
    }
    time_average = avg_stats(&time_stats) / USEC_PER_MSEC;
    time_stddev = stddev_stats(&time_stats) / USEC_PER_MSEC;
    printf("  Average kallsyms__parse took: %.3f ms (+- %.3f ms)\n",
    time_average, time_stddev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_kallsyms_parse(argc: c_int, argv: *const c_char) -> c_int {
    int bench_kallsyms_parse(int argc, const char **argv)
    {
    argc = parse_options(argc, argv, options, bench_usage, 0);
    if (argc) {
    usage_with_options(bench_usage, options);
    exit(EXIT_FAILURE);
    }
    return do_kallsyms_parse();
    }
