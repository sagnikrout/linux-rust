//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_exec_test.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exec_priv {
    pub dev: *mut device,
    pub drm: *mut drm_device,
}

#[no_mangle]
unsafe extern "C" fn drm_exec_test_init(test: *mut kunit) -> c_int {
    static int drm_exec_test_init(struct kunit *test)
    {
    struct drm_exec_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
    test.priv = priv;
    priv.dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.dev);
    priv.drm = __drm_kunit_helper_alloc_drm_device(test, priv.dev, sizeof(*priv.drm), 0,
    DRIVER_MODESET);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv.drm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sanitycheck(test: *mut kunit) {
    static void sanitycheck(struct kunit *test)
    {
    struct drm_exec exec;
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_fini(&exec);
    KUNIT_SUCCEED(test);
    }
#[no_mangle]
unsafe extern "C" fn test_lock(test: *mut kunit) {
    static void test_lock(struct kunit *test)
    {
    struct drm_exec_priv *priv = test.priv;
    let mut gobj: drm_gem_object = { };
    struct drm_exec exec;
    int ret;
    drm_gem_private_object_init(priv.drm, &gobj, PAGE_SIZE);
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec) {
    ret = drm_exec_lock_obj(&exec, &gobj);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    }
    drm_exec_fini(&exec);
    }
#[no_mangle]
unsafe extern "C" fn test_lock_unlock(test: *mut kunit) {
    static void test_lock_unlock(struct kunit *test)
    {
    struct drm_exec_priv *priv = test.priv;
    let mut gobj: drm_gem_object = { };
    struct drm_exec exec;
    int ret;
    drm_gem_private_object_init(priv.drm, &gobj, PAGE_SIZE);
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec) {
    ret = drm_exec_lock_obj(&exec, &gobj);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    drm_exec_unlock_obj(&exec, &gobj);
    ret = drm_exec_lock_obj(&exec, &gobj);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    }
    drm_exec_fini(&exec);
    }
#[no_mangle]
unsafe extern "C" fn test_duplicates(test: *mut kunit) {
    static void test_duplicates(struct kunit *test)
    {
    struct drm_exec_priv *priv = test.priv;
    let mut gobj: drm_gem_object = { };
    struct drm_exec exec;
    int ret;
    drm_gem_private_object_init(priv.drm, &gobj, PAGE_SIZE);
    drm_exec_init(&exec, DRM_EXEC_IGNORE_DUPLICATES, 0);
    drm_exec_until_all_locked(&exec) {
    ret = drm_exec_lock_obj(&exec, &gobj);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    ret = drm_exec_lock_obj(&exec, &gobj);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    }
    drm_exec_unlock_obj(&exec, &gobj);
    drm_exec_fini(&exec);
    }
#[no_mangle]
unsafe extern "C" fn test_prepare(test: *mut kunit) {
    static void test_prepare(struct kunit *test)
    {
    struct drm_exec_priv *priv = test.priv;
    let mut gobj: drm_gem_object = { };
    struct drm_exec exec;
    int ret;
    drm_gem_private_object_init(priv.drm, &gobj, PAGE_SIZE);
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec) {
    ret = drm_exec_prepare_obj(&exec, &gobj, 1);
    drm_exec_retry_on_contention(&exec);
    KUNIT_EXPECT_EQ(test, ret, 0);
    if (ret)
    break;
    }
    drm_exec_fini(&exec);
    drm_gem_private_object_fini(&gobj);
    }
#[no_mangle]
unsafe extern "C" fn test_prepare_array(test: *mut kunit) {
    static void test_prepare_array(struct kunit *test)
    {
    struct drm_exec_priv *priv = test.priv;
    struct drm_gem_object *gobj1;
    struct drm_gem_object *gobj2;
    struct drm_gem_object *array[] = {
    (gobj1 = kunit_kzalloc(test, sizeof(*gobj1), GFP_KERNEL)),
    (gobj2 = kunit_kzalloc(test, sizeof(*gobj2), GFP_KERNEL)),
    };
    struct drm_exec exec;
    int ret;
    if (!gobj1 || !gobj2) {
    KUNIT_FAIL(test, "Failed to allocate GEM objects.\n");
    return;
    }
    drm_gem_private_object_init(priv.drm, gobj1, PAGE_SIZE);
    drm_gem_private_object_init(priv.drm, gobj2, PAGE_SIZE);
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec)
    ret = drm_exec_prepare_array(&exec, array, ARRAY_SIZE(array),
    1);
    KUNIT_EXPECT_EQ(test, ret, 0);
    drm_exec_fini(&exec);
    drm_gem_private_object_fini(gobj1);
    drm_gem_private_object_fini(gobj2);
    }
#[no_mangle]
unsafe extern "C" fn test_multiple_loops(test: *mut kunit) {
    static void test_multiple_loops(struct kunit *test)
    {
    struct drm_exec exec;
    {
    __label__ drm_exec_retry;
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec)
    {
    break;
    }
    drm_exec_fini(&exec);
    }
    {
    __label__ drm_exec_retry;
    drm_exec_init(&exec, DRM_EXEC_INTERRUPTIBLE_WAIT, 0);
    drm_exec_until_all_locked(&exec)
    {
    break;
    }
    drm_exec_fini(&exec);
    }
    KUNIT_SUCCEED(test);
    }
    static struct kunit_case drm_exec_tests[] = {
    KUNIT_CASE(sanitycheck),
    KUNIT_CASE(test_lock),
    KUNIT_CASE(test_lock_unlock),
    KUNIT_CASE(test_duplicates),
    KUNIT_CASE(test_prepare),
    KUNIT_CASE(test_prepare_array),
    KUNIT_CASE(test_multiple_loops),
    {}
    };
    static struct kunit_suite drm_exec_test_suite = {
    .name = "drm_exec",
    .init = drm_exec_test_init,
    .test_cases = drm_exec_tests,
    };
    kunit_test_suite(drm_exec_test_suite);
    MODULE_AUTHOR("AMD");
    MODULE_DESCRIPTION("Kunit test for drm_exec functions");
    MODULE_LICENSE("GPL and additional rights");
