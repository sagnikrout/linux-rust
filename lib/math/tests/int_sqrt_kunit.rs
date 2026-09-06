//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/int_sqrt_kunit.c
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
    pub x: c_ulong,
    pub expected_result: c_ulong,
    pub name: *const c_char,
}

    static const struct test_case_params params[] = {
    { 0, 0, "edge case: square root of 0" },
    { 1, 1, "perfect square: square root of 1" },
    { 2, 1, "non-perfect square: square root of 2" },
    { 3, 1, "non-perfect square: square root of 3" },
    { 4, 2, "perfect square: square root of 4" },
    { 5, 2, "non-perfect square: square root of 5" },
    { 6, 2, "non-perfect square: square root of 6" },
    { 7, 2, "non-perfect square: square root of 7" },
    { 8, 2, "non-perfect square: square root of 8" },
    { 9, 3, "perfect square: square root of 9" },
    { 15, 3, "non-perfect square: square root of 15 (N-1 from 16)" },
    { 16, 4, "perfect square: square root of 16" },
    { 17, 4, "non-perfect square: square root of 17 (N+1 from 16)" },
    { 80, 8, "non-perfect square: square root of 80 (N-1 from 81)" },
    { 81, 9, "perfect square: square root of 81" },
    { 82, 9, "non-perfect square: square root of 82 (N+1 from 81)" },
    { 255, 15, "non-perfect square: square root of 255 (N-1 from 256)" },
    { 256, 16, "perfect square: square root of 256" },
    { 257, 16, "non-perfect square: square root of 257 (N+1 from 256)" },
    { 2147483648, 46340, "large input: square root of 2147483648" },
    { 4294967295, 65535, "edge case: ULONG_MAX for 32-bit" },
    };
#[no_mangle]
unsafe extern "C" fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    static void get_desc(const struct test_case_params *tc, char *desc)
    {
    strscpy(desc, tc.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(int_sqrt, params, get_desc);
#[no_mangle]
unsafe extern "C" fn int_sqrt_test(test: *mut kunit) {
    static void int_sqrt_test(struct kunit *test)
    {
    const struct test_case_params *tc = (const struct test_case_params *)test.param_value;
    KUNIT_EXPECT_EQ(test, tc.expected_result, int_sqrt(tc.x));
    }
    static struct kunit_case math_int_sqrt_test_cases[] = {
    KUNIT_CASE_PARAM(int_sqrt_test, int_sqrt_gen_params),
    {}
    };
    static struct kunit_suite int_sqrt_test_suite = {
    .name = "math-int_sqrt",
    .test_cases = math_int_sqrt_test_cases,
    };
    kunit_test_suites(&int_sqrt_test_suite);
    MODULE_DESCRIPTION("math.int_sqrt KUnit test suite");
    MODULE_LICENSE("GPL");
