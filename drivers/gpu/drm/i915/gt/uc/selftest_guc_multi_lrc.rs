//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/uc/selftest_guc_multi_lrc.c
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
// Copyright © 2019 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn logical_sort(engines: *mut intel_engine_cs, num_engines: c_int) {
    static void logical_sort(struct intel_engine_cs **engines, int num_engines)
    {
    struct intel_engine_cs *sorted[MAX_ENGINE_INSTANCE + 1];
    int i, j;
    for (i = 0; i < num_engines; ++i)
    for (j = 0; j < MAX_ENGINE_INSTANCE + 1; ++j) {
    if (engines[j].logical_mask & BIT(i)) {
    sorted[i] = engines[j];
    break;
    }
    }
    memcpy(*engines, *sorted,
    sizeof(struct intel_engine_cs *) * num_engines);
    }
    static struct intel_context *
    multi_lrc_create_parent(struct intel_gt *gt, u8 class,
    unsigned long flags)
    {
    struct intel_engine_cs *siblings[MAX_ENGINE_INSTANCE + 1];
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
    let mut i: c_int = 0;
    for_each_engine(engine, gt, id) {
    if (engine.class != class)
    continue;
    siblings[i++] = engine;
    }
    if (i <= 1)
    return core::ptr::null_mut();
    logical_sort(siblings, i);
    return intel_engine_create_parallel(siblings, 1, i);
    }
#[no_mangle]
unsafe extern "C" fn multi_lrc_context_unpin(ce: *mut intel_context) {
    static void multi_lrc_context_unpin(struct intel_context *ce)
    {
    struct intel_context *child;
    GEM_BUG_ON(!intel_context_is_parent(ce));
    for_each_child(ce, child)
    intel_context_unpin(child);
    intel_context_unpin(ce);
    }
#[no_mangle]
unsafe extern "C" fn multi_lrc_context_put(ce: *mut intel_context) {
    static void multi_lrc_context_put(struct intel_context *ce)
    {
    GEM_BUG_ON(!intel_context_is_parent(ce));
//
// Only the parent gets the creation ref put in the uAPI, the parent
// itself is responsible for creation ref put on the children.
//
    intel_context_put(ce);
    }
    static struct i915_request *
    multi_lrc_nop_request(struct intel_context *ce)
    {
    struct intel_context *child;
    struct i915_request *rq, *child_rq;
    let mut i: c_int = 0;
    GEM_BUG_ON(!intel_context_is_parent(ce));
    rq = intel_context_create_request(ce);
    if (IS_ERR(rq))
    return rq;
    i915_request_get(rq);
    i915_request_add(rq);
    for_each_child(ce, child) {
    child_rq = intel_context_create_request(child);
    if (IS_ERR(child_rq))
    goto child_error;
    if (++i == ce.parallel.number_children)
    set_bit(I915_FENCE_FLAG_SUBMIT_PARALLEL,
    &child_rq.fence.flags);
    i915_request_add(child_rq);
    }
    return rq;
    child_error:
    i915_request_put(rq);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn __intel_guc_multi_lrc_basic(gt: *mut intel_gt, class: c_uint) -> c_int {
    static int __intel_guc_multi_lrc_basic(struct intel_gt *gt, unsigned int class)
    {
    struct intel_context *parent;
    struct i915_request *rq;
    int ret;
    parent = multi_lrc_create_parent(gt, class, 0);
    if (IS_ERR(parent)) {
    gt_err(gt, "Failed creating contexts: %pe\n", parent);
    return PTR_ERR(parent);
    } else if (!parent) {
    gt_dbg(gt, "Not enough engines in class: %d\n", class);
    return 0;
    }
    rq = multi_lrc_nop_request(parent);
    if (IS_ERR(rq)) {
    ret = PTR_ERR(rq);
    gt_err(gt, "Failed creating requests: %pe\n", rq);
    goto out;
    }
    ret = intel_selftest_wait_for_rq(rq);
    if (ret)
    gt_err(gt, "Failed waiting on request: %pe\n", ERR_PTR(ret));
    i915_request_put(rq);
    if (ret >= 0) {
    ret = intel_gt_wait_for_idle(gt, HZ * 5);
    if (ret < 0)
    gt_err(gt, "GT failed to idle: %pe\n", ERR_PTR(ret));
    }
    out:
    multi_lrc_context_unpin(parent);
    multi_lrc_context_put(parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_guc_multi_lrc_basic(arg: *mut c_void) -> c_int {
    static int intel_guc_multi_lrc_basic(void *arg)
    {
    struct intel_gt *gt = arg;
    unsigned int class;
    int ret;
    for (class = 0; class < MAX_ENGINE_CLASS + 1; ++class) {
// We don't support breadcrumb handshake on these classes
    if (class == COMPUTE_CLASS || class == RENDER_CLASS)
    continue;
    ret = __intel_guc_multi_lrc_basic(gt, class);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_guc_multi_lrc_live_selftests(i915: *mut drm_i915_private) -> c_int {
    int intel_guc_multi_lrc_live_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(intel_guc_multi_lrc_basic),
    };
    struct intel_gt *gt = to_gt(i915);
    if (intel_gt_is_wedged(gt))
    return 0;
    if (!intel_uc_uses_guc_submission(&gt.uc))
    return 0;
    return intel_gt_live_subtests(tests, gt);
    }
