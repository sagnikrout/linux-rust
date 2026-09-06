//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/set-tz.c
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


// Set tz value
// by: John Stultz <john.stultz@linaro.org>
// (C) Copyright Linaro 2016
// Licensed under the GPLv2
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
pub unsafe extern "C" fn set_tz(min: c_int, dst: c_int) -> c_int {
    int set_tz(int min, int dst)
    {
    struct timezone tz;
    tz.tz_minuteswest = min;
    tz.tz_dsttime = dst;
    return settimeofday(0, &tz);
    }
#[no_mangle]
pub unsafe extern "C" fn get_tz_min() -> c_int {
    int get_tz_min(void)
    {
    struct timezone tz;
    struct timeval tv;
    memset(&tz, 0, sizeof(tz));
    gettimeofday(&tv, &tz);
    return tz.tz_minuteswest;
    }
#[no_mangle]
pub unsafe extern "C" fn get_tz_dst() -> c_int {
    int get_tz_dst(void)
    {
    struct timezone tz;
    struct timeval tv;
    memset(&tz, 0, sizeof(tz));
    gettimeofday(&tv, &tz);
    return tz.tz_dsttime;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int i, ret;
    int min, dst;
    min = get_tz_min();
    dst = get_tz_dst();
    printf("tz_minuteswest started at %i, dst at %i\n", min, dst);
    printf("Checking tz_minuteswest can be properly set: ");
    fflush(stdout);
    for (i = -15*60; i < 15*60; i += 30) {
    ret = set_tz(i, dst);
    ret = get_tz_min();
    if (ret != i) {
    printf("[FAILED] expected: %i got %i\n", i, ret);
    goto err;
    }
    }
    printf("[OK]\n");
    printf("Checking invalid tz_minuteswest values are caught: ");
    fflush(stdout);
    if (!set_tz(-15*60-1, dst)) {
    printf("[FAILED] %i didn't return failure!\n", -15*60-1);
    goto err;
    }
    if (!set_tz(15*60+1, dst)) {
    printf("[FAILED] %i didn't return failure!\n", 15*60+1);
    goto err;
    }
    if (!set_tz(-24*60, dst)) {
    printf("[FAILED] %i didn't return failure!\n", -24*60);
    goto err;
    }
    if (!set_tz(24*60, dst)) {
    printf("[FAILED] %i didn't return failure!\n", 24*60);
    goto err;
    }
    printf("[OK]\n");
    set_tz(min, dst);
    ksft_exit_pass();
    err:
    set_tz(min, dst);
    ksft_exit_fail();
    }
