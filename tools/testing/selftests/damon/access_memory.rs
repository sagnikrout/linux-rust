//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/damon/access_memory.c
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
// Artificial memory access program for testing DAMON.
//

    enum access_mode {
    ACCESS_MODE_ONCE,
    ACCESS_MODE_REPEAT,
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char **regions;
    clock_t start_clock;
    int nr_regions;
    int sz_region;
    int access_time_ms;
    let mut mode: enum access_mode = ACCESS_MODE_ONCE;
    int i;
    if (argc < 4) {
    printf("Usage: %s <number> <size (bytes)> <time (ms)> [mode]\n",
    argv[0]);
    return -1;
    }
    nr_regions = atoi(argv[1]);
    sz_region = atoi(argv[2]);
    access_time_ms = atoi(argv[3]);
    if (argc > 4 && !strcmp(argv[4], "repeat"))
    mode = ACCESS_MODE_REPEAT;
    regions = malloc(sizeof(*regions) * nr_regions);
    for (i = 0; i < nr_regions; i++)
    regions[i] = malloc(sz_region);
    do {
    for (i = 0; i < nr_regions; i++) {
    start_clock = clock();
    while ((clock() - start_clock) * 1000 / CLOCKS_PER_SEC
    < access_time_ms)
    memset(regions[i], i, sz_region);
    }
    } while (mode == ACCESS_MODE_REPEAT);
    return 0;
    }
