//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_args_test.c
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
// Copyright © 2024 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn call_args_example(test: *mut kunit) {
    static void call_args_example(struct kunit *test)
    {

    KUNIT_EXPECT_EQ(test, bar, 1);
    KUNIT_EXPECT_EQ(test, buz, 4);

    }
#[no_mangle]
unsafe extern "C" fn drop_first_arg_example(test: *mut kunit) {
    static void drop_first_arg_example(struct kunit *test)
    {

    KUNIT_EXPECT_EQ(test, bar, 3);

    }
#[no_mangle]
unsafe extern "C" fn first_arg_example(test: *mut kunit) {
    static void first_arg_example(struct kunit *test)
    {
    let mut X: c_int = 1;

    KUNIT_EXPECT_EQ(test, bar, X);
    KUNIT_EXPECT_STREQ(test, __stringify(bar), "X");

    }
#[no_mangle]
unsafe extern "C" fn last_arg_example(test: *mut kunit) {
    static void last_arg_example(struct kunit *test)
    {
    let mut Q: c_int = 1;

    KUNIT_EXPECT_EQ(test, bar, Q);
    KUNIT_EXPECT_STREQ(test, __stringify(bar), "Q");

    }
#[no_mangle]
unsafe extern "C" fn pick_arg_example(test: *mut kunit) {
    static void pick_arg_example(struct kunit *test)
    {
    let mut Y: c_int = 1, Z = 2;

    KUNIT_EXPECT_EQ(test, bar, Y);
    KUNIT_EXPECT_STREQ(test, __stringify(bar), "Y");
    KUNIT_EXPECT_EQ(test, buz, Z);
    KUNIT_EXPECT_STREQ(test, __stringify(buz), "Z");

    }
#[no_mangle]
unsafe extern "C" fn if_args_example(test: *mut kunit) {
    static void if_args_example(struct kunit *test)
    {
    enum { Z = 1, Q };

    KUNIT_EXPECT_EQ(test, bar, Z);
    KUNIT_EXPECT_EQ(test, buz, Q);
    KUNIT_EXPECT_STREQ(test, __stringify(bar), "Z");
    KUNIT_EXPECT_STREQ(test, __stringify(buz), "Q");

    }
#[no_mangle]
unsafe extern "C" fn sep_comma_example(test: *mut kunit) {
    static void sep_comma_example(struct kunit *test)
    {

    static const char * const a[] = { bar };
    KUNIT_EXPECT_STREQ(test, a[0], "X");
    KUNIT_EXPECT_STREQ(test, a[1], "Y");
    KUNIT_EXPECT_STREQ(test, a[2], "Z");
    KUNIT_EXPECT_STREQ(test, a[3], "Q");
    KUNIT_EXPECT_EQ(test, buz, 4);

    }
// Macro flag: #define NO_ARGS

#[no_mangle]
unsafe extern "C" fn count_args_test(test: *mut kunit) {
    static void count_args_test(struct kunit *test)
    {
    int count;
// COUNT_ARGS() counts to 12
    count = COUNT_ARGS();
    KUNIT_EXPECT_EQ(test, count, 0);
    count = COUNT_ARGS(1);
    KUNIT_EXPECT_EQ(test, count, 1);
    count = COUNT_ARGS(a, b, c, d, e);
    KUNIT_EXPECT_EQ(test, count, 5);
    count = COUNT_ARGS(a, b, c, d, e, f, g, h, i, j, k, l);
    KUNIT_EXPECT_EQ(test, count, 12);
// COUNT_ARGS() does not expand params
    count = COUNT_ARGS(NO_ARGS);
    KUNIT_EXPECT_EQ(test, count, 1);
    count = COUNT_ARGS(FOO_ARGS);
    KUNIT_EXPECT_EQ(test, count, 1);
    }
#[no_mangle]
unsafe extern "C" fn call_args_test(test: *mut kunit) {
    static void call_args_test(struct kunit *test)
    {
    int count;
    count = CALL_ARGS(COUNT_ARGS, NO_ARGS);
    KUNIT_EXPECT_EQ(test, count, 0);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, NO_ARGS), 0);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, FOO_ARGS), 4);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, FOO_ARGS, FOO_ARGS), 8);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, MAX_ARGS), 12);
    }
#[no_mangle]
unsafe extern "C" fn drop_first_arg_test(test: *mut kunit) {
    static void drop_first_arg_test(struct kunit *test)
    {
    let mut Y: c_int = -2, Z = -3, Q = -4;
    int a[] = { DROP_FIRST_ARG(FOO_ARGS) };
    KUNIT_EXPECT_EQ(test, DROP_FIRST_ARG(0, -1), -1);
    KUNIT_EXPECT_EQ(test, DROP_FIRST_ARG(DROP_FIRST_ARG(0, -1, -2)), -2);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, DROP_FIRST_ARG(FOO_ARGS)), 3);
    KUNIT_EXPECT_EQ(test, DROP_FIRST_ARG(DROP_FIRST_ARG(DROP_FIRST_ARG(FOO_ARGS))), -4);
    KUNIT_EXPECT_EQ(test, a[0], -2);
    KUNIT_EXPECT_EQ(test, a[1], -3);
    KUNIT_EXPECT_EQ(test, a[2], -4);

    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, foo), 3);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, bar), 2);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, buz), 1);
    KUNIT_EXPECT_STREQ(test, __stringify(buz), "Q");

    }
#[no_mangle]
unsafe extern "C" fn first_arg_test(test: *mut kunit) {
    static void first_arg_test(struct kunit *test)
    {
    let mut X: c_int = -1;
    int a[] = { FIRST_ARG(FOO_ARGS) };
    KUNIT_EXPECT_EQ(test, FIRST_ARG(-1, -2), -1);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, FIRST_ARG(FOO_ARGS)), 1);
    KUNIT_EXPECT_EQ(test, FIRST_ARG(FOO_ARGS), -1);
    KUNIT_EXPECT_EQ(test, a[0], -1);
    KUNIT_EXPECT_STREQ(test, __stringify(FIRST_ARG(FOO_ARGS)), "X");
    }
#[no_mangle]
unsafe extern "C" fn last_arg_test(test: *mut kunit) {
    static void last_arg_test(struct kunit *test)
    {
    let mut Q: c_int = -4;
    int a[] = { LAST_ARG(FOO_ARGS) };
    KUNIT_EXPECT_EQ(test, LAST_ARG(-1, -2), -2);
    KUNIT_EXPECT_EQ(test, CALL_ARGS(COUNT_ARGS, LAST_ARG(FOO_ARGS)), 1);
    KUNIT_EXPECT_EQ(test, LAST_ARG(FOO_ARGS), -4);
    KUNIT_EXPECT_EQ(test, a[0], -4);
    KUNIT_EXPECT_STREQ(test, __stringify(LAST_ARG(FOO_ARGS)), "Q");
    KUNIT_EXPECT_EQ(test, LAST_ARG(MAX_ARGS), -12);
    KUNIT_EXPECT_STREQ(test, __stringify(LAST_ARG(MAX_ARGS)), "-12");
    }
#[no_mangle]
unsafe extern "C" fn if_args_test(test: *mut kunit) {
    static void if_args_test(struct kunit *test)
    {
    let mut with_args: bool = true;
    let mut no_args: bool = false;
    enum { X = 100 };
    KUNIT_EXPECT_TRUE(test, IF_ARGS(true, false, FOO_ARGS));
    KUNIT_EXPECT_FALSE(test, IF_ARGS(true, false, NO_ARGS));
    KUNIT_EXPECT_TRUE(test, CONCATENATE(IF_ARGS(with, no, FOO_ARGS), _args));
    KUNIT_EXPECT_FALSE(test, CONCATENATE(IF_ARGS(with, no, NO_ARGS), _args));
    KUNIT_EXPECT_STREQ(test, __stringify(IF_ARGS(yes, no, FOO_ARGS)), "yes");
    KUNIT_EXPECT_STREQ(test, __stringify(IF_ARGS(yes, no, NO_ARGS)), "no");
    KUNIT_EXPECT_EQ(test, IF_ARGS(CALL_ARGS(COUNT_ARGS, FOO_ARGS), -1, FOO_ARGS), 4);
    KUNIT_EXPECT_EQ(test, IF_ARGS(CALL_ARGS(COUNT_ARGS, FOO_ARGS), -1, NO_ARGS), -1);
    KUNIT_EXPECT_EQ(test, IF_ARGS(CALL_ARGS(COUNT_ARGS, NO_ARGS), -1, FOO_ARGS), 0);
    KUNIT_EXPECT_EQ(test, IF_ARGS(CALL_ARGS(COUNT_ARGS, NO_ARGS), -1, NO_ARGS), -1);
    KUNIT_EXPECT_EQ(test,
    CALL_ARGS(FIRST_ARG,
    CALL_ARGS(CONCATENATE, IF_ARGS(FOO, MAX, FOO_ARGS), _ARGS)), X);
    KUNIT_EXPECT_EQ(test,
    CALL_ARGS(FIRST_ARG,
    CALL_ARGS(CONCATENATE, IF_ARGS(FOO, MAX, NO_ARGS), _ARGS)), -1);
    KUNIT_EXPECT_EQ(test,
    CALL_ARGS(COUNT_ARGS,
    CALL_ARGS(CONCATENATE, IF_ARGS(FOO, MAX, FOO_ARGS), _ARGS)), 4);
    KUNIT_EXPECT_EQ(test,
    CALL_ARGS(COUNT_ARGS,
    CALL_ARGS(CONCATENATE, IF_ARGS(FOO, MAX, NO_ARGS), _ARGS)), 12);
    }
    static struct kunit_case args_tests[] = {
    KUNIT_CASE(count_args_test),
    KUNIT_CASE(call_args_example),
    KUNIT_CASE(call_args_test),
    KUNIT_CASE(drop_first_arg_example),
    KUNIT_CASE(drop_first_arg_test),
    KUNIT_CASE(first_arg_example),
    KUNIT_CASE(first_arg_test),
    KUNIT_CASE(last_arg_example),
    KUNIT_CASE(last_arg_test),
    KUNIT_CASE(pick_arg_example),
    KUNIT_CASE(if_args_example),
    KUNIT_CASE(if_args_test),
    KUNIT_CASE(sep_comma_example),
    {}
    };
    static struct kunit_suite args_test_suite = {
    .name = "args",
    .test_cases = args_tests,
    };
    kunit_test_suite(args_test_suite);
