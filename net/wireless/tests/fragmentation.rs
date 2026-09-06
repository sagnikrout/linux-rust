//! Automatically rewritten from C to Rust
//! Source: net/wireless/tests/fragmentation.c
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
// KUnit tests for element fragmentation
//
// Copyright (C) 2023-2024 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn defragment_0(test: *mut kunit) {
    static void defragment_0(struct kunit *test)
    {
    ssize_t ret;
    static const u8 input[] = {
    [0] = WLAN_EID_EXTENSION,
    [1] = 254,
    [2] = WLAN_EID_EXT_EHT_MULTI_LINK,
    [27] = 27,
    [123] = 123,
    [254 + 2] = WLAN_EID_FRAGMENT,
    [254 + 3] = 7,
    [254 + 3 + 7] = 0, /* for size */
    };
    u8 *data = kunit_kzalloc(test, sizeof(input), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, data);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    core::ptr::null_mut(), 0,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 253);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    data, ret,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 253);
    KUNIT_EXPECT_MEMEQ(test, data, input + 3, 253);
    }
#[no_mangle]
unsafe extern "C" fn defragment_1(test: *mut kunit) {
    static void defragment_1(struct kunit *test)
    {
    ssize_t ret;
    static const u8 input[] = {
    [0] = WLAN_EID_EXTENSION,
    [1] = 255,
    [2] = WLAN_EID_EXT_EHT_MULTI_LINK,
    [27] = 27,
    [123] = 123,
    [255 + 2] = WLAN_EID_FRAGMENT,
    [255 + 3] = 7,
    [255 + 3 + 1] = 0xaa,
    [255 + 3 + 8] = WLAN_EID_FRAGMENT, /* not used */
    [255 + 3 + 9] = 1,
    [255 + 3 + 10] = 0, /* for size */
    };
    u8 *data = kunit_kzalloc(test, sizeof(input), GFP_KERNEL);
    const struct element *elem;
    let mut count: c_int = 0;
    KUNIT_ASSERT_NOT_NULL(test, data);
    for_each_element(elem, input, sizeof(input))
    count++;
// check the elements are right
    KUNIT_ASSERT_EQ(test, count, 3);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    core::ptr::null_mut(), 0,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 254 + 7);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    data, ret,
    WLAN_EID_FRAGMENT);
// this means the last fragment was not used
    KUNIT_EXPECT_EQ(test, ret, 254 + 7);
    KUNIT_EXPECT_MEMEQ(test, data, input + 3, 254);
    KUNIT_EXPECT_MEMEQ(test, data + 254, input + 255 + 4, 7);
    }
#[no_mangle]
unsafe extern "C" fn defragment_2(test: *mut kunit) {
    static void defragment_2(struct kunit *test)
    {
    ssize_t ret;
    static const u8 input[] = {
    [0] = WLAN_EID_EXTENSION,
    [1] = 255,
    [2] = WLAN_EID_EXT_EHT_MULTI_LINK,
    [27] = 27,
    [123] = 123,
    [257 + 0] = WLAN_EID_FRAGMENT,
    [257 + 1] = 255,
    [257 + 20] = 0xaa,
    [2 * 257 + 0] = WLAN_EID_FRAGMENT,
    [2 * 257 + 1] = 1,
    [2 * 257 + 2] = 0xcc,
    [2 * 257 + 3] = WLAN_EID_FRAGMENT, /* not used */
    [2 * 257 + 4] = 1,
    [2 * 257 + 5] = 0, /* for size */
    };
    u8 *data = kunit_kzalloc(test, sizeof(input), GFP_KERNEL);
    const struct element *elem;
    let mut count: c_int = 0;
    KUNIT_ASSERT_NOT_NULL(test, data);
    for_each_element(elem, input, sizeof(input))
    count++;
// check the elements are right
    KUNIT_ASSERT_EQ(test, count, 4);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    core::ptr::null_mut(), 0,
    WLAN_EID_FRAGMENT);
// this means the last fragment was not used
    KUNIT_EXPECT_EQ(test, ret, 254 + 255 + 1);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    data, ret,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 254 + 255 + 1);
    KUNIT_EXPECT_MEMEQ(test, data, input + 3, 254);
    KUNIT_EXPECT_MEMEQ(test, data + 254, input + 257 + 2, 255);
    KUNIT_EXPECT_MEMEQ(test, data + 254 + 255, input + 2 * 257 + 2, 1);
    }
#[no_mangle]
unsafe extern "C" fn defragment_at_end(test: *mut kunit) {
    static void defragment_at_end(struct kunit *test)
    {
    ssize_t ret;
    static const u8 input[] = {
    [0] = WLAN_EID_EXTENSION,
    [1] = 255,
    [2] = WLAN_EID_EXT_EHT_MULTI_LINK,
    [27] = 27,
    [123] = 123,
    [255 + 2] = WLAN_EID_FRAGMENT,
    [255 + 3] = 7,
    [255 + 3 + 7] = 0, /* for size */
    };
    u8 *data = kunit_kzalloc(test, sizeof(input), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, data);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    core::ptr::null_mut(), 0,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 254 + 7);
    ret = cfg80211_defragment_element((void *)input,
    input, sizeof(input),
    data, ret,
    WLAN_EID_FRAGMENT);
    KUNIT_EXPECT_EQ(test, ret, 254 + 7);
    KUNIT_EXPECT_MEMEQ(test, data, input + 3, 254);
    KUNIT_EXPECT_MEMEQ(test, data + 254, input + 255 + 4, 7);
    }
    static struct kunit_case element_fragmentation_test_cases[] = {
    KUNIT_CASE(defragment_0),
    KUNIT_CASE(defragment_1),
    KUNIT_CASE(defragment_2),
    KUNIT_CASE(defragment_at_end),
    {}
    };
    static struct kunit_suite element_fragmentation = {
    .name = "cfg80211-element-defragmentation",
    .test_cases = element_fragmentation_test_cases,
    };
    kunit_test_suite(element_fragmentation);
