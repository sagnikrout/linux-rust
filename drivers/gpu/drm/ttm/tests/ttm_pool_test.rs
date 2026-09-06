//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/tests/ttm_pool_test.c
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
pub struct ttm_pool_test_case {
    pub description: *const c_char,
    pub order: c_uint,
    pub alloc_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_pool_test_priv {
    pub devs: *mut ttm_test_devices,
// Used to create mock ttm_tts
    pub mock_bo: *mut ttm_buffer_object,
}

    static struct ttm_operation_ctx simple_ctx = {
    .interruptible = true,
    .no_wait_gpu = false,
    };
#[no_mangle]
unsafe extern "C" fn ttm_pool_test_init(test: *mut kunit) -> c_int {
    static int ttm_pool_test_init(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, priv);
    priv.devs = ttm_test_devices_basic(test);
    test.priv = priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_test_fini(test: *mut kunit) {
    static void ttm_pool_test_fini(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    ttm_test_devices_put(test, priv.devs);
    }
    static struct ttm_tt *ttm_tt_kunit_init(struct kunit *test,
    u32 page_flags,
    enum ttm_caching caching,
    size_t size)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_tt *tt;
    int err;
    bo = ttm_bo_kunit_init(test, priv.devs, size, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_NULL(test, bo);
    priv.mock_bo = bo;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_tt_init(tt, priv.mock_bo, page_flags, caching, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    return tt;
    }
    static struct ttm_pool *ttm_pool_pre_populated(struct kunit *test,
    size_t size,
    enum ttm_caching caching)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_test_devices *devs = priv.devs;
    struct ttm_pool *pool;
    struct ttm_tt *tt;
    int err;
    tt = ttm_tt_kunit_init(test, 0, caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    pool = kunit_kzalloc(test, sizeof(*pool), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pool);
    ttm_pool_init(pool, devs.dev, NUMA_NO_NODE, TTM_ALLOCATION_POOL_USE_DMA_ALLOC);
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    return pool;
    }
    static const struct ttm_pool_test_case ttm_pool_basic_cases[] = {
    {
    .description = "One page",
    .order = 0,
    },
    {
    .description = "More than one page",
    .order = 2,
    },
    {
    .description = "Above the allocation limit",
    .order = MAX_PAGE_ORDER + 1,
    },
    {
    .description = "One page, with coherent DMA mappings enabled",
    .order = 0,
    .alloc_flags = TTM_ALLOCATION_POOL_USE_DMA_ALLOC,
    },
    {
    .description = "Above the allocation limit, with coherent DMA mappings enabled",
    .order = MAX_PAGE_ORDER + 1,
    .alloc_flags = TTM_ALLOCATION_POOL_USE_DMA_ALLOC,
    },
    };
    static void ttm_pool_alloc_case_desc(const struct ttm_pool_test_case *t,
    char *desc)
    {
    strscpy(desc, t.description, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(ttm_pool_alloc_basic, ttm_pool_basic_cases,
    ttm_pool_alloc_case_desc);
#[no_mangle]
unsafe extern "C" fn ttm_pool_alloc_basic(test: *mut kunit) {
    static void ttm_pool_alloc_basic(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_test_devices *devs = priv.devs;
    const struct ttm_pool_test_case *params = test.param_value;
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct page *fst_page, *last_page;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut expected_num_pages: c_uint = 1 << params.order;
    let mut size: usize = expected_num_pages * PAGE_SIZE;
    int err;
    tt = ttm_tt_kunit_init(test, 0, caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    pool = kunit_kzalloc(test, sizeof(*pool), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pool);
    ttm_pool_init(pool, devs.dev, NUMA_NO_NODE, params.alloc_flags);
    KUNIT_ASSERT_PTR_EQ(test, pool.dev, devs.dev);
    KUNIT_ASSERT_EQ(test, pool.nid, NUMA_NO_NODE);
    KUNIT_ASSERT_EQ(test, pool.alloc_flags, params.alloc_flags);
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_EQ(test, tt.num_pages, expected_num_pages);
    fst_page = tt.pages[0];
    last_page = tt.pages[tt.num_pages - 1];
    if (params.order <= MAX_PAGE_ORDER) {
    if (ttm_pool_uses_dma_alloc(pool)) {
    KUNIT_ASSERT_NOT_NULL(test, (void *)fst_page.private);
    KUNIT_ASSERT_NOT_NULL(test, (void *)last_page.private);
    } else {
    KUNIT_ASSERT_EQ(test, fst_page.private, params.order);
    }
    } else {
    if (ttm_pool_uses_dma_alloc(pool)) {
    KUNIT_ASSERT_NOT_NULL(test, (void *)fst_page.private);
    KUNIT_ASSERT_NULL(test, (void *)last_page.private);
    } else {
//
// We expect to alloc one big block, followed by
// order 0 blocks
//
    KUNIT_ASSERT_EQ(test, fst_page.private,
    min_t(unsigned int, MAX_PAGE_ORDER,
    params.order));
    KUNIT_ASSERT_EQ(test, last_page.private, 0);
    }
    }
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_alloc_basic_dma_addr(test: *mut kunit) {
    static void ttm_pool_alloc_basic_dma_addr(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_test_devices *devs = priv.devs;
    const struct ttm_pool_test_case *params = test.param_value;
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct ttm_buffer_object *bo;
    dma_addr_t dma1, dma2;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut expected_num_pages: c_uint = 1 << params.order;
    let mut size: usize = expected_num_pages * PAGE_SIZE;
    int err;
    tt = kunit_kzalloc(test, sizeof(*tt), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    bo = ttm_bo_kunit_init(test, devs, size, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_NULL(test, bo);
    err = ttm_sg_tt_init(tt, bo, 0, caching);
    KUNIT_ASSERT_EQ(test, err, 0);
    pool = kunit_kzalloc(test, sizeof(*pool), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pool);
    ttm_pool_init(pool, devs.dev, NUMA_NO_NODE, TTM_ALLOCATION_POOL_USE_DMA_ALLOC);
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_EQ(test, tt.num_pages, expected_num_pages);
    dma1 = tt.dma_address[0];
    dma2 = tt.dma_address[tt.num_pages - 1];
    KUNIT_ASSERT_NOT_NULL(test, (void *)(uintptr_t)dma1);
    KUNIT_ASSERT_NOT_NULL(test, (void *)(uintptr_t)dma2);
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_alloc_order_caching_match(test: *mut kunit) {
    static void ttm_pool_alloc_order_caching_match(struct kunit *test)
    {
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct ttm_pool_type *pt;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut order: c_uint = 0;
    let mut size: usize = PAGE_SIZE;
    int err;
    pool = ttm_pool_pre_populated(test, size, caching);
    pt = &pool.caching[caching].orders[order];
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt.pages));
    tt = ttm_tt_kunit_init(test, 0, caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_TRUE(test, !list_lru_count(&pt.pages));
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_alloc_caching_mismatch(test: *mut kunit) {
    static void ttm_pool_alloc_caching_mismatch(struct kunit *test)
    {
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct ttm_pool_type *pt_pool, *pt_tt;
    let mut tt_caching: enum ttm_caching = ttm_uncached;
    let mut pool_caching: enum ttm_caching = ttm_cached;
    let mut size: usize = PAGE_SIZE;
    let mut order: c_uint = 0;
    int err;
    pool = ttm_pool_pre_populated(test, size, pool_caching);
    pt_pool = &pool.caching[pool_caching].orders[order];
    pt_tt = &pool.caching[tt_caching].orders[order];
    tt = ttm_tt_kunit_init(test, 0, tt_caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_pool.pages));
    KUNIT_ASSERT_TRUE(test, !list_lru_count(&pt_tt.pages));
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_pool.pages));
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_tt.pages));
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_alloc_order_mismatch(test: *mut kunit) {
    static void ttm_pool_alloc_order_mismatch(struct kunit *test)
    {
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct ttm_pool_type *pt_pool, *pt_tt;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut order: c_uint = 2;
    let mut fst_size: usize = (1 << order) * PAGE_SIZE;
    let mut snd_size: usize = PAGE_SIZE;
    int err;
    pool = ttm_pool_pre_populated(test, fst_size, caching);
    pt_pool = &pool.caching[caching].orders[order];
    pt_tt = &pool.caching[caching].orders[0];
    tt = ttm_tt_kunit_init(test, 0, caching, snd_size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_pool.pages));
    KUNIT_ASSERT_TRUE(test, !list_lru_count(&pt_tt.pages));
    err = ttm_pool_alloc(pool, tt, &simple_ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_pool.pages));
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt_tt.pages));
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_free_dma_alloc(test: *mut kunit) {
    static void ttm_pool_free_dma_alloc(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_test_devices *devs = priv.devs;
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    struct ttm_pool_type *pt;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut order: c_uint = 2;
    let mut size: usize = (1 << order) * PAGE_SIZE;
    tt = ttm_tt_kunit_init(test, 0, caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    pool = kunit_kzalloc(test, sizeof(*pool), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pool);
    ttm_pool_init(pool, devs.dev, NUMA_NO_NODE, TTM_ALLOCATION_POOL_USE_DMA_ALLOC);
    ttm_pool_alloc(pool, tt, &simple_ctx);
    pt = &pool.caching[caching].orders[order];
    KUNIT_ASSERT_TRUE(test, !list_lru_count(&pt.pages));
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt.pages));
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_free_no_dma_alloc(test: *mut kunit) {
    static void ttm_pool_free_no_dma_alloc(struct kunit *test)
    {
    struct ttm_pool_test_priv *priv = test.priv;
    struct ttm_test_devices *devs = priv.devs;
    struct ttm_tt *tt;
    struct ttm_pool *pool;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut order: c_uint = 2;
    let mut size: usize = (1 << order) * PAGE_SIZE;
    tt = ttm_tt_kunit_init(test, 0, caching, size);
    KUNIT_ASSERT_NOT_NULL(test, tt);
    pool = kunit_kzalloc(test, sizeof(*pool), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, pool);
    ttm_pool_init(pool, devs.dev, NUMA_NO_NODE, 0);
    ttm_pool_alloc(pool, tt, &simple_ctx);
    ttm_pool_free(pool, tt);
    ttm_tt_fini(tt);
    ttm_pool_fini(pool);
    }
#[no_mangle]
unsafe extern "C" fn ttm_pool_fini_basic(test: *mut kunit) {
    static void ttm_pool_fini_basic(struct kunit *test)
    {
    struct ttm_pool *pool;
    struct ttm_pool_type *pt;
    let mut caching: enum ttm_caching = ttm_uncached;
    let mut order: c_uint = 0;
    let mut size: usize = PAGE_SIZE;
    pool = ttm_pool_pre_populated(test, size, caching);
    pt = &pool.caching[caching].orders[order];
    KUNIT_ASSERT_FALSE(test, !list_lru_count(&pt.pages));
    ttm_pool_fini(pool);
    KUNIT_ASSERT_TRUE(test, !list_lru_count(&pt.pages));
    }
    static struct kunit_case ttm_pool_test_cases[] = {
    KUNIT_CASE_PARAM(ttm_pool_alloc_basic, ttm_pool_alloc_basic_gen_params),
    KUNIT_CASE_PARAM(ttm_pool_alloc_basic_dma_addr,
    ttm_pool_alloc_basic_gen_params),
    KUNIT_CASE(ttm_pool_alloc_order_caching_match),
    KUNIT_CASE(ttm_pool_alloc_caching_mismatch),
    KUNIT_CASE(ttm_pool_alloc_order_mismatch),
    KUNIT_CASE(ttm_pool_free_dma_alloc),
    KUNIT_CASE(ttm_pool_free_no_dma_alloc),
    KUNIT_CASE(ttm_pool_fini_basic),
    {}
    };
    static struct kunit_suite ttm_pool_test_suite = {
    .name = "ttm_pool",
    .init = ttm_pool_test_init,
    .exit = ttm_pool_test_fini,
    .test_cases = ttm_pool_test_cases,
    };
    kunit_test_suites(&ttm_pool_test_suite);
    MODULE_DESCRIPTION("KUnit tests for ttm_pool APIs");
    MODULE_LICENSE("GPL and additional rights");
