//! Automatically rewritten from C to Rust
//! Source: drivers/platform/wmi/tests/string_kunit.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// KUnit test for the ACPI-WMI string conversion code.
//
// Copyright (C) 2025 Armin Wolf <W_Armin@gmx.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_string_param {
    pub name: *const c_char,
    pub wmi_string: *const wmi_string,
//
// Remember that using sizeof() on a struct wmi_string will
// always return a size of two bytes due to the flexible
// array member!
//
    pub wmi_string_length: usize,
    pub utf8_string: *const u8,
    pub utf8_string_length: usize,
}

pub const TEST_WMI_STRING_LENGTH: c_int = 12;
    static const struct wmi_string test_wmi_string = {
    .length = cpu_to_le16(10),
    .chars = {
    cpu_to_le16(u'T'),
    cpu_to_le16(u'E'),
    cpu_to_le16(u'S'),
    cpu_to_le16(u'T'),
    cpu_to_le16(u'\0'),
    },
    };
    static const u8 test_utf8_string[] = "TEST";
pub const SPECIAL_WMI_STRING_LENGTH: c_int = 14;
    static const struct wmi_string special_wmi_string = {
    .length = cpu_to_le16(12),
    .chars = {
    cpu_to_le16(u'Ä'),
    cpu_to_le16(u'Ö'),
    cpu_to_le16(u'Ü'),
    cpu_to_le16(u'ß'),
    cpu_to_le16(u'€'),
    cpu_to_le16(u'\0'),
    },
    };
    static const u8 special_utf8_string[] = "ÄÖÜß€";
pub const MULTI_POINT_WMI_STRING_LENGTH: c_int = 12;
    static const struct wmi_string multi_point_wmi_string = {
    .length = cpu_to_le16(10),
    .chars = {
    cpu_to_le16(u'K'),
// 🐧
    cpu_to_le16(0xD83D),
    cpu_to_le16(0xDC27),
    cpu_to_le16(u'!'),
    cpu_to_le16(u'\0'),
    },
    };
    static const u8 multi_point_utf8_string[] = "K🐧!";
pub const PADDED_TEST_WMI_STRING_LENGTH: c_int = 14;
    static const struct wmi_string padded_test_wmi_string = {
    .length = cpu_to_le16(12),
    .chars = {
    cpu_to_le16(u'T'),
    cpu_to_le16(u'E'),
    cpu_to_le16(u'S'),
    cpu_to_le16(u'T'),
    cpu_to_le16(u'\0'),
    cpu_to_le16(u'\0'),
    },
    };
    static const u8 padded_test_utf8_string[] = "TEST\0";
pub const OVERSIZED_TEST_WMI_STRING_LENGTH: c_int = 14;
    static const struct wmi_string oversized_test_wmi_string = {
    .length = cpu_to_le16(8),
    .chars = {
    cpu_to_le16(u'T'),
    cpu_to_le16(u'E'),
    cpu_to_le16(u'S'),
    cpu_to_le16(u'T'),
    cpu_to_le16(u'!'),
    cpu_to_le16(u'\0'),
    },
    };
    static const u8 oversized_test_utf8_string[] = "TEST!";
pub const INVALID_TEST_WMI_STRING_LENGTH: c_int = 14;
    static const struct wmi_string invalid_test_wmi_string = {
    .length = cpu_to_le16(12),
    .chars = {
    cpu_to_le16(u'T'),
// 🐧, with low surrogate missing
    cpu_to_le16(0xD83D),
    cpu_to_le16(u'E'),
    cpu_to_le16(u'S'),
    cpu_to_le16(u'T'),
    cpu_to_le16(u'\0'),
    },
    };
// We have to split the string here to end the hex escape sequence
    static const u8 invalid_test_utf8_string[] = "T" "\xF0\x9F" "EST";
    static const struct wmi_string_param wmi_string_params_array[] = {
    {
    .name = "ascii_string",
    .wmi_string = &test_wmi_string,
    .wmi_string_length = TEST_WMI_STRING_LENGTH,
    .utf8_string = test_utf8_string,
    .utf8_string_length = sizeof(test_utf8_string),
    },
    {
    .name = "special_string",
    .wmi_string = &special_wmi_string,
    .wmi_string_length = SPECIAL_WMI_STRING_LENGTH,
    .utf8_string = special_utf8_string,
    .utf8_string_length = sizeof(special_utf8_string),
    },
    {
    .name = "multi_point_string",
    .wmi_string = &multi_point_wmi_string,
    .wmi_string_length = MULTI_POINT_WMI_STRING_LENGTH,
    .utf8_string = multi_point_utf8_string,
    .utf8_string_length = sizeof(multi_point_utf8_string),
    },
    };
#[no_mangle]
unsafe extern "C" fn wmi_string_param_get_desc(param: *const wmi_string_param, desc: *mut c_char) {
    static void wmi_string_param_get_desc(const struct wmi_string_param *param, char *desc)
    {
    strscpy(desc, param.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(wmi_string, wmi_string_params_array, wmi_string_param_get_desc);
#[no_mangle]
unsafe extern "C" fn wmi_string_to_utf8s_test(test: *mut kunit) {
    static void wmi_string_to_utf8s_test(struct kunit *test)
    {
    const struct wmi_string_param *param = test.param_value;
    ssize_t ret;
    u8 *result;
    result = kunit_kzalloc(test, param.utf8_string_length, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, result);
    ret = wmi_string_to_utf8s(param.wmi_string, result, param.utf8_string_length);
    KUNIT_EXPECT_EQ(test, ret, param.utf8_string_length - 1);
    KUNIT_EXPECT_MEMEQ(test, result, param.utf8_string, param.utf8_string_length);
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_from_utf8s_test(test: *mut kunit) {
    static void wmi_string_from_utf8s_test(struct kunit *test)
    {
    const struct wmi_string_param *param = test.param_value;
    struct wmi_string *result;
    size_t max_chars;
    ssize_t ret;
    max_chars = (param.wmi_string_length - sizeof(*result)) / 2;
    result = kunit_kzalloc(test, param.wmi_string_length, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, result);
    ret = wmi_string_from_utf8s(result, max_chars, param.utf8_string,
    param.utf8_string_length);
    KUNIT_EXPECT_EQ(test, ret, max_chars - 1);
    KUNIT_EXPECT_MEMEQ(test, result, param.wmi_string, param.wmi_string_length);
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_to_utf8s_padded_test(test: *mut kunit) {
    static void wmi_string_to_utf8s_padded_test(struct kunit *test)
    {
    u8 result[sizeof(padded_test_utf8_string)];
    ssize_t ret;
    ret = wmi_string_to_utf8s(&padded_test_wmi_string, result, sizeof(result));
    KUNIT_EXPECT_EQ(test, ret, sizeof(test_utf8_string) - 1);
    KUNIT_EXPECT_MEMEQ(test, result, test_utf8_string, sizeof(test_utf8_string));
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_from_utf8s_padded_test(test: *mut kunit) {
    static void wmi_string_from_utf8s_padded_test(struct kunit *test)
    {
    struct wmi_string *result;
    size_t max_chars;
    ssize_t ret;
    max_chars = (PADDED_TEST_WMI_STRING_LENGTH - sizeof(*result)) / 2;
    result = kunit_kzalloc(test, PADDED_TEST_WMI_STRING_LENGTH, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, result);
    ret = wmi_string_from_utf8s(result, max_chars, padded_test_utf8_string,
    sizeof(padded_test_utf8_string));
    KUNIT_EXPECT_EQ(test, ret, sizeof(test_utf8_string) - 1);
    KUNIT_EXPECT_MEMEQ(test, result, &test_wmi_string, sizeof(test_wmi_string));
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_to_utf8s_oversized_test(test: *mut kunit) {
    static void wmi_string_to_utf8s_oversized_test(struct kunit *test)
    {
    u8 result[sizeof(oversized_test_utf8_string)];
    ssize_t ret;
    ret = wmi_string_to_utf8s(&oversized_test_wmi_string, result, sizeof(result));
    KUNIT_EXPECT_EQ(test, ret, sizeof(test_utf8_string) - 1);
    KUNIT_EXPECT_MEMEQ(test, result, test_utf8_string, sizeof(test_utf8_string));
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_from_utf8s_oversized_test(test: *mut kunit) {
    static void wmi_string_from_utf8s_oversized_test(struct kunit *test)
    {
    struct wmi_string *result;
    size_t max_chars;
    ssize_t ret;
    max_chars = (TEST_WMI_STRING_LENGTH - sizeof(*result)) / 2;
    result = kunit_kzalloc(test, TEST_WMI_STRING_LENGTH, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, result);
    ret = wmi_string_from_utf8s(result, max_chars, oversized_test_utf8_string,
    sizeof(oversized_test_utf8_string));
    KUNIT_EXPECT_EQ(test, ret, sizeof(test_utf8_string) - 1);
    KUNIT_EXPECT_MEMEQ(test, result, &test_wmi_string, sizeof(test_wmi_string));
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_to_utf8s_invalid_test(test: *mut kunit) {
    static void wmi_string_to_utf8s_invalid_test(struct kunit *test)
    {
    u8 result[sizeof(invalid_test_utf8_string)];
    ssize_t ret;
    ret = wmi_string_to_utf8s(&invalid_test_wmi_string, result, sizeof(result));
    KUNIT_EXPECT_EQ(test, ret, sizeof(test_utf8_string) - 1);
    KUNIT_EXPECT_MEMEQ(test, result, test_utf8_string, sizeof(test_utf8_string));
    }
#[no_mangle]
unsafe extern "C" fn wmi_string_from_utf8s_invalid_test(test: *mut kunit) {
    static void wmi_string_from_utf8s_invalid_test(struct kunit *test)
    {
    struct wmi_string *result;
    size_t max_chars;
    ssize_t ret;
    max_chars = (INVALID_TEST_WMI_STRING_LENGTH - sizeof(*result)) / 2;
    result = kunit_kzalloc(test, INVALID_TEST_WMI_STRING_LENGTH, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, result);
    ret = wmi_string_from_utf8s(result, max_chars, invalid_test_utf8_string,
    sizeof(invalid_test_utf8_string));
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    }
    static struct kunit_case wmi_string_test_cases[] = {
    KUNIT_CASE_PARAM(wmi_string_to_utf8s_test, wmi_string_gen_params),
    KUNIT_CASE_PARAM(wmi_string_from_utf8s_test, wmi_string_gen_params),
    KUNIT_CASE(wmi_string_to_utf8s_padded_test),
    KUNIT_CASE(wmi_string_from_utf8s_padded_test),
    KUNIT_CASE(wmi_string_to_utf8s_oversized_test),
    KUNIT_CASE(wmi_string_from_utf8s_oversized_test),
    KUNIT_CASE(wmi_string_to_utf8s_invalid_test),
    KUNIT_CASE(wmi_string_from_utf8s_invalid_test),
    {}
    };
    static struct kunit_suite wmi_string_test_suite = {
    .name = "wmi_string",
    .test_cases = wmi_string_test_cases,
    };
    kunit_test_suite(wmi_string_test_suite);
    MODULE_AUTHOR("Armin Wolf <W_Armin@gmx.de>");
    MODULE_DESCRIPTION("KUnit test for the ACPI-WMI string conversion code");
    MODULE_LICENSE("GPL");
