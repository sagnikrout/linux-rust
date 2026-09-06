//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/tests/ttm_device_test.c
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
pub struct ttm_device_test_case {
    pub description: *const c_char,
    pub alloc_flags: c_uint,
    pub pools_init_expected: bool,
}

#[no_mangle]
unsafe extern "C" fn ttm_device_init_basic(test: *mut kunit) {
    static void ttm_device_init_basic(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_device *ttm_dev;
    struct ttm_resource_manager *ttm_sys_man;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_EXPECT_PTR_EQ(test, ttm_dev.funcs, &ttm_dev_funcs);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev.wq);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev.man_drv[TTM_PL_SYSTEM]);
    ttm_sys_man = &ttm_dev.sysman;
    KUNIT_ASSERT_NOT_NULL(test, ttm_sys_man);
    KUNIT_EXPECT_TRUE(test, ttm_sys_man.use_tt);
    KUNIT_EXPECT_TRUE(test, ttm_sys_man.use_type);
    KUNIT_ASSERT_NOT_NULL(test, ttm_sys_man.func);
    KUNIT_EXPECT_PTR_EQ(test, ttm_dev.dev_mapping,
    priv.drm.anon_inode.i_mapping);
    ttm_device_fini(ttm_dev);
    }
#[no_mangle]
unsafe extern "C" fn ttm_device_init_multiple(test: *mut kunit) {
    static void ttm_device_init_multiple(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_device *ttm_devs;
    unsigned int i, num_dev = 3;
    int err;
    ttm_devs = kunit_kcalloc(test, num_dev, sizeof(*ttm_devs), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_devs);
    for (i = 0; i < num_dev; i++) {
    err = ttm_device_kunit_init(priv, &ttm_devs[i], 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_EXPECT_PTR_EQ(test, ttm_devs[i].dev_mapping,
    priv.drm.anon_inode.i_mapping);
    KUNIT_ASSERT_NOT_NULL(test, ttm_devs[i].wq);
    KUNIT_EXPECT_PTR_EQ(test, ttm_devs[i].funcs, &ttm_dev_funcs);
    KUNIT_ASSERT_NOT_NULL(test, ttm_devs[i].man_drv[TTM_PL_SYSTEM]);
    }
    KUNIT_ASSERT_EQ(test, list_count_nodes(&ttm_devs[0].device_list), num_dev);
    for (i = 0; i < num_dev; i++)
    ttm_device_fini(&ttm_devs[i]);
    }
#[no_mangle]
unsafe extern "C" fn ttm_device_fini_basic(test: *mut kunit) {
    static void ttm_device_fini_basic(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_device *ttm_dev;
    struct ttm_resource_manager *man;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    man = ttm_manager_type(ttm_dev, TTM_PL_SYSTEM);
    KUNIT_ASSERT_NOT_NULL(test, man);
    ttm_device_fini(ttm_dev);
    KUNIT_ASSERT_FALSE(test, man.use_type);
    KUNIT_ASSERT_TRUE(test, list_empty(&man.lru[0]));
    KUNIT_ASSERT_NULL(test, ttm_dev.man_drv[TTM_PL_SYSTEM]);
    }
#[no_mangle]
unsafe extern "C" fn ttm_device_init_no_vma_man(test: *mut kunit) {
    static void ttm_device_init_no_vma_man(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct drm_device *drm = priv.drm;
    struct ttm_device *ttm_dev;
    struct drm_vma_offset_manager *vma_man;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
// Let's pretend there's no VMA manager allocated
    vma_man = drm.vma_offset_manager;
    drm.vma_offset_manager = core::ptr::null_mut();
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_EXPECT_EQ(test, err, -EINVAL);
// Bring the manager back for a graceful cleanup
    drm.vma_offset_manager = vma_man;
    }
    static const struct ttm_device_test_case ttm_device_cases[] = {
    {
    .description = "No DMA allocations, no DMA32 required",
    .pools_init_expected = false,
    },
    {
    .description = "DMA allocations, DMA32 required",
    .alloc_flags = TTM_ALLOCATION_POOL_USE_DMA_ALLOC |
    TTM_ALLOCATION_POOL_USE_DMA32,
    .pools_init_expected = true,
    },
    {
    .description = "No DMA allocations, DMA32 required",
    .alloc_flags = TTM_ALLOCATION_POOL_USE_DMA32,
    .pools_init_expected = false,
    },
    {
    .description = "DMA allocations, no DMA32 required",
    .alloc_flags = TTM_ALLOCATION_POOL_USE_DMA_ALLOC,
    .pools_init_expected = true,
    },
    };
#[no_mangle]
unsafe extern "C" fn ttm_device_case_desc(t: *const ttm_device_test_case, desc: *mut c_char) {
    static void ttm_device_case_desc(const struct ttm_device_test_case *t, char *desc)
    {
    strscpy(desc, t.description, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(ttm_device, ttm_device_cases, ttm_device_case_desc);
#[no_mangle]
unsafe extern "C" fn ttm_device_init_pools(test: *mut kunit) {
    static void ttm_device_init_pools(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    const struct ttm_device_test_case *params = test.param_value;
    struct ttm_device *ttm_dev;
    struct ttm_pool *pool;
    struct ttm_pool_type pt;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, params.alloc_flags);
    KUNIT_ASSERT_EQ(test, err, 0);
    pool = &ttm_dev.pool;
    KUNIT_ASSERT_NOT_NULL(test, pool);
    KUNIT_EXPECT_PTR_EQ(test, pool.dev, priv.dev);
    KUNIT_EXPECT_EQ(test, pool.alloc_flags, params.alloc_flags);
    if (params.pools_init_expected) {
    for (int i = 0; i < TTM_NUM_CACHING_TYPES; ++i) {
    for (int j = 0; j < NR_PAGE_ORDERS; ++j) {
    pt = pool.caching[i].orders[j];
    KUNIT_EXPECT_PTR_EQ(test, pt.pool, pool);
    KUNIT_EXPECT_EQ(test, pt.caching, i);
    KUNIT_EXPECT_EQ(test, pt.order, j);
    if (ttm_pool_uses_dma_alloc(pool))
    KUNIT_ASSERT_FALSE(test,
    list_lru_count(&pt.pages));
    }
    }
    }
    ttm_device_fini(ttm_dev);
    }
    static struct kunit_case ttm_device_test_cases[] = {
    KUNIT_CASE(ttm_device_init_basic),
    KUNIT_CASE(ttm_device_init_multiple),
    KUNIT_CASE(ttm_device_fini_basic),
    KUNIT_CASE(ttm_device_init_no_vma_man),
    KUNIT_CASE_PARAM(ttm_device_init_pools, ttm_device_gen_params),
    {}
    };
    static struct kunit_suite ttm_device_test_suite = {
    .name = "ttm_device",
    .init = ttm_test_devices_init,
    .exit = ttm_test_devices_fini,
    .test_cases = ttm_device_test_cases,
    };
    kunit_test_suites(&ttm_device_test_suite);
    MODULE_DESCRIPTION("KUnit tests for ttm_device APIs");
    MODULE_LICENSE("GPL and additional rights");
