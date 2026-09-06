//! Automatically rewritten from C to Rust
//! Source: lib/crypto/tests/nh_kunit.c
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

#[no_mangle]
unsafe extern "C" fn test_nh(test: *mut kunit) {
    static void test_nh(struct kunit *test)
    {
    u32 *key = memdup_buf(test, nh_test_key, NH_KEY_BYTES);
    __le64 hash[NH_NUM_PASSES];
    le32_to_cpu_array(key, NH_KEY_WORDS);
    nh(key, nh_test_msg, 16, hash);
    KUNIT_ASSERT_MEMEQ(test, hash, nh_test_val16, NH_HASH_BYTES);
    nh(key, nh_test_msg, 96, hash);
    KUNIT_ASSERT_MEMEQ(test, hash, nh_test_val96, NH_HASH_BYTES);
    nh(key, nh_test_msg, 256, hash);
    KUNIT_ASSERT_MEMEQ(test, hash, nh_test_val256, NH_HASH_BYTES);
    nh(key, nh_test_msg, 1024, hash);
    KUNIT_ASSERT_MEMEQ(test, hash, nh_test_val1024, NH_HASH_BYTES);
    }
    static struct kunit_case nh_test_cases[] = {
    KUNIT_CASE(test_nh),
    {},
    };
    static struct kunit_suite nh_test_suite = {
    .name = "nh",
    .test_cases = nh_test_cases,
    };
    kunit_test_suite(nh_test_suite);
    MODULE_DESCRIPTION("KUnit tests for NH");
    MODULE_LICENSE("GPL");
