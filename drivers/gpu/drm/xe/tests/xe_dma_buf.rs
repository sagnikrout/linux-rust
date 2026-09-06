//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_dma_buf.c
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
// Copyright © 2022 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn p2p_enabled(params: *mut dma_buf_test_params) -> bool {
    static bool p2p_enabled(struct dma_buf_test_params *params)
    {
    return IS_ENABLED(CONFIG_PCI_P2PDMA) && params.attach_ops &&
    params.attach_ops.allow_peer2peer;
    }
#[no_mangle]
unsafe extern "C" fn is_dynamic(params: *mut dma_buf_test_params) -> bool {
    static bool is_dynamic(struct dma_buf_test_params *params)
    {
    return params.attach_ops && params.attach_ops.invalidate_mappings;
    }
    static void check_residency(struct kunit *test, struct xe_bo *exported,
    struct xe_bo *imported, struct dma_buf *dmabuf,
    struct drm_exec *exec)
    {
    struct dma_buf_test_params *params = to_dma_buf_test_params(test.priv);
    struct dma_buf_attachment *attach;
    u32 mem_type;
    int ret;
    xe_bo_assert_held(exported);
    xe_bo_assert_held(imported);
    mem_type = XE_PL_VRAM0;
    if (!(params.mem_mask & XE_BO_FLAG_VRAM0))
// No VRAM allowed
    mem_type = XE_PL_TT;
#[no_mangle]
pub unsafe extern "C" fn if(!p2p_enabled(params): params->force_different_devices &&) -> else {
    else if (params.force_different_devices && !p2p_enabled(params))
// No P2P
    mem_type = XE_PL_TT;
    else if (params.force_different_devices && !is_dynamic(params) &&
    (params.mem_mask & XE_BO_FLAG_SYSTEM))
// Pin migrated to TT on non-dynamic attachments.
    mem_type = XE_PL_TT;
    if (!xe_bo_is_mem_type(exported, mem_type)) {
    KUNIT_FAIL(test, "Exported bo was not in expected memory type.\n");
    return;
    }
    if (xe_bo_is_pinned(exported))
    return;
//
// Evict exporter. Evicting the exported bo will
// evict also the imported bo through the invalidate_mappings() functionality if
// importer is on a different device. If they're on the same device,
// the exporter and the importer should be the same bo.
//
    ret = xe_bo_evict(exported, exec);
    if (ret) {
    if (ret != -EINTR && ret != -ERESTARTSYS)
    KUNIT_FAIL(test, "Evicting exporter failed with err=%d.\n",
    ret);
    return;
    }
// Verify that also importer has been evicted to SYSTEM
    if (exported != imported && !xe_bo_is_mem_type(imported, XE_PL_SYSTEM)) {
    KUNIT_FAIL(test, "Importer wasn't properly evicted.\n");
    return;
    }
// Re-validate the importer. This should move also exporter in.
    ret = xe_bo_validate(imported, core::ptr::null_mut(), false, exec);
    if (ret) {
    if (ret != -EINTR && ret != -ERESTARTSYS)
    KUNIT_FAIL(test, "Validating importer failed with err=%d.\n",
    ret);
    return;
    }
    KUNIT_EXPECT_TRUE(test, xe_bo_is_mem_type(exported, mem_type));
// Check that we can pin without migrating.
    attach = list_first_entry_or_null(&dmabuf.attachments, typeof(*attach), node);
    if (attach) {
    let mut err: c_int = dma_buf_pin(attach);
    if (!err) {
    KUNIT_EXPECT_TRUE(test, xe_bo_is_mem_type(exported, mem_type));
    dma_buf_unpin(attach);
    }
    KUNIT_EXPECT_EQ(test, err, 0);
    }
    if (params.force_different_devices)
    KUNIT_EXPECT_TRUE(test, xe_bo_is_mem_type(imported, XE_PL_TT));
    else
    KUNIT_EXPECT_TRUE(test, exported == imported);
    }
#[no_mangle]
unsafe extern "C" fn xe_test_dmabuf_import_same_driver(xe: *mut xe_device) {
    static void xe_test_dmabuf_import_same_driver(struct xe_device *xe)
    {
    struct kunit *test = kunit_get_current_test();
    struct dma_buf_test_params *params = to_dma_buf_test_params(test.priv);
    struct drm_gem_object *import;
    struct dma_buf *dmabuf;
    struct xe_bo *bo;
    size_t size;
// No VRAM on this device?
    if (!ttm_manager_type(&xe.ttm, XE_PL_VRAM0) &&
    (params.mem_mask & XE_BO_FLAG_VRAM0))
    return;
    size = PAGE_SIZE;
    if ((params.mem_mask & XE_BO_FLAG_VRAM0) &&
    xe.info.vram_flags & XE_VRAM_FLAGS_NEED64K)
    size = SZ_64K;
    kunit_info(test, "running %s\n", __func__);
    bo = xe_bo_create_user(xe, core::ptr::null_mut(), size, DRM_XE_GEM_CPU_CACHING_WC,
    params.mem_mask, core::ptr::null_mut());
    if (IS_ERR(bo)) {
    KUNIT_FAIL(test, "xe_bo_create() failed with err=%ld\n",
    PTR_ERR(bo));
    return;
    }
    dmabuf = xe_gem_prime_export(&bo.ttm.base, 0);
    if (IS_ERR(dmabuf)) {
    KUNIT_FAIL(test, "xe_gem_prime_export() failed with err=%ld\n",
    PTR_ERR(dmabuf));
    goto out;
    }
    bo.ttm.base.dma_buf = dmabuf;
    import = xe_gem_prime_import(&xe.drm, dmabuf);
    if (!IS_ERR(import)) {
    struct xe_bo *import_bo = gem_to_xe_bo(import);
//
// Did import succeed when it shouldn't due to lack of p2p support?
//
    if (params.force_different_devices &&
    !p2p_enabled(params) &&
    !(params.mem_mask & XE_BO_FLAG_SYSTEM)) {
    KUNIT_FAIL(test,
    "xe_gem_prime_import() succeeded when it shouldn't have\n");
    } else {
    struct drm_exec *exec = XE_VALIDATION_OPT_OUT;
    int err;
// Is everything where we expect it to be?
    xe_bo_lock(import_bo, false);
    err = xe_bo_validate(import_bo, core::ptr::null_mut(), false, exec);
// Pinning in VRAM is not allowed for non-dynamic attachments
    if (!is_dynamic(params) &&
    params.force_different_devices &&
    !(params.mem_mask & XE_BO_FLAG_SYSTEM))
    KUNIT_EXPECT_EQ(test, err, -EINVAL);
// Otherwise only expect interrupts or success.
#[no_mangle]
pub unsafe extern "C" fn if(-ERESTARTSYS: err && err != -EINTR && err !=) -> else {
    else if (err && err != -EINTR && err != -ERESTARTSYS)
    KUNIT_EXPECT_TRUE(test, !err || err == -EINTR ||
    err == -ERESTARTSYS);
    if (!err)
    check_residency(test, bo, import_bo, dmabuf, exec);
    xe_bo_unlock(import_bo);
    }
    drm_gem_object_put(import);
    } else if (PTR_ERR(import) != -EOPNOTSUPP) {
// Unexpected error code.
    KUNIT_FAIL(test,
    "xe_gem_prime_import failed with the wrong err=%ld\n",
    PTR_ERR(import));
    } else if (!params.force_different_devices ||
    p2p_enabled(params) ||
    (params.mem_mask & XE_BO_FLAG_SYSTEM)) {
// Shouldn't fail if we can reuse same bo, use p2p or use system
    KUNIT_FAIL(test, "dynamic p2p attachment failed with err=%ld\n",
    PTR_ERR(import));
    }
    bo.ttm.base.dma_buf = core::ptr::null_mut();
    dma_buf_put(dmabuf);
    out:
    drm_gem_object_put(&bo.ttm.base);
    }
    static const struct dma_buf_attach_ops nop2p_attach_ops = {
    .allow_peer2peer = false,
    .invalidate_mappings = xe_dma_buf_move_notify
    };
//
// We test the implementation with bos of different residency and with
// importers with different capabilities; some lacking p2p support and some
// lacking dynamic capabilities (attach_ops == NULL). We also fake
// different devices avoiding the import shortcut that just reuses the same
// gem object.
//
    static const struct dma_buf_test_params test_params[] = {
    {.mem_mask = XE_BO_FLAG_VRAM0,
    .attach_ops = &xe_dma_buf_attach_ops},
    {.mem_mask = XE_BO_FLAG_VRAM0 | XE_BO_FLAG_NEEDS_CPU_ACCESS,
    .attach_ops = &xe_dma_buf_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_VRAM0,
    .attach_ops = &nop2p_attach_ops},
    {.mem_mask = XE_BO_FLAG_VRAM0,
    .attach_ops = &nop2p_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_VRAM0},
    {.mem_mask = XE_BO_FLAG_VRAM0,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM,
    .attach_ops = &xe_dma_buf_attach_ops},
    {.mem_mask = XE_BO_FLAG_SYSTEM,
    .attach_ops = &xe_dma_buf_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM,
    .attach_ops = &nop2p_attach_ops},
    {.mem_mask = XE_BO_FLAG_SYSTEM,
    .attach_ops = &nop2p_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM},
    {.mem_mask = XE_BO_FLAG_SYSTEM,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0,
    .attach_ops = &xe_dma_buf_attach_ops},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0 |
    XE_BO_FLAG_NEEDS_CPU_ACCESS,
    .attach_ops = &xe_dma_buf_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0,
    .attach_ops = &nop2p_attach_ops},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0,
    .attach_ops = &nop2p_attach_ops,
    .force_different_devices = true},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0},
    {.mem_mask = XE_BO_FLAG_SYSTEM | XE_BO_FLAG_VRAM0,
    .force_different_devices = true},
    {}
    };
#[no_mangle]
unsafe extern "C" fn dma_buf_run_device(xe: *mut xe_device) -> c_int {
    static int dma_buf_run_device(struct xe_device *xe)
    {
    const struct dma_buf_test_params *params;
    struct kunit *test = kunit_get_current_test();
    guard(xe_pm_runtime)(xe);
    for (params = test_params; params.mem_mask; ++params) {
    let mut p: dma_buf_test_params = *params;
    p.base.id = XE_TEST_LIVE_DMA_BUF;
    test.priv = &p;
    xe_test_dmabuf_import_same_driver(xe);
    }
// A non-zero return would halt iteration over driver devices
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xe_dma_buf_kunit(test: *mut kunit) {
    static void xe_dma_buf_kunit(struct kunit *test)
    {
    struct xe_device *xe = test.priv;
    dma_buf_run_device(xe);
    }
    static struct kunit_case xe_dma_buf_tests[] = {
    KUNIT_CASE_PARAM(xe_dma_buf_kunit, xe_pci_live_device_gen_param),
    {}
    };
    VISIBLE_IF_KUNIT
    struct kunit_suite xe_dma_buf_test_suite = {
    .name = "xe_dma_buf",
    .test_cases = xe_dma_buf_tests,
    .init = xe_kunit_helper_xe_device_live_test_init,
    };
    EXPORT_SYMBOL_IF_KUNIT(xe_dma_buf_test_suite);
