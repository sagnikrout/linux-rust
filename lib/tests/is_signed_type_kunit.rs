//! Automatically rewritten from C to Rust
//! Source: lib/tests/is_signed_type_kunit.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// ./tools/testing/kunit/kunit.py run is_signed_type [--raw_output]
//

    enum unsigned_enum {
    constant_a = 3,
    };
    enum signed_enum {
    constant_b = -1,
    constant_c = 2,
    };
#[no_mangle]
unsafe extern "C" fn is_signed_type_test(test: *mut kunit) {
    static void is_signed_type_test(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, is_signed_type(bool), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(signed char), true);
    KUNIT_EXPECT_EQ(test, is_signed_type(unsigned char), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(char), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(int), true);
    KUNIT_EXPECT_EQ(test, is_signed_type(unsigned int), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(long), true);
    KUNIT_EXPECT_EQ(test, is_signed_type(unsigned long), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(long long), true);
    KUNIT_EXPECT_EQ(test, is_signed_type(unsigned long long), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(enum unsigned_enum), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(enum signed_enum), true);
    KUNIT_EXPECT_EQ(test, is_signed_type(void *), false);
    KUNIT_EXPECT_EQ(test, is_signed_type(const char *), false);
    }
    static struct kunit_case is_signed_type_test_cases[] = {
    KUNIT_CASE(is_signed_type_test),
    {}
    };
    static struct kunit_suite is_signed_type_test_suite = {
    .name = "is_signed_type",
    .test_cases = is_signed_type_test_cases,
    };
    kunit_test_suite(is_signed_type_test_suite);
    MODULE_DESCRIPTION("is_signed_type() KUnit test suite");
    MODULE_LICENSE("Dual MIT/GPL");
