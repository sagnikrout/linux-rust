//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/gcd_kunit.c
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
    pub val1: c_ulong,
    pub val2: c_ulong,
    pub expected_result: c_ulong,
    pub name: *const c_char,
}

    static const struct test_case_params params[] = {
    { 48, 18, 6, "GCD of 48 and 18" },
    { 18, 48, 6, "GCD of 18 and 48" },
    { 56, 98, 14, "GCD of 56 and 98" },
    { 17, 13, 1, "Coprime numbers" },
    { 101, 103, 1, "Coprime numbers" },
    { 270, 192, 6, "GCD of 270 and 192" },
    { 0, 5, 5, "GCD with zero" },
    { 7, 0, 7, "GCD with zero reversed" },
    { 36, 36, 36, "GCD of identical numbers" },
    { ULONG_MAX, 1, 1, "GCD of max ulong and 1" },
    { ULONG_MAX, ULONG_MAX, ULONG_MAX, "GCD of max ulong values" },
    };
#[no_mangle]
unsafe extern "C" fn get_desc(tc: *const test_case_params, desc: *mut c_char) {
    static void get_desc(const struct test_case_params *tc, char *desc)
    {
    strscpy(desc, tc.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(gcd, params, get_desc);
#[no_mangle]
unsafe extern "C" fn gcd_test(test: *mut kunit) {
    static void gcd_test(struct kunit *test)
    {
    const struct test_case_params *tc = (const struct test_case_params *)test.param_value;
    KUNIT_EXPECT_EQ(test, tc.expected_result, gcd(tc.val1, tc.val2));
    }
    static struct kunit_case math_gcd_test_cases[] = {
    KUNIT_CASE_PARAM(gcd_test, gcd_gen_params),
    {}
    };
    static struct kunit_suite gcd_test_suite = {
    .name = "math-gcd",
    .test_cases = math_gcd_test_cases,
    };
    kunit_test_suite(gcd_test_suite);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("math.gcd KUnit test suite");
    MODULE_AUTHOR("Yu-Chun Lin <eleanor15x@gmail.com>");
