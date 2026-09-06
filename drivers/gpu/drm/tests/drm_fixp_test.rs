//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_fixp_test.c
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


// SPDX-License-Identifier: MIT
//
// Copyright 2022 Advanced Micro Devices, Inc.
//

#[no_mangle]
unsafe extern "C" fn drm_test_sm2fixp(test: *mut kunit) {
    static void drm_test_sm2fixp(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, 0x7fffffffffffffffll, ((1ull << 63) - 1));
// 1
    KUNIT_EXPECT_EQ(test, drm_int2fixp(1), drm_sm2fixp(1ull << DRM_FIXED_POINT));
// -1
    KUNIT_EXPECT_EQ(test, drm_int2fixp(-1),
    drm_sm2fixp((1ull << 63) | (1ull << DRM_FIXED_POINT)));
// 0.5
    KUNIT_EXPECT_EQ(test, drm_fixp_from_fraction(1, 2),
    drm_sm2fixp(1ull << (DRM_FIXED_POINT - 1)));
// -0.5
    KUNIT_EXPECT_EQ(test, drm_fixp_from_fraction(-1, 2),
    drm_sm2fixp((1ull << 63) | (1ull << (DRM_FIXED_POINT - 1))));
    }
#[no_mangle]
unsafe extern "C" fn drm_test_int2fixp(test: *mut kunit) {
    static void drm_test_int2fixp(struct kunit *test)
    {
// 1
    KUNIT_EXPECT_EQ(test, 1ll << 32, drm_int2fixp(1));
// -1
    KUNIT_EXPECT_EQ(test, -(1ll << 32), drm_int2fixp(-1));
// 1 + (-1) = 0
    KUNIT_EXPECT_EQ(test, 0, drm_int2fixp(1) + drm_int2fixp(-1));
// 1 / 2
    KUNIT_EXPECT_EQ(test, 1ll << 31, drm_fixp_from_fraction(1, 2));
// -0.5
    KUNIT_EXPECT_EQ(test, -(1ll << 31), drm_fixp_from_fraction(-1, 2));
// (1 / 2) + (-1) = 0.5
    KUNIT_EXPECT_EQ(test, 1ll << 31, drm_fixp_from_fraction(-1, 2) + drm_int2fixp(1));
// (1 / 2) - 1) = 0.5
    KUNIT_EXPECT_EQ(test, -(1ll << 31), drm_fixp_from_fraction(1, 2) + drm_int2fixp(-1));
// (1 / 2) - 1) = 0.5
    KUNIT_EXPECT_EQ(test, -(1ll << 31), drm_fixp_from_fraction(1, 2) - drm_int2fixp(1));
    }
    static struct kunit_case drm_fixp_tests[] = {
    KUNIT_CASE(drm_test_int2fixp),
    KUNIT_CASE(drm_test_sm2fixp),
    { }
    };
    static struct kunit_suite drm_fixp_test_suite = {
    .name = "drm_fixp",
    .test_cases = drm_fixp_tests,
    };
    kunit_test_suite(drm_fixp_test_suite);
    MODULE_AUTHOR("AMD");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("Unit tests for drm_fixed.h");
