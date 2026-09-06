//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_guc_db_mgr_test.c
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
// Copyright © 2023 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn guc_dbm_test_init(test: *mut kunit) -> c_int {
    static int guc_dbm_test_init(struct kunit *test)
    {
    struct xe_guc_db_mgr *dbm;
    xe_kunit_helper_xe_device_test_init(test);
    dbm = &xe_device_get_gt(test.priv, 0).uc.guc.dbm;
    mutex_init(dbm_mutex(dbm));
    test.priv = dbm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_empty(test: *mut kunit) {
    static void test_empty(struct kunit *test)
    {
    struct xe_guc_db_mgr *dbm = test.priv;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, 0), 0);
    KUNIT_ASSERT_EQ(test, dbm.count, 0);
    mutex_lock(dbm_mutex(dbm));
    KUNIT_EXPECT_LT(test, xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    mutex_unlock(dbm_mutex(dbm));
    KUNIT_EXPECT_LT(test, xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    }
#[no_mangle]
unsafe extern "C" fn test_default(test: *mut kunit) {
    static void test_default(struct kunit *test)
    {
    struct xe_guc_db_mgr *dbm = test.priv;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, ~0), 0);
    KUNIT_ASSERT_EQ(test, dbm.count, GUC_NUM_DOORBELLS);
    }
    static const unsigned int guc_dbm_params[] = {
    GUC_NUM_DOORBELLS / 64,
    GUC_NUM_DOORBELLS / 32,
    GUC_NUM_DOORBELLS / 8,
    GUC_NUM_DOORBELLS,
    };
#[no_mangle]
unsafe extern "C" fn uint_param_get_desc(p: *const c_uint, desc: *mut c_char) {
    static void uint_param_get_desc(const unsigned int *p, char *desc)
    {
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "%u", *p);
    }
    KUNIT_ARRAY_PARAM(guc_dbm, guc_dbm_params, uint_param_get_desc);
#[no_mangle]
unsafe extern "C" fn test_size(test: *mut kunit) {
    static void test_size(struct kunit *test)
    {
    const unsigned int *p = test.param_value;
    struct xe_guc_db_mgr *dbm = test.priv;
    unsigned int n;
    int id;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, *p), 0);
    KUNIT_ASSERT_EQ(test, dbm.count, *p);
    mutex_lock(dbm_mutex(dbm));
    for (n = 0; n < *p; n++) {
    KUNIT_EXPECT_GE(test, id = xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    KUNIT_EXPECT_LT(test, id, dbm.count);
    }
    KUNIT_EXPECT_LT(test, xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    mutex_unlock(dbm_mutex(dbm));
    mutex_lock(dbm_mutex(dbm));
    for (n = 0; n < *p; n++)
    xe_guc_db_mgr_release_id_locked(dbm, n);
    mutex_unlock(dbm_mutex(dbm));
    }
#[no_mangle]
unsafe extern "C" fn test_reuse(test: *mut kunit) {
    static void test_reuse(struct kunit *test)
    {
    const unsigned int *p = test.param_value;
    struct xe_guc_db_mgr *dbm = test.priv;
    unsigned int n;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, *p), 0);
    mutex_lock(dbm_mutex(dbm));
    for (n = 0; n < *p; n++)
    KUNIT_EXPECT_GE(test, xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    KUNIT_EXPECT_LT(test, xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    mutex_unlock(dbm_mutex(dbm));
    mutex_lock(dbm_mutex(dbm));
    for (n = 0; n < *p; n++) {
    xe_guc_db_mgr_release_id_locked(dbm, n);
    KUNIT_EXPECT_EQ(test, xe_guc_db_mgr_reserve_id_locked(dbm), n);
    }
    KUNIT_EXPECT_LT(test, xe_guc_db_mgr_reserve_id_locked(dbm), 0);
    mutex_unlock(dbm_mutex(dbm));
    mutex_lock(dbm_mutex(dbm));
    for (n = 0; n < *p; n++)
    xe_guc_db_mgr_release_id_locked(dbm, n);
    mutex_unlock(dbm_mutex(dbm));
    }
#[no_mangle]
unsafe extern "C" fn test_range_overlap(test: *mut kunit) {
    static void test_range_overlap(struct kunit *test)
    {
    const unsigned int *p = test.param_value;
    struct xe_guc_db_mgr *dbm = test.priv;
    int id1, id2, id3;
    unsigned int n;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, ~0), 0);
    KUNIT_ASSERT_LE(test, *p, dbm.count);
    KUNIT_ASSERT_GE(test, id1 = xe_guc_db_mgr_reserve_range(dbm, *p, 0), 0);
    for (n = 0; n < dbm.count - *p; n++) {
    KUNIT_ASSERT_GE(test, id2 = xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    KUNIT_ASSERT_NE(test, id2, id1);
    KUNIT_ASSERT_NE_MSG(test, id2 < id1, id2 > id1 + *p - 1,
    "id1=%d id2=%d", id1, id2);
    }
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    xe_guc_db_mgr_release_range(dbm, 0, dbm.count);
    if (*p >= 1) {
    KUNIT_ASSERT_GE(test, id1 = xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    KUNIT_ASSERT_GE(test, id2 = xe_guc_db_mgr_reserve_range(dbm, *p - 1, 0), 0);
    KUNIT_ASSERT_NE(test, id2, id1);
    KUNIT_ASSERT_NE_MSG(test, id1 < id2, id1 > id2 + *p - 2,
    "id1=%d id2=%d", id1, id2);
    for (n = 0; n < dbm.count - *p; n++) {
    KUNIT_ASSERT_GE(test, id3 = xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    KUNIT_ASSERT_NE(test, id3, id1);
    KUNIT_ASSERT_NE(test, id3, id2);
    KUNIT_ASSERT_NE_MSG(test, id3 < id2, id3 > id2 + *p - 2,
    "id3=%d id2=%d", id3, id2);
    }
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    xe_guc_db_mgr_release_range(dbm, 0, dbm.count);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_range_compact(test: *mut kunit) {
    static void test_range_compact(struct kunit *test)
    {
    const unsigned int *p = test.param_value;
    struct xe_guc_db_mgr *dbm = test.priv;
    unsigned int n;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, ~0), 0);
    KUNIT_ASSERT_NE(test, *p, 0);
    KUNIT_ASSERT_LE(test, *p, dbm.count);
    if (dbm.count % *p)
    kunit_skip(test, "must be divisible");
    KUNIT_ASSERT_GE(test, xe_guc_db_mgr_reserve_range(dbm, *p, 0), 0);
    for (n = 1; n < dbm.count / *p; n++)
    KUNIT_ASSERT_GE(test, xe_guc_db_mgr_reserve_range(dbm, *p, 0), 0);
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, 1, 0), 0);
    xe_guc_db_mgr_release_range(dbm, 0, dbm.count);
    }
#[no_mangle]
unsafe extern "C" fn test_range_spare(test: *mut kunit) {
    static void test_range_spare(struct kunit *test)
    {
    const unsigned int *p = test.param_value;
    struct xe_guc_db_mgr *dbm = test.priv;
    int id;
    KUNIT_ASSERT_EQ(test, xe_guc_db_mgr_init(dbm, ~0), 0);
    KUNIT_ASSERT_LE(test, *p, dbm.count);
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, *p, dbm.count), 0);
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, *p, dbm.count - *p + 1), 0);
    KUNIT_ASSERT_EQ(test, id = xe_guc_db_mgr_reserve_range(dbm, *p, dbm.count - *p), 0);
    KUNIT_ASSERT_LT(test, xe_guc_db_mgr_reserve_range(dbm, 1, dbm.count - *p), 0);
    xe_guc_db_mgr_release_range(dbm, id, *p);
    }
    static struct kunit_case guc_dbm_test_cases[] = {
    KUNIT_CASE(test_empty),
    KUNIT_CASE(test_default),
    KUNIT_CASE_PARAM(test_size, guc_dbm_gen_params),
    KUNIT_CASE_PARAM(test_reuse, guc_dbm_gen_params),
    KUNIT_CASE_PARAM(test_range_overlap, guc_dbm_gen_params),
    KUNIT_CASE_PARAM(test_range_compact, guc_dbm_gen_params),
    KUNIT_CASE_PARAM(test_range_spare, guc_dbm_gen_params),
    {}
    };
    static struct kunit_suite guc_dbm_suite = {
    .name = "guc_dbm",
    .test_cases = guc_dbm_test_cases,
    .init = guc_dbm_test_init,
    };
    kunit_test_suites(&guc_dbm_suite);
