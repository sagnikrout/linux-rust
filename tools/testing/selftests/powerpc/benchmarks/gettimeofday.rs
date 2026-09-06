//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/benchmarks/gettimeofday.c
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
// Copyright 2015, Anton Blanchard, IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn test_gettimeofday() -> c_int {
    static int test_gettimeofday(void)
    {
    int i;
    struct timeval tv_start, tv_end, tv_diff;
    gettimeofday(&tv_start, core::ptr::null_mut());
    for(i = 0; i < 100000000; i++) {
    gettimeofday(&tv_end, core::ptr::null_mut());
    }
    timersub(&tv_end, &tv_start, &tv_diff);
    printf("time = %.6f\n", tv_diff.tv_sec + (tv_diff.tv_usec) * 1e-6);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_gettimeofday, "gettimeofday");
    }
