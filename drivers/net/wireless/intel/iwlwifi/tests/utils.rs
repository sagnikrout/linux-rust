//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/tests/utils.c
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
//
// KUnit tests for utilities
//
// Copyright (C) 2024-2025 Intel Corporation
//

    MODULE_IMPORT_NS("IWLWIFI");
    static const struct average_neg_db_case {
    const char *desc;
    u8 neg_dbm[22];
    s8 result;
    } average_neg_db_cases[] = {
    {
    .desc = "Smallest possible value, all filled",
    .neg_dbm = {
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128
    },
    .result = -128,
    },
    {
    .desc = "Biggest possible value, all filled",
    .neg_dbm = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
    },
    .result = 0,
    },
    {
    .desc = "Smallest possible value, partial filled",
    .neg_dbm = {
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff,
    },
    .result = -128,
    },
    {
    .desc = "Biggest possible value, partial filled",
    .neg_dbm = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff,
    },
    .result = 0,
    },
    {
    .desc = "Adding -80dBm to -75dBm until it is still rounded to -79dBm",
    .neg_dbm = {
    75, 80, 80, 80, 80, 80, 80, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 0xff, 0xff, 0xff,
    0xff, 0xff,
    },
    .result = -79,
    },
    {
    .desc = "Adding -80dBm to -75dBm until it is just rounded to -80dBm",
    .neg_dbm = {
    75, 80, 80, 80, 80, 80, 80, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 80, 0xff, 0xff,
    0xff, 0xff,
    },
    .result = -80,
    },
    };
    KUNIT_ARRAY_PARAM_DESC(average_neg_db, average_neg_db_cases, desc)
#[no_mangle]
unsafe extern "C" fn test_average_neg_db(test: *mut kunit) {
    static void test_average_neg_db(struct kunit *test)
    {
    const struct average_neg_db_case *params = test.param_value;
    u8 reversed[ARRAY_SIZE(params.neg_dbm)];
    int i;
// Test the values in the given order
    KUNIT_ASSERT_EQ(test,
    iwl_average_neg_dbm(params.neg_dbm,
    ARRAY_SIZE(params.neg_dbm)),
    params.result);
// Test in reverse order
    for (i = 0; i < ARRAY_SIZE(params.neg_dbm); i++)
    reversed[ARRAY_SIZE(params.neg_dbm) - i - 1] =
    params.neg_dbm[i];
    KUNIT_ASSERT_EQ(test,
    iwl_average_neg_dbm(reversed,
    ARRAY_SIZE(params.neg_dbm)),
    params.result);
    }
    static struct kunit_case average_db_case[] = {
    KUNIT_CASE_PARAM(test_average_neg_db, average_neg_db_gen_params),
    {}
    };
    static struct kunit_suite average_db = {
    .name = "iwl-average-db",
    .test_cases = average_db_case,
    };
    kunit_test_suite(average_db);
