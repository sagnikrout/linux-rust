//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_rtp_tables_test.c
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
// Copyright © 2026 Intel Corporation
//

    static const void *table##_gen_params(struct kunit *test,				\
    const void *prev, char *desc)			\
    {											\
    typeof((table.entries)[0]) *__next = prev ?					\
    ((typeof(__next))prev) + 1 : (table.entries);				\
    if (__next - table.entries < table.n_entries) {					\
    scnprintf(desc, KUNIT_PARAM_DESC_SIZE, #table "/%s", __next.name);	\
    return __next;								\
    }										\
    return core::ptr::null_mut();									\
    }
#[no_mangle]
unsafe extern "C" fn xe_rtp_table_gt_test(test: *mut kunit) {
    static void xe_rtp_table_gt_test(struct kunit *test)
    {
    const struct xe_rtp_entry_sr *entry = test.param_value;
    for (int i = 0; i < entry.n_rules; i++) {
    KUNIT_EXPECT_TRUE(test,
    entry.rules[i].match_type != XE_RTP_MATCH_ENGINE_CLASS ||
    entry.flags & XE_RTP_ENTRY_FLAG_FOREACH_ENGINE);
    KUNIT_EXPECT_TRUE(test,
    entry.rules[i].match_type != XE_RTP_MATCH_NOT_ENGINE_CLASS ||
    entry.flags & XE_RTP_ENTRY_FLAG_FOREACH_ENGINE);
    }
    }
    RTP_TABLE_PARAM(gt_was);
    RTP_TABLE_PARAM(gt_tunings);
#[no_mangle]
unsafe extern "C" fn xe_rtp_table_oob_test(test: *mut kunit) {
    static void xe_rtp_table_oob_test(struct kunit *test)
    {
    const struct xe_rtp_entry *entry = test.param_value;
    for (int i = 0; i < entry.n_rules; i++) {
    let mut match_type: u8 = entry.rules[i].match_type;
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_ENGINE_CLASS);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_NOT_ENGINE_CLASS);
    }
    }
    RTP_TABLE_PARAM(oob_was);
#[no_mangle]
unsafe extern "C" fn xe_rtp_table_dev_oob_test(test: *mut kunit) {
    static void xe_rtp_table_dev_oob_test(struct kunit *test)
    {
    const struct xe_rtp_entry *entry = test.param_value;
    for (int i = 0; i < entry.n_rules; i++) {
    let mut match_type: u8 = entry.rules[i].match_type;
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_ENGINE_CLASS);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_NOT_ENGINE_CLASS);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_GRAPHICS_VERSION);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_GRAPHICS_VERSION_RANGE);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_GRAPHICS_VERSION_ANY_GT);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_GRAPHICS_STEP);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_MEDIA_VERSION);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_MEDIA_VERSION_RANGE);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_MEDIA_VERSION_ANY_GT);
    KUNIT_EXPECT_NE(test, match_type, XE_RTP_MATCH_MEDIA_STEP);
    }
    }
    RTP_TABLE_PARAM(device_oob_was);
#[no_mangle]
unsafe extern "C" fn xe_rtp_table_missing_upper_bound_test(test: *mut kunit) {
    static void xe_rtp_table_missing_upper_bound_test(struct kunit *test)
    {
    const struct xe_rtp_entry_sr *entry = test.param_value;
    for (int i = 0; i < entry.n_rules; i++) {
    let mut match_type: u8 = entry.rules[i].match_type;
    KUNIT_EXPECT_FALSE(test,
    match_type == XE_RTP_MATCH_GRAPHICS_VERSION_RANGE &&
    entry.rules[i].ver_end == XE_RTP_END_VERSION_UNDEFINED);
    KUNIT_EXPECT_FALSE(test,
    match_type == XE_RTP_MATCH_MEDIA_VERSION_RANGE &&
    entry.rules[i].ver_end == XE_RTP_END_VERSION_UNDEFINED);
    }
    }
    RTP_TABLE_PARAM(register_whitelist);
    static struct kunit_case xe_rtp_table_tests[] = {
    KUNIT_CASE_PARAM(xe_rtp_table_gt_test, gt_was_gen_params),
    KUNIT_CASE_PARAM(xe_rtp_table_gt_test, gt_tunings_gen_params),
    KUNIT_CASE_PARAM(xe_rtp_table_oob_test, oob_was_gen_params),
    KUNIT_CASE_PARAM(xe_rtp_table_dev_oob_test, device_oob_was_gen_params),
    KUNIT_CASE_PARAM(xe_rtp_table_missing_upper_bound_test,
    register_whitelist_gen_params),
    {}
    };
    static struct kunit_suite xe_rtp_tables_test_suite = {
    .name = "xe_rtp_tables_test",
    .test_cases = xe_rtp_table_tests,
    };
    kunit_test_suite(xe_rtp_tables_test_suite);
