//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_wa_test.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright © 2023 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn xe_wa_test_init(test: *mut kunit) -> c_int {
    static int xe_wa_test_init(struct kunit *test)
    {
    const struct xe_pci_fake_data *param = test.param_value;
    let mut data: xe_pci_fake_data = *param;
    struct device *dev;
    struct xe_device *xe;
    struct xe_gt *gt;
    int id;
    int ret;
    dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
    xe = xe_kunit_helper_alloc_xe_device(test, dev);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, xe);
    test.priv = &data;
    ret = xe_pci_fake_device_init(xe);
    KUNIT_ASSERT_EQ(test, ret, 0);
// Needed for sanitize_mcr().
    for_each_gt(gt, xe, id) {
    xe_gt_mcr_init_early(gt);
    xe_gt_mmio_init(gt);
    }
// TODO: init hw engines for engine/LRC WAs
    xe.drm.dev = dev;
    test.priv = xe;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xe_wa_gt(test: *mut kunit) {
    static void xe_wa_gt(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    struct xe_gt *gt;
    int id;
    for_each_gt(gt, xe, id) {
    xe_reg_sr_init(&gt.reg_sr, "GT", xe);
    xe_wa_process_gt(gt);
    xe_tuning_process_gt(gt);
    KUNIT_EXPECT_EQ(test, gt.reg_sr.errors, 0);
    }
    }
    static struct kunit_case xe_wa_tests[] = {
    KUNIT_CASE_PARAM(xe_wa_gt, xe_pci_fake_data_gen_params),
    {}
    };
    static struct kunit_suite xe_rtp_test_suite = {
    .name = "xe_wa",
    .init = xe_wa_test_init,
    .test_cases = xe_wa_tests,
    };
    kunit_test_suite(xe_rtp_test_suite);
