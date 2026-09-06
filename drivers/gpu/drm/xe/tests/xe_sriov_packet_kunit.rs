//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_sriov_packet_kunit.c
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
// Copyright © 2026 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn sriov_packet_test_init(test: *mut kunit) -> c_int {
    static int sriov_packet_test_init(struct kunit *test)
    {
    struct xe_pci_fake_data fake = {
    .sriov_mode = XE_SRIOV_MODE_PF,
    .platform = XE_PANTHERLAKE, /* we need MEMIRQ */
    .subplatform = XE_SUBPLATFORM_NONE,
    .graphics_verx100 = 3000,
    .media_verx100 = 3000,
    };
    struct xe_device *xe;
    test.priv = &fake;
    xe_kunit_helper_xe_device_test_init(test);
    xe = test.priv;
// pretend we can support at least VF1
    xe.sriov.pf.device_total_vfs = 1;
    xe.sriov.pf.driver_max_vfs = 1;
    KUNIT_ASSERT_EQ(test, 0, xe_sriov_init(xe));
    KUNIT_ASSERT_TRUE(test, xe_sriov_pf_migration_supported(xe));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_descriptor_init(test: *mut kunit) {
    static void test_descriptor_init(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    struct xe_sriov_packet **desc;
// note: with lock held we should avoid KUNIT_ASSERT()
    guard(mutex)(pf_migration_mutex(xe, TEST_VF));
    KUNIT_EXPECT_EQ(test, 0, pf_descriptor_init(xe, TEST_VF));
    desc = pf_pick_descriptor(xe, TEST_VF);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, *desc);
    if (!*desc)
    return;
    KUNIT_EXPECT_NE(test, (*desc).hdr.version, 0);
    KUNIT_EXPECT_EQ(test, (*desc).hdr.version, XE_SRIOV_PACKET_SUPPORTED_VERSION);
    KUNIT_EXPECT_EQ(test, (*desc).hdr.type, XE_SRIOV_PACKET_TYPE_DESCRIPTOR);
    KUNIT_EXPECT_NE(test, (*desc).hdr.size, 0);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, (*desc).vaddr);
    if (!(*desc).vaddr)
    return;
    KUNIT_EXPECT_EQ(test, 0, xe_sriov_packet_process_descriptor(xe, TEST_VF, *desc));
    switch ((*desc).hdr.version) {
    case 1:
// v1 is KLV based
    KUNIT_EXPECT_TRUE(test, IS_ALIGNED((*desc).hdr.size, sizeof(u32)));
// v1 has at least DEVID and REVID KLVs
    KUNIT_EXPECT_LE(test, 2,
    xe_guc_klv_count((*desc).vaddr,
    (*desc).hdr.size / sizeof(u32)));
    break;
    default:
    kunit_mark_skipped(test, "no test code for version %u\n", (*desc).hdr.version);
    return;
    }
    }
    static struct kunit_case sriov_packet_test_cases[] = {
    KUNIT_CASE(test_descriptor_init),
    {}
    };
    static struct kunit_suite sriov_packet_suite = {
    .name = "sriov_packet",
    .test_cases = sriov_packet_test_cases,
    .init = sriov_packet_test_init,
    };
    kunit_test_suite(sriov_packet_suite);
