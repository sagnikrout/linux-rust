//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/inconsistency-check.c
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


// Time inconsistency check test
// by: john stultz (johnstul@us.ibm.com)
// (C) Copyright IBM 2003, 2004, 2005, 2012
// (C) Copyright Linaro Limited 2015
// Licensed under the GPLv2
//
// To build:
// $ gcc inconsistency-check.c -o inconsistency-check -lrt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// CLOCK_HWSPECIFIC == CLOCK_SGI_CYCLE (Deprecated)
pub const CLOCK_HWSPECIFIC: c_int = 10;
pub const CALLS_PER_LOOP: c_int = 64;
// returns 1 if a <= b, 0 otherwise
#[no_mangle]
pub unsafe extern "C" fn in_order(a: timespec, b: timespec) -> c_int {
    static inline int in_order(struct timespec a, struct timespec b)
    {
// use unsigned to avoid false positives on 2038 rollover
    if ((unsigned long)a.tv_sec < (unsigned long)b.tv_sec)
    return 1;
    if ((unsigned long)a.tv_sec > (unsigned long)b.tv_sec)
    return 0;
    if (a.tv_nsec > b.tv_nsec)
    return 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn consistency_test(clock_type: c_int, seconds: c_ulong) -> c_int {
    int consistency_test(int clock_type, unsigned long seconds)
    {
    struct timespec list[CALLS_PER_LOOP];
    int i, inconsistent;
    long now, then;
    time_t t;
    char *start_str;
    clock_gettime(clock_type, &list[0]);
    now = then = list[0].tv_sec;
// timestamp start of test
    t = time(0);
    start_str = ctime(&t);
    while (seconds == -1 || now - then < seconds) {
    inconsistent = -1;
// Fill list
    for (i = 0; i < CALLS_PER_LOOP; i++)
    clock_gettime(clock_type, &list[i]);
// Check for inconsistencies
    for (i = 0; i < CALLS_PER_LOOP - 1; i++)
    if (!in_order(list[i], list[i+1]))
    inconsistent = i;
// display inconsistency
    if (inconsistent >= 0) {
    unsigned long long delta;
    ksft_print_msg("\%s\n", start_str);
    for (i = 0; i < CALLS_PER_LOOP; i++) {
    if (i == inconsistent)
    ksft_print_msg("--------------------\n");
    ksft_print_msg("%lu:%lu\n", list[i].tv_sec,
    list[i].tv_nsec);
    if (i == inconsistent + 1)
    ksft_print_msg("--------------------\n");
    }
    delta = list[inconsistent].tv_sec * NSEC_PER_SEC;
    delta += list[inconsistent].tv_nsec;
    delta -= list[inconsistent+1].tv_sec * NSEC_PER_SEC;
    delta -= list[inconsistent+1].tv_nsec;
    ksft_print_msg("Delta: %llu ns\n", delta);
    fflush(0);
// timestamp inconsistency
    t = time(0);
    ksft_print_msg("%s\n", ctime(&t));
    return -1;
    }
    now = list[0].tv_sec;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int clockid, opt;
    let mut userclock: c_int = CLOCK_REALTIME;
    let mut maxclocks: c_int = CLOCK_TAI + 1;
    let mut runtime: c_int = 10;
    struct timespec ts;
// Process arguments
    while ((opt = getopt(argc, argv, "t:c:")) != -1) {
    switch (opt) {
    case 't':
    runtime = atoi(optarg);
    break;
    case 'c':
    userclock = atoi(optarg);
    maxclocks = userclock + 1;
    break;
    default:
    printf("Usage: %s [-t <secs>] [-c <clockid>]\n", argv[0]);
    printf("	-t: Number of seconds to run\n");
    printf("	-c: clockid to use (default, all clockids)\n");
    exit(-1);
    }
    }
    setbuf(stdout, core::ptr::null_mut());
    ksft_print_header();
    ksft_set_plan(maxclocks - userclock);
    for (clockid = userclock; clockid < maxclocks; clockid++) {
    if (clockid == CLOCK_HWSPECIFIC || clock_gettime(clockid, &ts)) {
    ksft_test_result_skip("%-31s\n", clock_name(clockid));
    continue;
    }
    if (consistency_test(clockid, runtime)) {
    ksft_test_result_fail("%-31s\n", clock_name(clockid));
    ksft_exit_fail();
    } else {
    ksft_test_result_pass("%-31s\n", clock_name(clockid));
    }
    }
    ksft_exit_pass();
    }
