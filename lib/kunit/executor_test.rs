//! Automatically rewritten from C to Rust
//! Source: lib/kunit/executor_test.c
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
// KUnit test for the KUnit executor.
//
// Copyright (C) 2021, Google LLC.
// Author: Daniel Latypov <dlatypov@google.com>
//

    static void free_suite_set_at_end(struct kunit *test, const void *to_free);
    static struct kunit_suite *alloc_fake_suite(struct kunit *test,
    const char *suite_name,
    struct kunit_case *test_cases);
    static void dummy_test(struct kunit *test) {}
    static struct kunit_case dummy_test_cases[] = {
// .run_case is not important, just needs to be non-NULL
    { .name = "test1", .run_case = dummy_test },
    { .name = "test2", .run_case = dummy_test },
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_filter_test(test: *mut kunit) {
    static void parse_filter_test(struct kunit *test)
    {
    let mut filter: kunit_glob_filter = {core::ptr::null_mut(), core::ptr::null_mut()};
    kunit_parse_glob_filter(&filter, "suite");
    KUNIT_EXPECT_STREQ(test, filter.suite_glob, "suite");
    KUNIT_EXPECT_FALSE(test, filter.test_glob);
    kfree(filter.suite_glob);
    kfree(filter.test_glob);
    kunit_parse_glob_filter(&filter, "suite.test");
    KUNIT_EXPECT_STREQ(test, filter.suite_glob, "suite");
    KUNIT_EXPECT_STREQ(test, filter.test_glob, "test");
    kfree(filter.suite_glob);
    kfree(filter.test_glob);
    }
#[no_mangle]
unsafe extern "C" fn filter_suites_test(test: *mut kunit) {
    static void filter_suites_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[3] = {core::ptr::null_mut(), core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[2],
    };
    struct kunit_suite_set got;
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "suite1", dummy_test_cases);
    subsuite[1] = alloc_fake_suite(test, "suite2", dummy_test_cases);
// Want: suite1, suite2, NULL -> suite2, NULL
    got = kunit_filter_suites(&suite_set, "suite2", core::ptr::null_mut(), core::ptr::null_mut(), &err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got);
// Validate we just have suite2
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0]);
    KUNIT_EXPECT_STREQ(test, (const char *)got.start[0].name, "suite2");
// Contains one element (end is 1 past end)
    KUNIT_ASSERT_EQ(test, got.end - got.start, 1);
    }
#[no_mangle]
unsafe extern "C" fn filter_suites_test_glob_test(test: *mut kunit) {
    static void filter_suites_test_glob_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[3] = {core::ptr::null_mut(), core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[2],
    };
    struct kunit_suite_set got;
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "suite1", dummy_test_cases);
    subsuite[1] = alloc_fake_suite(test, "suite2", dummy_test_cases);
// Want: suite1, suite2, NULL -> suite2 (just test1), NULL
    got = kunit_filter_suites(&suite_set, "suite2.test2", core::ptr::null_mut(), core::ptr::null_mut(), &err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got);
// Validate we just have suite2
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0]);
    KUNIT_EXPECT_STREQ(test, (const char *)got.start[0].name, "suite2");
    KUNIT_ASSERT_EQ(test, got.end - got.start, 1);
// Now validate we just have test2
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0].test_cases);
    KUNIT_EXPECT_STREQ(test, (const char *)got.start[0].test_cases[0].name, "test2");
    KUNIT_EXPECT_FALSE(test, got.start[0].test_cases[1].name);
    }
#[no_mangle]
unsafe extern "C" fn filter_suites_to_empty_test(test: *mut kunit) {
    static void filter_suites_to_empty_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[3] = {core::ptr::null_mut(), core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[2],
    };
    struct kunit_suite_set got;
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "suite1", dummy_test_cases);
    subsuite[1] = alloc_fake_suite(test, "suite2", dummy_test_cases);
    got = kunit_filter_suites(&suite_set, "not_found", core::ptr::null_mut(), core::ptr::null_mut(), &err);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got); /* just in case */
    KUNIT_EXPECT_PTR_EQ_MSG(test, got.start, got.end,
    "should be empty to indicate no match");
    }
#[no_mangle]
unsafe extern "C" fn parse_filter_attr_test(test: *mut kunit) {
    static void parse_filter_attr_test(struct kunit *test)
    {
    int j, filter_count;
    struct kunit_attr_filter *parsed_filters;
    char filters[] = "speed>slow, module!=example", *filter = filters;
    let mut err: c_int = 0;
    filter_count = kunit_get_filter_count(filters);
    KUNIT_EXPECT_EQ(test, filter_count, 2);
    parsed_filters = kunit_kcalloc(test, filter_count, sizeof(*parsed_filters),
    GFP_KERNEL);
    for (j = 0; j < filter_count; j++) {
    parsed_filters[j] = kunit_next_attr_filter(&filter, &err);
    KUNIT_ASSERT_EQ_MSG(test, err, 0, "failed to parse filter from '%s'", filters);
    }
    KUNIT_EXPECT_STREQ(test, kunit_attr_filter_name(parsed_filters[0]), "speed");
    KUNIT_EXPECT_STREQ(test, parsed_filters[0].input, ">slow");
    KUNIT_EXPECT_STREQ(test, kunit_attr_filter_name(parsed_filters[1]), "module");
    KUNIT_EXPECT_STREQ(test, parsed_filters[1].input, "!=example");
    }
    static struct kunit_case dummy_attr_test_cases[] = {
// .run_case is not important, just needs to be non-NULL
    { .name = "slow", .run_case = dummy_test, .module_name = "dummy",
    .attr.speed = KUNIT_SPEED_SLOW },
    { .name = "normal", .run_case = dummy_test, .module_name = "dummy" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn filter_attr_test(test: *mut kunit) {
    static void filter_attr_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[3] = {core::ptr::null_mut(), core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[2],
    };
    struct kunit_suite_set got;
    char filter[] = "speed>slow";
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "normal_suite", dummy_attr_test_cases);
    subsuite[1] = alloc_fake_suite(test, "slow_suite", dummy_attr_test_cases);
    subsuite[1].attr.speed = KUNIT_SPEED_SLOW; // Set suite attribute
//
// Want: normal_suite(slow, normal), slow_suite(slow, normal),
// NULL -> normal_suite(normal), NULL
//
// The normal test in slow_suite is filtered out because the speed
// attribute is unset and thus, the filtering is based on the parent attribute
// of slow.
//
    got = kunit_filter_suites(&suite_set, core::ptr::null_mut(), filter, core::ptr::null_mut(), &err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got);
// Validate we just have normal_suite
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0]);
    KUNIT_EXPECT_STREQ(test, got.start[0].name, "normal_suite");
    KUNIT_ASSERT_EQ(test, got.end - got.start, 1);
// Now validate we just have normal test case
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0].test_cases);
    KUNIT_EXPECT_STREQ(test, got.start[0].test_cases[0].name, "normal");
    KUNIT_EXPECT_FALSE(test, got.start[0].test_cases[1].name);
    }
#[no_mangle]
unsafe extern "C" fn filter_attr_empty_test(test: *mut kunit) {
    static void filter_attr_empty_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[3] = {core::ptr::null_mut(), core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[2],
    };
    struct kunit_suite_set got;
    char filter[] = "module!=dummy";
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "suite1", dummy_attr_test_cases);
    subsuite[1] = alloc_fake_suite(test, "suite2", dummy_attr_test_cases);
    got = kunit_filter_suites(&suite_set, core::ptr::null_mut(), filter, core::ptr::null_mut(), &err);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got); /* just in case */
    KUNIT_EXPECT_PTR_EQ_MSG(test, got.start, got.end,
    "should be empty to indicate no match");
    }
#[no_mangle]
unsafe extern "C" fn filter_attr_skip_test(test: *mut kunit) {
    static void filter_attr_skip_test(struct kunit *test)
    {
    struct kunit_suite *subsuite[2] = {core::ptr::null_mut()};
    struct kunit_suite_set suite_set = {
    .start = subsuite, .end = &subsuite[1],
    };
    struct kunit_suite_set got;
    char filter[] = "speed>slow";
    let mut err: c_int = 0;
    subsuite[0] = alloc_fake_suite(test, "suite", dummy_attr_test_cases);
// Want: suite(slow, normal), NULL -> suite(slow with SKIP, normal), NULL
    got = kunit_filter_suites(&suite_set, core::ptr::null_mut(), filter, "skip", &err);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start);
    KUNIT_ASSERT_EQ(test, err, 0);
    free_suite_set_at_end(test, &got);
// Validate we have both the slow and normal test
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, got.start[0].test_cases);
    KUNIT_ASSERT_EQ(test, kunit_suite_num_test_cases(got.start[0]), 2);
    KUNIT_EXPECT_STREQ(test, got.start[0].test_cases[0].name, "slow");
    KUNIT_EXPECT_STREQ(test, got.start[0].test_cases[1].name, "normal");
// Now ensure slow is skipped and normal is not
    KUNIT_EXPECT_EQ(test, got.start[0].test_cases[0].status, KUNIT_SKIPPED);
    KUNIT_EXPECT_FALSE(test, got.start[0].test_cases[1].status);
    }
    static struct kunit_case executor_test_cases[] = {
    KUNIT_CASE(parse_filter_test),
    KUNIT_CASE(filter_suites_test),
    KUNIT_CASE(filter_suites_test_glob_test),
    KUNIT_CASE(filter_suites_to_empty_test),
    KUNIT_CASE(parse_filter_attr_test),
    KUNIT_CASE(filter_attr_test),
    KUNIT_CASE(filter_attr_empty_test),
    KUNIT_CASE(filter_attr_skip_test),
    {}
    };
    static struct kunit_suite executor_test_suite = {
    .name = "kunit_executor_test",
    .test_cases = executor_test_cases,
    };
    kunit_test_suites(&executor_test_suite);
// Test helpers
#[no_mangle]
unsafe extern "C" fn free_suite_set(suite_set: *mut c_void) {
    static void free_suite_set(void *suite_set)
    {
    kunit_free_suite_set(*(struct kunit_suite_set *)suite_set);
    kfree(suite_set);
    }
// Use the resource API to register a call to free_suite_set.
// Since we never actually use the resource, it's safe to use on const data.
//
#[no_mangle]
unsafe extern "C" fn free_suite_set_at_end(test: *mut kunit, to_free: *const c_void) {
    static void free_suite_set_at_end(struct kunit *test, const void *to_free)
    {
    struct kunit_suite_set *free;
    if (!((struct kunit_suite_set *)to_free).start)
    return;
    free = kzalloc_obj(struct kunit_suite_set);
// free = *(struct kunit_suite_set *)to_free;
    kunit_add_action(test, free_suite_set, (void *)free);
    }
    static struct kunit_suite *alloc_fake_suite(struct kunit *test,
    const char *suite_name,
    struct kunit_case *test_cases)
    {
    struct kunit_suite *suite;
// We normally never expect to allocate suites, hence the non-const cast.
    suite = kunit_kzalloc(test, sizeof(*suite), GFP_KERNEL);
    strscpy((char *)suite.name, suite_name, sizeof(suite.name));
    suite.test_cases = test_cases;
    return suite;
    }
