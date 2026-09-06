//! Automatically rewritten from C to Rust
//! Source: lib/crypto/tests/md5_kunit.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2025 Google LLC
//

    static struct kunit_case hash_test_cases[] = {
    HASH_KUNIT_CASES,
    KUNIT_CASE(benchmark_hash),
    {},
    };
    static struct kunit_suite hash_test_suite = {
    .name = "md5",
    .test_cases = hash_test_cases,
    };
    kunit_test_suite(hash_test_suite);
    MODULE_DESCRIPTION("KUnit tests and benchmark for MD5 and HMAC-MD5");
    MODULE_LICENSE("GPL");
