//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm/tests/vectors-page.c
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

#[no_mangle]
unsafe extern "C" fn test__vectors_page(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__vectors_page(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    void *start, *end;
    if (find_map(&start, &end, VECTORS__MAP_NAME)) {
    pr_err("%s not found, is CONFIG_KUSER_HELPERS enabled?\n",
    VECTORS__MAP_NAME);
    return TEST_FAIL;
    }
    return TEST_OK;
    }
    DEFINE_SUITE("Vectors page", vectors_page);
