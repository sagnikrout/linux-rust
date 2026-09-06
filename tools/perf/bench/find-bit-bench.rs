//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/find-bit-bench.c
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
// Benchmark find_next_bit and related bit operations.
//
// Copyright 2020 Google LLC.
//

    let mut outer_iterations: static unsigned int = 5;
    let mut inner_iterations: static unsigned int = 100000;
    static const struct option options[] = {
    OPT_UINTEGER('i', "outer-iterations", &outer_iterations,
    "Number of outer iterations used"),
    OPT_UINTEGER('j', "inner-iterations", &inner_iterations,
    "Number of inner iterations used"),
    OPT_END()
    };
    static const char *const bench_usage[] = {
    "perf bench mem find_bit <options>",
    core::ptr::null_mut()
    };
    static unsigned int accumulator;
    static unsigned int use_of_val;
#[no_mangle]
unsafe extern "C" fn workload(val: c_int) -> noinline void {
    static noinline void workload(int val)
    {
    use_of_val += val;
    accumulator++;
    }

#[no_mangle]
unsafe extern "C" fn asm_test_bit(nr: c_long, addr: *const c_ulong) -> bool {
    static bool asm_test_bit(long nr, const unsigned long *addr)
    {
    bool oldbit;
    asm volatile("bt %2,%1"
    : "=@ccc" (oldbit)
    : "m" (*(unsigned long *)addr), "Ir" (nr) : "memory");
    return oldbit;
    }

#[no_mangle]
unsafe extern "C" fn do_for_each_set_bit(num_bits: c_uint) -> c_int {
    static int do_for_each_set_bit(unsigned int num_bits)
    {
    unsigned long *to_test = bitmap_zalloc(num_bits);
    struct timeval start, end, diff;
    u64 runtime_us;
    struct stats fb_time_stats, tb_time_stats;
    double time_average, time_stddev;
    unsigned int bit, i, j;
    unsigned int set_bits, skip;
    init_stats(&fb_time_stats);
    init_stats(&tb_time_stats);
    for (set_bits = 1; set_bits <= num_bits; set_bits <<= 1) {
    bitmap_zero(to_test, num_bits);
    skip = num_bits / set_bits;
    for (i = 0; i < num_bits; i += skip)
    __set_bit(i, to_test);
    for (i = 0; i < outer_iterations; i++) {

    let mut old: c_uint = accumulator;

    gettimeofday(&start, core::ptr::null_mut());
    for (j = 0; j < inner_iterations; j++) {
    for_each_set_bit(bit, to_test, num_bits)
    workload(bit);
    }
    gettimeofday(&end, core::ptr::null_mut());
    assert(old + (inner_iterations * set_bits) == accumulator);
    timersub(&end, &start, &diff);
    runtime_us = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    update_stats(&fb_time_stats, runtime_us);

    old = accumulator;

    gettimeofday(&start, core::ptr::null_mut());
    for (j = 0; j < inner_iterations; j++) {
    for (bit = 0; bit < num_bits; bit++) {
    if (asm_test_bit(bit, to_test))
    workload(bit);
    }
    }
    gettimeofday(&end, core::ptr::null_mut());
    assert(old + (inner_iterations * set_bits) == accumulator);
    timersub(&end, &start, &diff);
    runtime_us = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    update_stats(&tb_time_stats, runtime_us);
    }
    printf("%d operations %d bits set of %d bits\n",
    inner_iterations, set_bits, num_bits);
    time_average = avg_stats(&fb_time_stats);
    time_stddev = stddev_stats(&fb_time_stats);
    printf("  Average for_each_set_bit took: %.3f usec (+- %.3f usec)\n",
    time_average, time_stddev);
    time_average = avg_stats(&tb_time_stats);
    time_stddev = stddev_stats(&tb_time_stats);
    printf("  Average test_bit loop took:    %.3f usec (+- %.3f usec)\n",
    time_average, time_stddev);
    if (use_of_val == accumulator)  /* Try to avoid compiler tricks. */
    printf("\n");
    }
    bitmap_free(to_test);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_mem_find_bit(argc: c_int, argv: *const c_char) -> c_int {
    int bench_mem_find_bit(int argc, const char **argv)
    {
    let mut err: c_int = 0, i;
    argc = parse_options(argc, argv, options, bench_usage, 0);
    if (argc) {
    usage_with_options(bench_usage, options);
    exit(EXIT_FAILURE);
    }
    for (i = 1; i <= 2048; i <<= 1)
    do_for_each_set_bit(i);
    return err;
    }
