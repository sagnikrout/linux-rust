//! Automatically rewritten from C to Rust
//! Source: drivers/base/power/qos-test.c
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
// Copyright 2019 NXP
//

// Basic test for aggregating two "min" requests
#[no_mangle]
unsafe extern "C" fn freq_qos_test_min(test: *mut kunit) {
    static void freq_qos_test_min(struct kunit *test)
    {
    struct freq_constraints	qos;
    struct freq_qos_request	req1, req2;
    int ret;
    freq_constraints_init(&qos);
    memset(&req1, 0, sizeof(req1));
    memset(&req2, 0, sizeof(req2));
    ret = freq_qos_add_request(&qos, &req1, FREQ_QOS_MIN, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    ret = freq_qos_add_request(&qos, &req2, FREQ_QOS_MIN, 2000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 2000);
    ret = freq_qos_remove_request(&req2);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 1000);
    ret = freq_qos_remove_request(&req1);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN),
    FREQ_QOS_MIN_DEFAULT_VALUE);
    }
// Test that requests for MAX_DEFAULT_VALUE have no effect
#[no_mangle]
unsafe extern "C" fn freq_qos_test_maxdef(test: *mut kunit) {
    static void freq_qos_test_maxdef(struct kunit *test)
    {
    struct freq_constraints	qos;
    struct freq_qos_request	req1, req2;
    int ret;
    freq_constraints_init(&qos);
    memset(&req1, 0, sizeof(req1));
    memset(&req2, 0, sizeof(req2));
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX),
    FREQ_QOS_MAX_DEFAULT_VALUE);
    ret = freq_qos_add_request(&qos, &req1, FREQ_QOS_MAX,
    FREQ_QOS_MAX_DEFAULT_VALUE);
    KUNIT_EXPECT_EQ(test, ret, 0);
    ret = freq_qos_add_request(&qos, &req2, FREQ_QOS_MAX,
    FREQ_QOS_MAX_DEFAULT_VALUE);
    KUNIT_EXPECT_EQ(test, ret, 0);
// Add max 1000
    ret = freq_qos_update_request(&req1, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 1000);
// Add max 2000, no impact
    ret = freq_qos_update_request(&req2, 2000);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 1000);
// Remove max 1000, new max 2000
    ret = freq_qos_remove_request(&req1);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MAX), 2000);
    }
//
// Test that a freq_qos_request can be added again after removal
//
// This issue was solved by commit 05ff1ba412fd ("PM: QoS: Invalidate frequency
// QoS requests after removal")
//
#[no_mangle]
unsafe extern "C" fn freq_qos_test_readd(test: *mut kunit) {
    static void freq_qos_test_readd(struct kunit *test)
    {
    struct freq_constraints	qos;
    struct freq_qos_request	req;
    int ret;
    freq_constraints_init(&qos);
    memset(&req, 0, sizeof(req));
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN),
    FREQ_QOS_MIN_DEFAULT_VALUE);
// Add
    ret = freq_qos_add_request(&qos, &req, FREQ_QOS_MIN, 1000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 1000);
// Remove
    ret = freq_qos_remove_request(&req);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN),
    FREQ_QOS_MIN_DEFAULT_VALUE);
// Add again
    ret = freq_qos_add_request(&qos, &req, FREQ_QOS_MIN, 2000);
    KUNIT_EXPECT_EQ(test, ret, 1);
    KUNIT_EXPECT_EQ(test, freq_qos_read_value(&qos, FREQ_QOS_MIN), 2000);
    }
    static struct kunit_case pm_qos_test_cases[] = {
    KUNIT_CASE(freq_qos_test_min),
    KUNIT_CASE(freq_qos_test_maxdef),
    KUNIT_CASE(freq_qos_test_readd),
    {},
    };
    static struct kunit_suite pm_qos_test_module = {
    .name = "qos-kunit-test",
    .test_cases = pm_qos_test_cases,
    };
    kunit_test_suites(&pm_qos_test_module);
