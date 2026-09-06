//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/int_pow_kunit.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case_params {
    pub base: u64,
    pub exponent: c_uint,
    pub expected_result: u64,
    pub name: *const c_char,
}

    static const struct test_case_params params[] = {
    { 64, 0, 1, "Power of zero" },
    { 64, 1, 64, "Power of one"},
    { 0, 5, 0, "Base zero" },
    { 1, 64, 1, "Base one" },
    { 2, 2, 4, "Two squared"},
    { 2, 3, 8, "Two cubed"},
    { 5, 5, 3125, "Five raised to the fifth power" },
    { U64_MAX, 1, U64_MAX, "Max base" },
    { 2, 63, 9223372036854775808ULL, "Large result"},
    };
#[no_mangle]
unsafe extern "C" fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    static void get_desc(const struct test_case_params *tc, char *desc)
    {
    strscpy(desc, tc.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(int_pow, params, get_desc);
#[no_mangle]
unsafe extern "C" fn int_pow_test(test: *mut kunit) {
    static void int_pow_test(struct kunit *test)
    {
    const struct test_case_params *tc = (const struct test_case_params *)test.param_value;
    KUNIT_EXPECT_EQ(test, tc.expected_result, int_pow(tc.base, tc.exponent));
    }
    static struct kunit_case math_int_pow_test_cases[] = {
    KUNIT_CASE_PARAM(int_pow_test, int_pow_gen_params),
    {}
    };
    static struct kunit_suite int_pow_test_suite = {
    .name = "math-int_pow",
    .test_cases = math_int_pow_test_cases,
    };
    kunit_test_suites(&int_pow_test_suite);
    MODULE_DESCRIPTION("math.int_pow KUnit test suite");
    MODULE_LICENSE("GPL");
