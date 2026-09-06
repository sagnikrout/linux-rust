//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/rational_kunit.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rational_test_param {
    pub den: unsigned long num,,
    pub max_den: unsigned long max_num,,
    pub exp_den: unsigned long exp_num,,
    pub name: *const c_char,
}

    static const struct rational_test_param test_parameters[] = {
    { 1230,	10,	100, 20,	100, 1,    "Exceeds bounds, semi-convergent term > 1/2 last term" },
    { 34567,100, 	120, 20,	120, 1,    "Exceeds bounds, semi-convergent term < 1/2 last term" },
    { 1, 30,	100, 10,	0, 1,	   "Closest to zero" },
    { 1, 19,	100, 10,	1, 10,     "Closest to smallest non-zero" },
    { 27,32,	16, 16,		11, 13,    "Use convergent" },
    { 1155, 7735,	255, 255,	33, 221,   "Exact answer" },
    { 87, 32,	70, 32,		68, 25,    "Semiconvergent, numerator limit" },
    { 14533, 4626,	15000, 2400,	7433, 2366, "Semiconvergent, denominator limit" },
    };
#[no_mangle]
unsafe extern "C" fn get_desc(param: *const rational_test_param, desc: *mut c_char) {
    static void get_desc(const struct rational_test_param *param, char *desc)
    {
    strscpy(desc, param.name, KUNIT_PARAM_DESC_SIZE);
    }
// Creates function rational_gen_params
    KUNIT_ARRAY_PARAM(rational, test_parameters, get_desc);
#[no_mangle]
unsafe extern "C" fn rational_test(test: *mut kunit) {
    static void rational_test(struct kunit *test)
    {
    const struct rational_test_param *param = (const struct rational_test_param *)test.param_value;
    let mut n: c_ulong = 0, d = 0;
    rational_best_approximation(param.num, param.den, param.max_num, param.max_den, &n, &d);
    KUNIT_EXPECT_EQ(test, n, param.exp_num);
    KUNIT_EXPECT_EQ(test, d, param.exp_den);
    }
    static struct kunit_case rational_test_cases[] = {
    KUNIT_CASE_PARAM(rational_test, rational_gen_params),
    {}
    };
    static struct kunit_suite rational_test_suite = {
    .name = "rational",
    .test_cases = rational_test_cases,
    };
    kunit_test_suites(&rational_test_suite);
    MODULE_DESCRIPTION("Rational fractions unit test");
    MODULE_LICENSE("GPL v2");
