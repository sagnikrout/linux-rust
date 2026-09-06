//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/set-tai.c
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


// Set tai offset
// by: John Stultz <john.stultz@linaro.org>
// (C) Copyright Linaro 2013
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
pub unsafe extern "C" fn set_tai(offset: c_int) -> c_int {
    int set_tai(int offset)
    {
    struct timex tx;
    memset(&tx, 0, sizeof(tx));
    tx.modes = ADJ_TAI;
    tx.constant = offset;
    return adjtimex(&tx);
    }
#[no_mangle]
pub unsafe extern "C" fn get_tai() -> c_int {
    int get_tai(void)
    {
    struct timex tx;
    memset(&tx, 0, sizeof(tx));
    adjtimex(&tx);
    return tx.tai;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int i, ret;
    ret = get_tai();
    printf("tai offset started at %i\n", ret);
    printf("Checking tai offsets can be properly set: ");
    fflush(stdout);
    for (i = 1; i <= 60; i++) {
    ret = set_tai(i);
    ret = get_tai();
    if (ret != i) {
    printf("[FAILED] expected: %i got %i\n", i, ret);
    ksft_exit_fail();
    }
    }
    printf("[OK]\n");
    ksft_exit_pass();
    }
