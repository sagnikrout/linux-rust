//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/skew_consistency.c
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


// ADJ_FREQ Skew consistency test
// by: john stultz (johnstul@us.ibm.com)
// (C) Copyright IBM 2012
// Licensed under the GPLv2
//
// NOTE: This is a meta-test which cranks the ADJ_FREQ knob back
// and forth and watches for consistency problems. Thus this test requires
// that the inconsistency-check tests be present in the same directory it
// is run from.
//
// To build:
// $ gcc skew_consistency.c -o skew_consistency -lrt
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
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct timex tx;
    int ret, ppm;
    pid_t pid;
    printf("Running Asynchronous Frequency Changing Tests...\n");
    pid = fork();
    if (!pid)
    return system("./inconsistency-check -t 60");
    ppm = 500;
    ret = 0;
    while (pid != waitpid(pid, &ret, WNOHANG)) {
    ppm = -ppm;
    tx.modes = ADJ_FREQUENCY;
    tx.freq = ppm << 16;
    adjtimex(&tx);
    usleep(500000);
    }
// Set things back
    tx.modes = ADJ_FREQUENCY;
    tx.offset = 0;
    adjtimex(&tx);
    if (ret) {
    printf("[FAILED]\n");
    ksft_exit_fail();
    }
    printf("[OK]\n");
    ksft_exit_pass();
    }
