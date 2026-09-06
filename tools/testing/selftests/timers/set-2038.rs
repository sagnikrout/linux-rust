//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/set-2038.c
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


// Time bounds setting test
// by: john stultz (johnstul@us.ibm.com)
// (C) Copyright IBM 2012
// Licensed under the GPLv2
//
// NOTE: This is a meta-test which sets the time to edge cases then
// uses other tests to detect problems. Thus this test requires that
// the inconsistency-check and nanosleep tests be present in the same
// directory it is run from.
//
// To build:
// $ gcc set-2038.c -o set-2038 -lrt
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

pub const YEAR_1970: c_int = 1;
pub const YEAR_2038: c_uint = 0x7fffffffL			/*overflows 32bit time_t */;

#[no_mangle]
pub unsafe extern "C" fn is32bits() -> c_int {
    int is32bits(void)
    {
    return (sizeof(long) == 4);
    }
#[no_mangle]
pub unsafe extern "C" fn settime(time: c_longlong) -> c_int {
    int settime(long long time)
    {
    struct timeval now;
    int ret;
    now.tv_sec = (time_t)time;
    now.tv_usec  = 0;
    ret = settimeofday(&now, core::ptr::null_mut());
    printf("Setting time to 0x%lx: %d\n", (long)time, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_tests() -> c_int {
    int do_tests(void)
    {
    int ret;
    ret = system("date");
    ret = system("./inconsistency-check -c 0 -t 20");
    ret |= system("./nanosleep");
    ret |= system("./nsleep-lat");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut ret: c_int = 0;
    int opt, dangerous = 0;
    time_t start;
// Process arguments
    while ((opt = getopt(argc, argv, "d")) != -1) {
    switch (opt) {
    case 'd':
    dangerous = 1;
    }
    }
    start = time(0);
// First test that crazy values don't work
    if (!settime(YEAR_1901)) {
    ret = -1;
    goto out;
    }
    if (!settime(YEAR_MAX)) {
    ret = -1;
    goto out;
    }
    if (!is32bits() && !settime(YEAR_2262)) {
    ret = -1;
    goto out;
    }
// Now test behavior near edges
    settime(YEAR_1970);
    ret = do_tests();
    if (ret)
    goto out;
    settime(YEAR_2038 - 600);
    ret = do_tests();
    if (ret)
    goto out;
// The rest of the tests can blowup on 32bit systems
    if (is32bits() && !dangerous)
    goto out;
// Test rollover behavior 32bit edge
    settime(YEAR_2038 - 10);
    ret = do_tests();
    if (ret)
    goto out;
    settime(YEAR_2262 - 600);
    ret = do_tests();
    out:
// restore clock
    settime(start);
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    }
