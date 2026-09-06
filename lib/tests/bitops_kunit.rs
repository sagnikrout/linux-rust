//! Automatically rewritten from C to Rust
//! Source: lib/tests/bitops_kunit.c
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
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2026 Ryota Sakamoto <sakamo.ryota@gmail.com>
//

// use an enum because that's the most common BITMAP usage
    enum bitops_fun {
    BITOPS_4 = 4,
    BITOPS_7 = 7,
    BITOPS_11 = 11,
    BITOPS_31 = 31,
    BITOPS_88 = 88,
    BITOPS_LENGTH = 256
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitops_test_case {
    pub str: *const c_char,
    pub nr: c_long,
}

    static struct bitops_test_case bitops_cases[] = {
    {
    .str = "BITOPS_4",
    .nr = BITOPS_4,
    },
    {
    .str = "BITOPS_7",
    .nr = BITOPS_7,
    },
    {
    .str = "BITOPS_11",
    .nr = BITOPS_11,
    },
    {
    .str = "BITOPS_31",
    .nr = BITOPS_31,
    },
    {
    .str = "BITOPS_88",
    .nr = BITOPS_88,
    },
    };
    KUNIT_ARRAY_PARAM_DESC(bitops, bitops_cases, str);
#[no_mangle]
unsafe extern "C" fn test_set_bit_clear_bit(test: *mut kunit) {
    static void test_set_bit_clear_bit(struct kunit *test)
    {
    const struct bitops_test_case *params = test.param_value;
    DECLARE_BITMAP(bitmap, BITOPS_LENGTH);
    int bit_set;
    bitmap_zero(bitmap, BITOPS_LENGTH);
    set_bit(params.nr, bitmap);
    KUNIT_EXPECT_TRUE(test, test_bit(params.nr, bitmap));
    clear_bit(params.nr, bitmap);
    KUNIT_EXPECT_FALSE(test, test_bit(params.nr, bitmap));
    bit_set = find_first_bit(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_EQ(test, bit_set, BITOPS_LENGTH);
    }
#[no_mangle]
unsafe extern "C" fn test_change_bit(test: *mut kunit) {
    static void test_change_bit(struct kunit *test)
    {
    const struct bitops_test_case *params = test.param_value;
    DECLARE_BITMAP(bitmap, BITOPS_LENGTH);
    int bit_set;
    bitmap_zero(bitmap, BITOPS_LENGTH);
    change_bit(params.nr, bitmap);
    KUNIT_EXPECT_TRUE(test, test_bit(params.nr, bitmap));
    change_bit(params.nr, bitmap);
    KUNIT_EXPECT_FALSE(test, test_bit(params.nr, bitmap));
    bit_set = find_first_bit(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_EQ(test, bit_set, BITOPS_LENGTH);
    }
#[no_mangle]
unsafe extern "C" fn test_test_and_set_bit_test_and_clear_bit(test: *mut kunit) {
    static void test_test_and_set_bit_test_and_clear_bit(struct kunit *test)
    {
    const struct bitops_test_case *params = test.param_value;
    DECLARE_BITMAP(bitmap, BITOPS_LENGTH);
    int bit_set;
    bitmap_zero(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_FALSE(test, test_and_set_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_and_set_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_and_clear_bit(params.nr, bitmap));
    KUNIT_EXPECT_FALSE(test, test_bit(params.nr, bitmap));
    KUNIT_EXPECT_FALSE(test, test_and_clear_bit(params.nr, bitmap));
    KUNIT_EXPECT_FALSE(test, test_bit(params.nr, bitmap));
    bit_set = find_first_bit(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_EQ(test, bit_set, BITOPS_LENGTH);
    }
#[no_mangle]
unsafe extern "C" fn test_test_and_change_bit(test: *mut kunit) {
    static void test_test_and_change_bit(struct kunit *test)
    {
    const struct bitops_test_case *params = test.param_value;
    DECLARE_BITMAP(bitmap, BITOPS_LENGTH);
    int bit_set;
    bitmap_zero(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_FALSE(test, test_and_change_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_bit(params.nr, bitmap));
    KUNIT_EXPECT_TRUE(test, test_and_change_bit(params.nr, bitmap));
    KUNIT_EXPECT_FALSE(test, test_bit(params.nr, bitmap));
    bit_set = find_first_bit(bitmap, BITOPS_LENGTH);
    KUNIT_EXPECT_EQ(test, bit_set, BITOPS_LENGTH);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct order_test_case {
    pub str: *const c_char,
    pub count: c_uint,
    pub expected: c_int,
}

    static struct order_test_case order_test_cases[] = {
    {"0x00000003", 0x00000003,  2},
    {"0x00000004", 0x00000004,  2},
    {"0x00001fff", 0x00001fff, 13},
    {"0x00002000", 0x00002000, 13},
    {"0x50000000", 0x50000000, 31},
    {"0x80000000", 0x80000000, 31},
    {"0x80003000", 0x80003000, 32},
    };
    KUNIT_ARRAY_PARAM_DESC(order, order_test_cases, str);
#[no_mangle]
unsafe extern "C" fn test_get_count_order(test: *mut kunit) {
    static void test_get_count_order(struct kunit *test)
    {
    const struct order_test_case *params = test.param_value;
    KUNIT_EXPECT_EQ(test, get_count_order(params.count), params.expected);
    KUNIT_EXPECT_EQ(test, get_count_order_long(params.count), params.expected);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct order_long_test_case {
    pub str: *const c_char,
    pub count: c_ulong,
    pub expected: c_int,
}

    static struct order_long_test_case order_long_test_cases[] = {
    {"0x0000000300000000", 0x0000000300000000, 34},
    {"0x0000000400000000", 0x0000000400000000, 34},
    {"0x00001fff00000000", 0x00001fff00000000, 45},
    {"0x0000200000000000", 0x0000200000000000, 45},
    {"0x5000000000000000", 0x5000000000000000, 63},
    {"0x8000000000000000", 0x8000000000000000, 63},
    {"0x8000300000000000", 0x8000300000000000, 64},
    };
    KUNIT_ARRAY_PARAM_DESC(order_long, order_long_test_cases, str);
#[no_mangle]
unsafe extern "C" fn test_get_count_order_long(test: *mut kunit) {
    static void test_get_count_order_long(struct kunit *test)
    {
    const struct order_long_test_case *params = test.param_value;
    KUNIT_EXPECT_EQ(test, get_count_order_long(params.count), params.expected);
    }

    static struct kunit_case bitops_test_cases[] = {
    KUNIT_CASE_PARAM(test_set_bit_clear_bit, bitops_gen_params),
    KUNIT_CASE_PARAM(test_change_bit, bitops_gen_params),
    KUNIT_CASE_PARAM(test_test_and_set_bit_test_and_clear_bit, bitops_gen_params),
    KUNIT_CASE_PARAM(test_test_and_change_bit, bitops_gen_params),
    KUNIT_CASE_PARAM(test_get_count_order, order_gen_params),

    KUNIT_CASE_PARAM(test_get_count_order_long, order_long_gen_params),

    {},
    };
    static struct kunit_suite bitops_test_suite = {
    .name = "bitops",
    .test_cases = bitops_test_cases,
    };
    kunit_test_suite(bitops_test_suite);
    MODULE_AUTHOR("Jesse Brandeburg <jesse.brandeburg@intel.com>");
    MODULE_AUTHOR("Wei Yang <richard.weiyang@gmail.com>");
    MODULE_AUTHOR("Ryota Sakamoto <sakamo.ryota@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Bit testing module");
