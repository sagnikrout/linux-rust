//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/tests/amdgpu_dm_kunit_helpers.c
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
// KUnit test helpers for amdgpu_dm tests.
//
// Copyright 2026 Advanced Micro Devices, Inc.
//

    struct amdgpu_device *dm_kunit_alloc_adev(struct kunit *test)
    {
    struct drm_device *drm;
    struct device *dev;
    dev = drm_kunit_helper_alloc_device(test);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
    drm = __drm_kunit_helper_alloc_drm_device(test, dev,
    sizeof(struct amdgpu_device),
    offsetof(struct amdgpu_device, ddev),
    DRIVER_MODESET | DRIVER_ATOMIC);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, drm);
    return drm_to_adev(drm);
    }
    EXPORT_SYMBOL(dm_kunit_alloc_adev);
    struct dc_link *dm_kunit_alloc_link(struct kunit *test)
    {
    struct dc_link *link;
    link = kunit_kzalloc(test, sizeof(*link), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, link);
    return link;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_link);
    struct dc *dm_kunit_alloc_dc_with_ctx(struct kunit *test)
    {
    struct dc_context *ctx;
    struct dc *dc;
    dc = kunit_kzalloc(test, sizeof(*dc), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dc);
    ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ctx);
    dc.ctx = ctx;
    ctx.dc = dc;
    return dc;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_dc_with_ctx);
    struct dc_link *dm_kunit_alloc_link_with_ctx(struct kunit *test)
    {
    struct dc_link *link;
    struct dc *dc;
    link = dm_kunit_alloc_link(test);
    dc = dm_kunit_alloc_dc_with_ctx(test);
    link.ctx = dc.ctx;
    return link;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_link_with_ctx);
    struct amdgpu_display_manager *dm_kunit_alloc_dm(struct kunit *test)
    {
    struct amdgpu_display_manager *dm;
    struct dc *dc;
    struct dc_state *state;
    dm = kunit_kzalloc(test, sizeof(*dm), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dm);
    dc = kunit_kzalloc(test, sizeof(*dc), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dc);
    state = kunit_kzalloc(test, sizeof(*state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state);
    dm.dc = dc;
    dc.current_state = state;
    return dm;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_dm);
    struct dc_stream_state *dm_kunit_alloc_stream(struct kunit *test,
    struct dc_link *link)
    {
    struct dc_stream_state *stream;
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, stream);
    stream.link = link;
    kref_init(&stream.refcount);
    return stream;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_stream);
    struct dc_state *dm_kunit_alloc_dc_state(struct kunit *test)
    {
    struct dc_state *state;
    state = kunit_kzalloc(test, sizeof(*state), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, state);
    return state;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_dc_state);
    struct clk_mgr *dm_kunit_alloc_clk_mgr(struct kunit *test)
    {
    struct clk_mgr *clk_mgr;
    struct clk_mgr_funcs *funcs;
    clk_mgr = kunit_kzalloc(test, sizeof(*clk_mgr), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, clk_mgr);
    funcs = kunit_kzalloc(test, sizeof(*funcs), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, funcs);
    clk_mgr.funcs = funcs;
    return clk_mgr;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_clk_mgr);
    void dm_kunit_add_stream_to_state(struct kunit *test, struct dc_state *state,
    unsigned int index, struct dc_link *link)
    {
    struct dc_stream_state *stream;
    KUNIT_ASSERT_LT(test, index, (unsigned int)MAX_PIPES);
    stream = kunit_kzalloc(test, sizeof(*stream), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, stream);
    stream.link = link;
    state.streams[index] = stream;
    if (state.stream_count <= index)
    state.stream_count = index + 1;
    }
    EXPORT_SYMBOL(dm_kunit_add_stream_to_state);
    struct amdgpu_dm_connector *dm_kunit_alloc_connector(struct kunit *test,
    struct amdgpu_device *adev,
    struct dc_link *link)
    {
    struct amdgpu_dm_connector *aconnector;
    aconnector = drmm_kzalloc(adev_to_drm(adev), sizeof(*aconnector), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, aconnector);
    if (adev)
    aconnector.base.dev = &adev.ddev;
    aconnector.dc_link = link;
    return aconnector;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_connector);
    struct drm_device *dm_kunit_alloc_drm_with_connector_list(struct kunit *test)
    {
    struct drm_device *dev;
    dev = kunit_kzalloc(test, sizeof(*dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
    INIT_LIST_HEAD(&dev.mode_config.connector_list);
    spin_lock_init(&dev.mode_config.connector_list_lock);
    return dev;
    }
    EXPORT_SYMBOL(dm_kunit_alloc_drm_with_connector_list);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("KUnit test helpers for amdgpu_dm tests");
