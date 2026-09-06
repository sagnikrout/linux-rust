//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/adjtick.c
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


// adjtimex() tick adjustment test
// by:   John Stultz <john.stultz@linaro.org>
// (C) Copyright Linaro Limited 2015
// Licensed under the GPLv2
//
// To build:
// $ gcc adjtick.c -o adjtick -lrt
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

pub const MILLION: c_int = 1000000;
    long systick;
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
    clock_gettime(CLOCK_MONOTONIC, mon);
    clock_gettime(CLOCK_MONOTONIC_RAW, raw);
// Try to get a more tightly bound pairing
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
pub unsafe extern "C" fn get_ppm_drift() -> c_longlong {
    long long get_ppm_drift(void)
    {
    struct timespec mon_start, raw_start, mon_end, raw_end;
    long long delta1, delta2, eppm;
    get_monotonic_and_raw(&mon_start, &raw_start);
    sleep(15);
    get_monotonic_and_raw(&mon_end, &raw_end);
    delta1 = diff_timespec(mon_start, mon_end);
    delta2 = diff_timespec(raw_start, raw_end);
    eppm = (delta1*MILLION)/delta2 - MILLION;
    return eppm;
    }
#[no_mangle]
pub unsafe extern "C" fn check_tick_adj(tickval: c_long) -> c_int {
    int check_tick_adj(long tickval)
    {
    long long eppm, ppm;
    struct timex tx1;
    tx1.modes	 = ADJ_TICK;
    tx1.modes	|= ADJ_OFFSET;
    tx1.modes	|= ADJ_FREQUENCY;
    tx1.modes	|= ADJ_STATUS;
    tx1.status	= STA_PLL;
    tx1.offset	= 0;
    tx1.freq	= 0;
    tx1.tick	= tickval;
    adjtimex(&tx1);
    sleep(1);
    ppm = ((long long)tickval * MILLION)/systick - MILLION;
    printf("Estimating tick (act: %ld usec, %lld ppm): ", tickval, ppm);
    eppm = get_ppm_drift();
    printf("%lld usec, %lld ppm", systick + (systick * eppm / MILLION), eppm);
    fflush(stdout);
    tx1.modes = 0;
    adjtimex(&tx1);
    if (tx1.offset || tx1.freq || tx1.tick != tickval) {
    printf("	[ERROR]\n");
    printf("\tUnexpected adjtimex return values, make sure ntpd is not running.\n");
    return -1;
    }
//
// Here we use 100ppm difference as an error bound.
// We likely should see better, but some coarse clocksources
// cannot match the HZ tick size accurately, so we have a
// internal correction factor that doesn't scale exactly
// with the adjustment, resulting in > 10ppm error during
// a 10% adjustment. 100ppm also gives us more breathing
// room for interruptions during the measurement.
//
    if (llabs(eppm - ppm) > 100) {
    printf("	[FAILED]\n");
    return -1;
    }
    printf("	[OK]\n");
    return  0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct timespec raw;
    long tick, max, interval, err;
    struct timex tx1;
    err = 0;
    setbuf(stdout, core::ptr::null_mut());
    if (clock_gettime(CLOCK_MONOTONIC_RAW, &raw)) {
    printf("ERR: NO CLOCK_MONOTONIC_RAW\n");
    return -1;
    }
    printf("Each iteration takes about 15 seconds\n");
    systick = sysconf(_SC_CLK_TCK);
    systick = USEC_PER_SEC/sysconf(_SC_CLK_TCK);
    max = systick/10; /* +/- 10% */
    interval = max/4; /* in 4 steps each side */
    for (tick = (systick - max); tick < (systick + max); tick += interval) {
    if (check_tick_adj(tick)) {
    err = 1;
    break;
    }
    }
// Reset things to zero
    tx1.modes	 = ADJ_TICK;
    tx1.modes	|= ADJ_OFFSET;
    tx1.modes	|= ADJ_FREQUENCY;
    tx1.offset	 = 0;
    tx1.freq	 = 0;
    tx1.tick	 = systick;
    adjtimex(&tx1);
    if (err)
    ksft_exit_fail();
    ksft_exit_pass();
    }
