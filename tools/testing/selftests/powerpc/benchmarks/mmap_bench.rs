//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/benchmarks/mmap_bench.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2016, Anton Blanchard, Michael Ellerman, IBM Corp.
//

pub const ITERATIONS: c_int = 5000000;

    static int pg_fault;
    let mut iterations: static int = ITERATIONS;
    static struct option options[] = {
    { "pgfault", no_argument, &pg_fault, 1 },
    { "iterations", required_argument, 0, 'i' },
    { 0, },
    };
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("mmap_bench <--pgfault> <--iterations count>\n");
    }
#[no_mangle]
pub unsafe extern "C" fn test_mmap() -> c_int {
    int test_mmap(void)
    {
    struct timespec ts_start, ts_end;
    let mut i: c_ulong = iterations;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    while (i--) {
    char *c = mmap(core::ptr::null_mut(), MEMSIZE, PROT_READ|PROT_WRITE,
    MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    FAIL_IF(c == MAP_FAILED);
    if (pg_fault) {
    int count;
    for (count = 0; count < CHUNK_COUNT; count++)
    c[count << 16] = 'c';
    }
    munmap(c, MEMSIZE);
    }
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    printf("time = %.6f\n", ts_end.tv_sec - ts_start.tv_sec + (ts_end.tv_nsec - ts_start.tv_nsec) / 1e9);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    signed char c;
    while (1) {
    let mut option_index: c_int = 0;
    c = getopt_long(argc, argv, "", options, &option_index);
    if (c == -1)
    break;
    switch (c) {
    case 0:
    if (options[option_index].flag != 0)
    break;
    usage();
    exit(1);
    break;
    case 'i':
    iterations = atoi(optarg);
    break;
    default:
    usage();
    exit(1);
    }
    }
    test_harness_set_timeout(300);
    return test_harness(test_mmap, "mmap_bench");
    }
