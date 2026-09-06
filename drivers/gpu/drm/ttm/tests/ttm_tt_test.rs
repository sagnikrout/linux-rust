//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/tests/ttm_tt_test.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_tt_test_case {
    pub description: *const c_char,
    pub size: u32,
    pub extra_pages_num: u32,
}

    static const struct ttm_tt_test_case ttm_tt_init_basic_cases[] = {
    {
    .description = "Page-aligned size",
    .size = SZ_4K,
    },
    {
    .description = "Extra pages requested",
    .size = SZ_4K,
    .extra_pages_num = 1,
    },
    };
    static void ttm_tt_init_case_desc(const struct ttm_tt_test_case *t,
    char *desc)
    {
    strscpy(desc, t.description, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(ttm_tt_init_basic, ttm_tt_init_basic_cases,
    ttm_tt_init_case_desc);
#[no_mangle]
unsafe extern "C" fn ttm_tt_init_basic(test: *mut kunit) {
    static void ttm_tt_init_basic(struct kunit *test)
    {
    const struct ttm_tt_test_case *params = test.param_value;
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    let mut page_flags: u32 = TTM_TT_FLAG_ZERO_ALLOC;
    let mut caching: enum ttm_caching = ttm_cached;
    let mut extra_pages: u32 = params.extra_pages_num;
    let mut num_pages: c_int = params.size >> PAGE_SHIFT;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, params.size, core::ptr::null_mut());
    err = ttm_tt_init(tt, bo, page_flags, caching, extra_pages);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_EQ(test, tt.num_pages, num_pages + extra_pages);
    KUNIT_ASSERT_EQ(test, tt.page_flags, page_flags);
    KUNIT_ASSERT_EQ(test, tt.caching, caching);
    KUNIT_ASSERT_NULL(test, tt.dma_address);
    KUNIT_ASSERT_NULL(test, tt.swap_storage);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_init_misaligned(test: *mut kunit) {
    static void ttm_tt_init_misaligned(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    let mut caching: enum ttm_caching = ttm_cached;
    let mut size: u32 = SZ_8K;
    let mut num_pages: c_int = (size + SZ_4K) >> PAGE_SHIFT;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, size, core::ptr::null_mut());
// Make the object size misaligned
    bo.base.size += 1;
    err = ttm_tt_init(tt, bo, 0, caching, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_EQ(test, tt.num_pages, num_pages);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_fini_basic(test: *mut kunit) {
    static void ttm_tt_fini_basic(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    let mut caching: enum ttm_caching = ttm_cached;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_tt_init(tt, bo, 0, caching, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_NOT_NULL(test, tt.pages);
    ttm_tt_fini(tt);
    KUNIT_ASSERT_NULL(test, tt.pages);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_fini_sg(test: *mut kunit) {
    static void ttm_tt_fini_sg(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    let mut caching: enum ttm_caching = ttm_cached;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_sg_tt_init(tt, bo, 0, caching);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_NOT_NULL(test, tt.dma_address);
    ttm_tt_fini(tt);
    KUNIT_ASSERT_NULL(test, tt.dma_address);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_fini_shmem(test: *mut kunit) {
    static void ttm_tt_fini_shmem(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    struct file *shmem;
    let mut caching: enum ttm_caching = ttm_cached;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_tt_init(tt, bo, 0, caching, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    shmem = shmem_file_setup("ttm swap", BO_SIZE, EMPTY_VMA_FLAGS);
    tt.swap_storage = shmem;
    ttm_tt_fini(tt);
    KUNIT_ASSERT_NULL(test, tt.swap_storage);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_create_basic(test: *mut kunit) {
    static void ttm_tt_create_basic(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo.type = ttm_bo_type_device;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
    KUNIT_EXPECT_EQ(test, err, 0);
    KUNIT_EXPECT_NOT_NULL(test, bo.ttm);
// Free manually, as it was allocated outside of KUnit
    kfree(bo.ttm);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_create_invalid_bo_type(test: *mut kunit) {
    static void ttm_tt_create_invalid_bo_type(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo.type = ttm_bo_type_sg + 1;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
    KUNIT_EXPECT_EQ(test, err, -EINVAL);
    KUNIT_EXPECT_NULL(test, bo.ttm);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_create_ttm_exists(test: *mut kunit) {
    static void ttm_tt_create_ttm_exists(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    let mut caching: enum ttm_caching = ttm_cached;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_tt_init(tt, bo, 0, caching, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.ttm = tt;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
// Expect to keep the previous TTM
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_PTR_EQ(test, tt, bo.ttm);
    }
    static struct ttm_tt *ttm_tt_null_create(struct ttm_buffer_object *bo,
    u32 page_flags)
    {
    return core::ptr::null_mut();
    }
    static struct ttm_device_funcs ttm_dev_empty_funcs = {
    .ttm_tt_create = ttm_tt_null_create,
    };
#[no_mangle]
unsafe extern "C" fn ttm_tt_create_failed(test: *mut kunit) {
    static void ttm_tt_create_failed(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    struct ttm_buffer_object *bo;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
// Update ttm_device_funcs so we don't alloc ttm_tt
    devs.ttm_dev.funcs = &ttm_dev_empty_funcs;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_EQ(test, err, -ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_destroy_basic(test: *mut kunit) {
    static void ttm_tt_destroy_basic(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    struct ttm_buffer_object *bo;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_NOT_NULL(test, bo.ttm);
    ttm_tt_destroy(devs.ttm_dev, bo.ttm);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_populate_null_ttm(test: *mut kunit) {
    static void ttm_tt_populate_null_ttm(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    let mut ctx: ttm_operation_ctx = { };
    int err;
    err = ttm_tt_populate(devs.ttm_dev, core::ptr::null_mut(), &ctx);
    KUNIT_ASSERT_EQ(test, err, -EINVAL);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_populate_populated_ttm(test: *mut kunit) {
    static void ttm_tt_populate_populated_ttm(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    let mut ctx: ttm_operation_ctx = { };
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    struct page *populated_page;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_tt_init(tt, bo, 0, ttm_cached, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_tt_populate(devs.ttm_dev, tt, &ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    populated_page = *tt.pages;
    err = ttm_tt_populate(devs.ttm_dev, tt, &ctx);
    KUNIT_ASSERT_PTR_EQ(test, populated_page, *tt.pages);
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_unpopulate_basic(test: *mut kunit) {
    static void ttm_tt_unpopulate_basic(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    let mut ctx: ttm_operation_ctx = { };
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_tt_init(tt, bo, 0, ttm_cached, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_tt_populate(devs.ttm_dev, tt, &ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_TRUE(test, ttm_tt_is_populated(tt));
    ttm_tt_unpopulate(devs.ttm_dev, tt);
    KUNIT_ASSERT_FALSE(test, ttm_tt_is_populated(tt));
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_unpopulate_empty_ttm(test: *mut kunit) {
    static void ttm_tt_unpopulate_empty_ttm(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_tt_init(tt, bo, 0, ttm_cached, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    ttm_tt_unpopulate(devs.ttm_dev, tt);
// Expect graceful handling of unpopulated TTs
    }
#[no_mangle]
unsafe extern "C" fn ttm_tt_swapin_basic(test: *mut kunit) {
    static void ttm_tt_swapin_basic(struct kunit *test)
    {
    const struct ttm_test_devices *devs = test.priv;
    let mut expected_num_pages: c_int = BO_SIZE >> PAGE_SHIFT;
    let mut ctx: ttm_operation_ctx = { };
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    int err, num_pages;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_tt_init(tt, bo, 0, ttm_cached, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_tt_populate(devs.ttm_dev, tt, &ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_TRUE(test, ttm_tt_is_populated(tt));
    num_pages = ttm_tt_swapout(devs.ttm_dev, tt, GFP_KERNEL);
    KUNIT_ASSERT_EQ(test, num_pages, expected_num_pages);
    KUNIT_ASSERT_NOT_NULL(test, tt.swap_storage);
    KUNIT_ASSERT_TRUE(test, tt.page_flags & TTM_TT_FLAG_SWAPPED);
// Swapout depopulates TT, allocate pages and then swap them in
    err = ttm_pool_alloc(&devs.ttm_dev.pool, tt, &ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_tt_swapin(tt);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_NULL(test, tt.swap_storage);
    KUNIT_ASSERT_FALSE(test, tt.page_flags & TTM_TT_FLAG_SWAPPED);
    }
    static struct kunit_case ttm_tt_test_cases[] = {
    KUNIT_CASE_PARAM(ttm_tt_init_basic, ttm_tt_init_basic_gen_params),
    KUNIT_CASE(ttm_tt_init_misaligned),
    KUNIT_CASE(ttm_tt_fini_basic),
    KUNIT_CASE(ttm_tt_fini_sg),
    KUNIT_CASE(ttm_tt_fini_shmem),
    KUNIT_CASE(ttm_tt_create_basic),
    KUNIT_CASE(ttm_tt_create_invalid_bo_type),
    KUNIT_CASE(ttm_tt_create_ttm_exists),
    KUNIT_CASE(ttm_tt_create_failed),
    KUNIT_CASE(ttm_tt_destroy_basic),
    KUNIT_CASE(ttm_tt_populate_null_ttm),
    KUNIT_CASE(ttm_tt_populate_populated_ttm),
    KUNIT_CASE(ttm_tt_unpopulate_basic),
    KUNIT_CASE(ttm_tt_unpopulate_empty_ttm),
    KUNIT_CASE(ttm_tt_swapin_basic),
    {}
    };
    static struct kunit_suite ttm_tt_test_suite = {
    .name = "ttm_tt",
    .init = ttm_test_devices_all_init,
    .exit = ttm_test_devices_fini,
    .test_cases = ttm_tt_test_cases,
    };
    kunit_test_suites(&ttm_tt_test_suite);
    MODULE_DESCRIPTION("KUnit tests for ttm_tt APIs");
    MODULE_LICENSE("GPL and additional rights");
