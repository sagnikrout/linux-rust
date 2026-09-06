//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/raw_skew.c
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


// CLOCK_MONOTONIC vs CLOCK_MONOTONIC_RAW skew test
// by: john stultz (johnstul@us.ibm.com)
// John Stultz <john.stultz@linaro.org>
// (C) Copyright IBM 2012
// (C) Copyright Linaro Limited 2015
// Licensed under the GPLv2
//
// To build:
// $ gcc raw_skew.c -o raw_skew -lrt
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

    __typeof__(x) __x = (x);	\
    __typeof__(s) __s = (s);	\
    __x < 0 ? -(-__x >> __s) : __x >> __s; \
    })
#[no_mangle]
pub unsafe extern "C" fn llabs(val: c_longlong) -> c_longlong {
    long long llabs(long long val)
    {
    if (val < 0)
    val = -val;
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn ts_to_nsec(ts: timespec) -> c_ulonglong {
    unsigned long long ts_to_nsec(struct timespec ts)
    {
    return ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
    }
#[no_mangle]
pub unsafe extern "C" fn nsec_to_ts(ns: c_longlong) -> timespec {
    struct timespec nsec_to_ts(long long ns)
    {
    struct timespec ts;
    ts.tv_sec = ns/NSEC_PER_SEC;
    ts.tv_nsec = ns%NSEC_PER_SEC;
    return ts;
    }
#[no_mangle]
pub unsafe extern "C" fn diff_timespec(start: timespec, end: timespec) -> c_longlong {
    long long diff_timespec(struct timespec start, struct timespec end)
    {
    long long start_ns, end_ns;
    start_ns = ts_to_nsec(start);
    end_ns = ts_to_nsec(end);
    return end_ns - start_ns;
    }
#[no_mangle]
pub unsafe extern "C" fn get_monotonic_and_raw(mon: *mut timespec, raw: *mut timespec) {
    void get_monotonic_and_raw(struct timespec *mon, struct timespec *raw)
    {
    struct timespec start, mid, end;
    let mut diff: c_longlong = 0, tmp;
    int i;
    for (i = 0; i < 3; i++) {
    long long newdiff;
    clock_gettime(CLOCK_MONOTONIC, &start);
    clock_gettime(CLOCK_MONOTONIC_RAW, &mid);
    clock_gettime(CLOCK_MONOTONIC, &end);
    newdiff = diff_timespec(start, end);
    if (diff == 0 || newdiff < diff) {
    diff = newdiff;
// raw = mid;
    tmp = (ts_to_nsec(start) + ts_to_nsec(end))/2;
// mon = nsec_to_ts(tmp);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct timespec mon, raw, start, end;
    long long delta1, delta2, interval, eppm, ppm;
    struct timex tx1, tx2;
    setbuf(stdout, core::ptr::null_mut());
    if (clock_gettime(CLOCK_MONOTONIC_RAW, &raw)) {
    printf("ERR: NO CLOCK_MONOTONIC_RAW\n");
    return -1;
    }
    tx1.modes = 0;
    adjtimex(&tx1);
    get_monotonic_and_raw(&mon, &raw);
    start = mon;
    delta1 = diff_timespec(mon, raw);
    if (tx1.offset)
    printf("WARNING: ADJ_OFFSET in progress, this will cause inaccurate results\n");
    printf("Estimating clock drift: ");
    fflush(stdout);
    sleep(120);
    get_monotonic_and_raw(&mon, &raw);
    end = mon;
    tx2.modes = 0;
    adjtimex(&tx2);
    delta2 = diff_timespec(mon, raw);
    interval = diff_timespec(start, end);
// calculate measured ppm between MONOTONIC and MONOTONIC_RAW
    eppm = ((delta2-delta1)*NSEC_PER_SEC)/interval;
    eppm = -eppm;
    printf("%lld.%i(est)", eppm/1000, abs((int)(eppm%1000)));
// Avg the two actual freq samples adjtimex gave us
    ppm = (long long)(tx1.freq + tx2.freq) * 1000 / 2;
    ppm = shift_right(ppm, 16);
    printf(" %lld.%i(act)", ppm/1000, abs((int)(ppm%1000)));
    if (llabs(eppm - ppm) > 1000) {
    if (tx1.offset || tx2.offset ||
    tx1.freq != tx2.freq || tx1.tick != tx2.tick) {
    printf("	[SKIP]\n");
    ksft_exit_skip("The clock was adjusted externally. Shutdown NTPd or other time sync daemons\n");
    }
    printf("	[FAILED]\n");
    ksft_exit_fail();
    }
    printf("	[OK]\n");
    ksft_exit_pass();
    }
