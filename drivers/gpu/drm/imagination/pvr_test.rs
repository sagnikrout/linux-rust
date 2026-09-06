//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imagination/pvr_test.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2025 Imagination Technologies Ltd.

#[no_mangle]
unsafe extern "C" fn decode_gpuid_string(test: *mut kunit) {
    static void decode_gpuid_string(struct kunit *test)
    {
    let mut bad_gpuid: pvr_gpu_id = { 0xdead, 0xbeef, 0xcafe, 0xface };
    let mut packed_bad_gpuid: u64 = pvr_gpu_id_to_packed_bvnc(&bad_gpuid);

    do {									\
    struct pvr_gpu_id _gpuid_out = bad_gpuid;			\
    int _err;							\
    _err = pvr_gpuid_decode_string(core::ptr::null_mut(), str_, &_gpuid_out);	\
    KUNIT_EXPECT_EQ(test, _err, err_);				\
    KUNIT_EXPECT_EQ(test,						\
    pvr_gpu_id_to_packed_bvnc(&_gpuid_out),		\
    value_);					\
    } while (0)

    GPUID_TEST_CASE(str_, 0, PVR_PACKED_BVNC(b_, v_, n_, c_))

    GPUID_TEST_CASE(str_, -EINVAL, packed_bad_gpuid)
    GPUID_TEST_CASE_OK("12.34.56.78", 12, 34, 56, 78);
    GPUID_TEST_CASE_OK("0.0.0.0", 0, 0, 0, 0);
    GPUID_TEST_CASE_INVAL("");
    GPUID_TEST_CASE_INVAL("42.foobar-invalid.gpuid.bvnc");
// String longer than PVR_GPUID_STRING_MAX_LENGTH.
    GPUID_TEST_CASE_INVAL("12.34.56.789012345678901234567890123456");
// Single value overflowing u16.
    GPUID_TEST_CASE_INVAL("12.34.56.999999");
// Wrong number of parts and/or dots.
    GPUID_TEST_CASE_INVAL("12.34.56.78.90");
    GPUID_TEST_CASE_INVAL("12.34.56..78");
    GPUID_TEST_CASE_INVAL("12.34..56");
    GPUID_TEST_CASE_INVAL("12.34.56");

    }
    static struct kunit_case pvr_tests_cases[] = {
    KUNIT_CASE(decode_gpuid_string),
    {},
    };
    static struct kunit_suite pvr_tests_suite = {
    .name = "pvr_tests",
    .test_cases = pvr_tests_cases,
    };
    kunit_test_suite(pvr_tests_suite);
    MODULE_AUTHOR("Imagination Technologies Ltd.");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("pvr kunit tests");
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
