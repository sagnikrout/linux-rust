//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/int_log_kunit.c
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
    pub value: u32,
    pub expected_result: c_uint,
    pub name: *const c_char,
}

// The expected result takes into account the log error
    static const struct test_case_params intlog2_params[] = {
    {0, 0, "Log base 2 of 0"},
    {1, 0, "Log base 2 of 1"},
    {2, 16777216, "Log base 2 of 2"},
    {3, 26591232, "Log base 2 of 3"},
    {4, 33554432, "Log base 2 of 4"},
    {8, 50331648, "Log base 2 of 8"},
    {16, 67108864, "Log base 2 of 16"},
    {32, 83886080, "Log base 2 of 32"},
    {U32_MAX, 536870911, "Log base 2 of MAX"},
    };
    static const struct test_case_params intlog10_params[] = {
    {0, 0, "Log base 10 of 0"},
    {1, 0, "Log base 10 of 1"},
    {6, 13055203, "Log base 10 of 6"},
    {10, 16777225, "Log base 10 of 10"},
    {100, 33554450, "Log base 10 of 100"},
    {1000, 50331675, "Log base 10 of 1000"},
    {10000, 67108862, "Log base 10 of 10000"},
    {U32_MAX, 161614247, "Log base 10 of MAX"}
    };
#[no_mangle]
unsafe extern "C" fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    static void get_desc(const struct test_case_params *tc, char *desc)
    {
    strscpy(desc, tc.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(intlog2, intlog2_params, get_desc);
#[no_mangle]
unsafe extern "C" fn intlog2_test(test: *mut kunit) {
    static void intlog2_test(struct kunit *test)
    {
    const struct test_case_params *tc = (const struct test_case_params *)test.param_value;
    KUNIT_EXPECT_EQ(test, tc.expected_result, intlog2(tc.value));
    }
    KUNIT_ARRAY_PARAM(intlog10, intlog10_params, get_desc);
#[no_mangle]
unsafe extern "C" fn intlog10_test(test: *mut kunit) {
    static void intlog10_test(struct kunit *test)
    {
    const struct test_case_params *tc = (const struct test_case_params *)test.param_value;
    KUNIT_EXPECT_EQ(test, tc.expected_result, intlog10(tc.value));
    }
    static struct kunit_case math_int_log_test_cases[] = {
    KUNIT_CASE_PARAM(intlog2_test, intlog2_gen_params),
    KUNIT_CASE_PARAM(intlog10_test, intlog10_gen_params),
    {}
    };
    static struct kunit_suite int_log_test_suite = {
    .name = "math-int_log",
    .test_cases =  math_int_log_test_cases,
    };
    kunit_test_suites(&int_log_test_suite);
    MODULE_DESCRIPTION("math.int_log KUnit test suite");
    MODULE_LICENSE("GPL");
