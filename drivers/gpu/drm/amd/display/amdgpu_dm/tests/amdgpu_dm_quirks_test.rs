//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_quirks_test.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// KUnit tests for amdgpu_dm_quirks.c
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

// Tests for retrieve_dmi_info()
//
// Verify that retrieve_dmi_info() always initialises aux_hpd_discon_quirk to
// false, even when the caller had previously set it to true.
//
// dm_test_quirks_aux_hpd_discon_reset - Test Quirks aux hpd discon reset
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_quirks_aux_hpd_discon_reset(test: *mut kunit) {
    static void dm_test_quirks_aux_hpd_discon_reset(struct kunit *test)
    {
    struct amdgpu_display_manager *dm;
    dm = kunit_kzalloc(test, sizeof(*dm), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dm);
    dm.aux_hpd_discon_quirk = true;
    retrieve_dmi_info(dm);
//
// In a KUnit / UML environment no real DMI table is present, so
// dmi_check_system() returns 0 and retrieve_dmi_info() leaves the
// quirk at its initialised-to-false value.
//
    KUNIT_EXPECT_FALSE(test, dm.aux_hpd_discon_quirk);
    }
//
// Verify that retrieve_dmi_info() always initialises edp0_on_dp1_quirk to
// false, even when the caller had previously set it to true.
//
// dm_test_quirks_edp0_on_dp1_reset - Test Quirks edp0 on dp1 reset
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_quirks_edp0_on_dp1_reset(test: *mut kunit) {
    static void dm_test_quirks_edp0_on_dp1_reset(struct kunit *test)
    {
    struct amdgpu_display_manager *dm;
    dm = kunit_kzalloc(test, sizeof(*dm), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dm);
    dm.edp0_on_dp1_quirk = true;
    retrieve_dmi_info(dm);
    KUNIT_EXPECT_FALSE(test, dm.edp0_on_dp1_quirk);
    }
//
// Verify that when no DMI match is found both quirks remain false after a
// fresh (zero-initialised) dm is passed to retrieve_dmi_info().
//
// dm_test_quirks_no_dmi_match_both_false - Test Quirks no dmi match both false
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_quirks_no_dmi_match_both_false(test: *mut kunit) {
    static void dm_test_quirks_no_dmi_match_both_false(struct kunit *test)
    {
    struct amdgpu_display_manager *dm;
    dm = kunit_kzalloc(test, sizeof(*dm), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dm);
    retrieve_dmi_info(dm);
    KUNIT_EXPECT_FALSE(test, dm.aux_hpd_discon_quirk);
    KUNIT_EXPECT_FALSE(test, dm.edp0_on_dp1_quirk);
    }
// Tests for dm_should_disable_stutter()
//
// dm_test_should_disable_stutter_match - Test the quirk device matches
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_disable_stutter_match(test: *mut kunit) {
    static void dm_test_should_disable_stutter_match(struct kunit *test)
    {
    struct pci_dev *pdev;
    pdev = kunit_kzalloc(test, sizeof(*pdev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pdev);
    pdev.vendor = 0x1002;
    pdev.device = 0x15dd;
    pdev.subsystem_vendor = 0x1002;
    pdev.subsystem_device = 0x15dd;
    pdev.revision = 0xc8;
    KUNIT_EXPECT_TRUE(test, dm_should_disable_stutter(pdev));
    }
//
// dm_test_should_disable_stutter_no_match - Test a non-quirk device does not match
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_disable_stutter_no_match(test: *mut kunit) {
    static void dm_test_should_disable_stutter_no_match(struct kunit *test)
    {
    struct pci_dev *pdev;
    pdev = kunit_kzalloc(test, sizeof(*pdev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pdev);
    pdev.vendor = 0x1002;
    pdev.device = 0x1234;
    KUNIT_EXPECT_FALSE(test, dm_should_disable_stutter(pdev));
    }
//
// dm_test_should_disable_stutter_revision_differs - Test a partial match (revision) fails
// @test: The KUnit test context
//
#[no_mangle]
unsafe extern "C" fn dm_test_should_disable_stutter_revision_differs(test: *mut kunit) {
    static void dm_test_should_disable_stutter_revision_differs(struct kunit *test)
    {
    struct pci_dev *pdev;
    pdev = kunit_kzalloc(test, sizeof(*pdev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pdev);
// Everything matches the quirk except the revision
    pdev.vendor = 0x1002;
    pdev.device = 0x15dd;
    pdev.subsystem_vendor = 0x1002;
    pdev.subsystem_device = 0x15dd;
    pdev.revision = 0x00;
    KUNIT_EXPECT_FALSE(test, dm_should_disable_stutter(pdev));
    }
    static struct kunit_case amdgpu_dm_quirks_tests[] = {
// retrieve_dmi_info
    KUNIT_CASE(dm_test_quirks_aux_hpd_discon_reset),
    KUNIT_CASE(dm_test_quirks_edp0_on_dp1_reset),
    KUNIT_CASE(dm_test_quirks_no_dmi_match_both_false),
// dm_should_disable_stutter
    KUNIT_CASE(dm_test_should_disable_stutter_match),
    KUNIT_CASE(dm_test_should_disable_stutter_no_match),
    KUNIT_CASE(dm_test_should_disable_stutter_revision_differs),
    {}
    };
    static struct kunit_suite amdgpu_dm_quirks_test_suite = {
    .name = "amdgpu_dm_quirks",
    .test_cases = amdgpu_dm_quirks_tests,
    };
    kunit_test_suite(amdgpu_dm_quirks_test_suite);
    MODULE_AUTHOR("AMD");
    MODULE_DESCRIPTION("KUnit tests for amdgpu_dm_quirks");
    MODULE_LICENSE("Dual MIT/GPL");
