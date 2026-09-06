//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/smb2maperror_test.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// KUnit tests of SMB2 maperror
//
// Copyright (C) 2025 KylinSoft Co., Ltd. All rights reserved.
// Author(s): ChenXiaoSong <chenxiaosong@kylinos.cn>
//

    static void
    test_cmp_map(struct kunit *test, const struct status_to_posix_error *expect)
    {
    const struct status_to_posix_error *result;
    result = smb2_get_err_map_test(expect.smb2_status);
    KUNIT_ASSERT_NOT_NULL(test, result);
    KUNIT_EXPECT_EQ(test, expect.smb2_status, result.smb2_status);
    KUNIT_EXPECT_EQ(test, expect.posix_error, result.posix_error);
    KUNIT_EXPECT_STREQ(test, expect.status_string, result.status_string);
    }
#[no_mangle]
unsafe extern "C" fn maperror_test_check_search(test: *mut kunit) {
    static void maperror_test_check_search(struct kunit *test)
    {
    unsigned int i;
    for (i = 0; i < smb2_error_map_num; i++)
    test_cmp_map(test, &smb2_error_map_table_test[i]);
    }
    static struct kunit_case maperror_test_cases[] = {
    KUNIT_CASE(maperror_test_check_search),
    {}
    };
    static struct kunit_suite maperror_suite = {
    .name = "smb2_maperror",
    .test_cases = maperror_test_cases,
    };
    kunit_test_suite(maperror_suite);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("KUnit tests of SMB2 maperror");
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
