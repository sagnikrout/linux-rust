//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_sriov_pf_service_kunit.c
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2024-2025 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn pf_service_test_init(test: *mut kunit) -> c_int {
    static int pf_service_test_init(struct kunit *test)
    {
    struct xe_pci_fake_data fake = {
    .sriov_mode = XE_SRIOV_MODE_PF,
    .platform = XE_TIGERLAKE, /* some random platform */
    .subplatform = XE_SUBPLATFORM_NONE,
    };
    struct xe_device *xe;
    test.priv = &fake;
    xe_kunit_helper_xe_device_test_init(test);
    xe = test.priv;
    KUNIT_ASSERT_EQ(test, xe_sriov_init(xe), 0);
    xe_sriov_pf_service_init(xe);
//
// sanity check:
// - all supported platforms VF/PF ABI versions must be defined
// - base version can't be newer than latest
//
    KUNIT_ASSERT_NE(test, 0, xe.sriov.pf.service.version.base.major);
    KUNIT_ASSERT_NE(test, 0, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_LE(test, xe.sriov.pf.service.version.base.major,
    xe.sriov.pf.service.version.latest.major);
    if (xe.sriov.pf.service.version.base.major == xe.sriov.pf.service.version.latest.major)
    KUNIT_ASSERT_LE(test, xe.sriov.pf.service.version.base.minor,
    xe.sriov.pf.service.version.latest.minor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_any(test: *mut kunit) {
    static void pf_negotiate_any(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe, VF2PF_HANDSHAKE_MAJOR_ANY,
    VF2PF_HANDSHAKE_MINOR_ANY,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.latest.minor);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_base_match(test: *mut kunit) {
    static void pf_negotiate_base_match(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.base.major,
    xe.sriov.pf.service.version.base.minor,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.base.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.base.minor);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_base_newer(test: *mut kunit) {
    static void pf_negotiate_base_newer(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.base.major,
    xe.sriov.pf.service.version.base.minor + 1,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.base.major);
    KUNIT_ASSERT_GE(test, minor, xe.sriov.pf.service.version.base.minor);
    if (xe.sriov.pf.service.version.base.major == xe.sriov.pf.service.version.latest.major)
    KUNIT_ASSERT_LE(test, minor, xe.sriov.pf.service.version.latest.minor);
    else
    KUNIT_FAIL(test, "FIXME: don't know how to test multi-version yet!\n");
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_base_next(test: *mut kunit) {
    static void pf_negotiate_base_next(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.base.major + 1, 0,
    &major, &minor));
    KUNIT_ASSERT_GE(test, major, xe.sriov.pf.service.version.base.major);
    KUNIT_ASSERT_LE(test, major, xe.sriov.pf.service.version.latest.major);
    if (major == xe.sriov.pf.service.version.latest.major)
    KUNIT_ASSERT_LE(test, minor, xe.sriov.pf.service.version.latest.minor);
    else
    KUNIT_FAIL(test, "FIXME: don't know how to test multi-version yet!\n");
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_base_older(test: *mut kunit) {
    static void pf_negotiate_base_older(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    if (!xe.sriov.pf.service.version.base.minor)
    kunit_skip(test, "no older minor\n");
    KUNIT_ASSERT_NE(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.base.major,
    xe.sriov.pf.service.version.base.minor - 1,
    &major, &minor));
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_base_prev(test: *mut kunit) {
    static void pf_negotiate_base_prev(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_NE(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.base.major - 1, 1,
    &major, &minor));
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_latest_match(test: *mut kunit) {
    static void pf_negotiate_latest_match(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.latest.major,
    xe.sriov.pf.service.version.latest.minor,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.latest.minor);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_latest_newer(test: *mut kunit) {
    static void pf_negotiate_latest_newer(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.latest.major,
    xe.sriov.pf.service.version.latest.minor + 1,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.latest.minor);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_latest_next(test: *mut kunit) {
    static void pf_negotiate_latest_next(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.latest.major + 1, 0,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.latest.minor);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_latest_older(test: *mut kunit) {
    static void pf_negotiate_latest_older(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    if (!xe.sriov.pf.service.version.latest.minor)
    kunit_skip(test, "no older minor\n");
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.latest.major,
    xe.sriov.pf.service.version.latest.minor - 1,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major);
    KUNIT_ASSERT_EQ(test, minor, xe.sriov.pf.service.version.latest.minor - 1);
    }
#[no_mangle]
unsafe extern "C" fn pf_negotiate_latest_prev(test: *mut kunit) {
    static void pf_negotiate_latest_prev(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    u32 major, minor;
    if (xe.sriov.pf.service.version.base.major == xe.sriov.pf.service.version.latest.major)
    kunit_skip(test, "no prev major");
    KUNIT_ASSERT_EQ(test, 0,
    pf_negotiate_version(xe,
    xe.sriov.pf.service.version.latest.major - 1,
    xe.sriov.pf.service.version.base.minor + 1,
    &major, &minor));
    KUNIT_ASSERT_EQ(test, major, xe.sriov.pf.service.version.latest.major - 1);
    KUNIT_ASSERT_GE(test, major, xe.sriov.pf.service.version.base.major);
    }
    static struct kunit_case pf_service_test_cases[] = {
    KUNIT_CASE(pf_negotiate_any),
    KUNIT_CASE(pf_negotiate_base_match),
    KUNIT_CASE(pf_negotiate_base_newer),
    KUNIT_CASE(pf_negotiate_base_next),
    KUNIT_CASE(pf_negotiate_base_older),
    KUNIT_CASE(pf_negotiate_base_prev),
    KUNIT_CASE(pf_negotiate_latest_match),
    KUNIT_CASE(pf_negotiate_latest_newer),
    KUNIT_CASE(pf_negotiate_latest_next),
    KUNIT_CASE(pf_negotiate_latest_older),
    KUNIT_CASE(pf_negotiate_latest_prev),
    {}
    };
    static struct kunit_suite pf_service_suite = {
    .name = "pf_service",
    .test_cases = pf_service_test_cases,
    .init = pf_service_test_init,
    };
    kunit_test_suite(pf_service_suite);
