//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/damon/access_memory_even.c
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
// Receives number of regions and size of each region from user.  Allocate the
// regions and repeatedly access even numbered (starting from zero) regions.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char **regions;
    int nr_regions;
    int sz_region;
    int i;
    if (argc != 3) {
    printf("Usage: %s <number> <size (bytes)>\n", argv[0]);
    return -1;
    }
    nr_regions = atoi(argv[1]);
    sz_region = atoi(argv[2]);
    regions = malloc(sizeof(*regions) * nr_regions);
    for (i = 0; i < nr_regions; i++)
    regions[i] = malloc(sz_region);
    while (1) {
    for (i = 0; i < nr_regions; i++) {
    if (i % 2 == 0)
    memset(regions[i], i, sz_region);
    }
    }
    return 0;
    }
