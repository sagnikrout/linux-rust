//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/mqueue-lat.c
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


// Measure mqueue timeout latency
// by: john stultz (john.stultz@linaro.org)
// (C) Copyright Linaro 2013
//
// Inspired with permission from example test by:
// Romain Francoise <romain@orebokech.com>
// Licensed under the GPLv2
//
// To build:
// $ gcc mqueue-lat.c -o mqueue-lat -lrt
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

#[no_mangle]
pub unsafe extern "C" fn timespec_sub(a: timespec, b: timespec) -> c_longlong {
    long long timespec_sub(struct timespec a, struct timespec b)
    {
    let mut ret: c_longlong = NSEC_PER_SEC * b.tv_sec + b.tv_nsec;
    ret -= NSEC_PER_SEC * a.tv_sec + a.tv_nsec;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn timespec_add(ts: timespec, ns: c_ulonglong) -> timespec {
    struct timespec timespec_add(struct timespec ts, unsigned long long ns)
    {
    ts.tv_nsec += ns;
    while (ts.tv_nsec >= NSEC_PER_SEC) {
    ts.tv_nsec -= NSEC_PER_SEC;
    ts.tv_sec++;
    }
    return ts;
    }
#[no_mangle]
pub unsafe extern "C" fn mqueue_lat_test() -> c_int {
    int mqueue_lat_test(void)
    {
    mqd_t q;
    struct mq_attr attr;
    struct timespec start, end, now, target;
    int i, count, ret;
    q = mq_open("/foo", O_CREAT | O_RDONLY, 0666, core::ptr::null_mut());
    if (q < 0) {
    perror("mq_open");
    return -1;
    }
    mq_getattr(q, &attr);
    count = 100;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (i = 0; i < count; i++) {
    char buf[attr.mq_msgsize];
    clock_gettime(CLOCK_REALTIME, &now);
    target = now;
    target = timespec_add(now, TARGET_TIMEOUT); /* 100ms */
    ret = mq_timedreceive(q, buf, sizeof(buf), core::ptr::null_mut(), &target);
    if (ret < 0 && errno != ETIMEDOUT) {
    perror("mq_timedreceive");
    return -1;
    }
    }
    clock_gettime(CLOCK_MONOTONIC, &end);
    mq_close(q);
    if ((timespec_sub(start, end)/count) > TARGET_TIMEOUT + UNRESONABLE_LATENCY)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int ret;
    printf("Mqueue latency :                          ");
    fflush(stdout);
    ret = mqueue_lat_test();
    if (ret < 0) {
    printf("[FAILED]\n");
    ksft_exit_fail();
    }
    printf("[OK]\n");
    ksft_exit_pass();
    }
