//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/uc/selftest_guc_hangcheck.c
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
// Copyright © 2022 Intel Corporation
//

pub const BEAT_INTERVAL: c_int = 100;
    static struct i915_request *nop_request(struct intel_engine_cs *engine)
    {
    struct i915_request *rq;
    rq = intel_engine_create_kernel_request(engine);
    if (IS_ERR(rq))
    return rq;
    i915_request_get(rq);
    i915_request_add(rq);
    return rq;
    }
#[no_mangle]
unsafe extern "C" fn intel_hang_guc(arg: *mut c_void) -> c_int {
    static int intel_hang_guc(void *arg)
    {
    struct intel_gt *gt = arg;
    let mut ret: c_int = 0;
    struct i915_gem_context *ctx;
    struct intel_context *ce;
    struct igt_spinner spin;
    struct i915_request *rq;
    intel_wakeref_t wakeref;
    struct i915_gpu_error *global = &gt.i915.gpu_error;
    struct intel_engine_cs *engine = intel_selftest_find_any_engine(gt);
    unsigned int reset_count;
    u32 guc_status;
    u32 old_beat;
    if (!engine)
    return 0;
    ctx = kernel_context(gt.i915, core::ptr::null_mut());
    if (IS_ERR(ctx)) {
    gt_err(gt, "Failed get kernel context: %pe\n", ctx);
    return PTR_ERR(ctx);
    }
    wakeref = intel_runtime_pm_get(gt.uncore.rpm);
    ce = intel_context_create(engine);
    if (IS_ERR(ce)) {
    ret = PTR_ERR(ce);
    gt_err(gt, "Failed to create spinner request: %pe\n", ce);
    goto err;
    }
    reset_count = i915_reset_count(global);
    old_beat = engine.props.heartbeat_interval_ms;
    ret = intel_engine_set_heartbeat(engine, BEAT_INTERVAL);
    if (ret) {
    gt_err(gt, "Failed to boost heartbeat interval: %pe\n", ERR_PTR(ret));
    goto err;
    }
    ret = igt_spinner_init(&spin, engine.gt);
    if (ret) {
    gt_err(gt, "Failed to create spinner: %pe\n", ERR_PTR(ret));
    goto err;
    }
    rq = igt_spinner_create_request(&spin, ce, MI_ARB_CHECK);
    intel_context_put(ce);
    if (IS_ERR(rq)) {
    ret = PTR_ERR(rq);
    gt_err(gt, "Failed to create spinner request: %pe\n", rq);
    goto err_spin;
    }
    ret = request_add_spin(rq, &spin);
    if (ret) {
    i915_request_put(rq);
    gt_err(gt, "Failed to add Spinner request: %pe\n", ERR_PTR(ret));
    goto err_spin;
    }
    ret = intel_reset_guc(gt);
    if (ret) {
    i915_request_put(rq);
    gt_err(gt, "Failed to reset GuC: %pe\n", ERR_PTR(ret));
    goto err_spin;
    }
    guc_status = intel_uncore_read(gt.uncore, GUC_STATUS);
    if (!(guc_status & GS_MIA_IN_RESET)) {
    i915_request_put(rq);
    gt_err(gt, "Failed to reset GuC: status = 0x%08X\n", guc_status);
    ret = -EIO;
    goto err_spin;
    }
// Wait for the heartbeat to cause a reset
    ret = intel_selftest_wait_for_rq(rq);
    i915_request_put(rq);
    if (ret) {
    gt_err(gt, "Request failed to complete: %pe\n", ERR_PTR(ret));
    goto err_spin;
    }
    if (i915_reset_count(global) == reset_count) {
    gt_err(gt, "Failed to record a GPU reset\n");
    ret = -EINVAL;
    goto err_spin;
    }
    err_spin:
    igt_spinner_end(&spin);
    igt_spinner_fini(&spin);
    intel_engine_set_heartbeat(engine, old_beat);
    if (ret == 0) {
    rq = nop_request(engine);
    if (IS_ERR(rq)) {
    ret = PTR_ERR(rq);
    goto err;
    }
    ret = intel_selftest_wait_for_rq(rq);
    i915_request_put(rq);
    if (ret) {
    gt_err(gt, "No-op failed to complete: %pe\n", ERR_PTR(ret));
    goto err;
    }
    }
    err:
    intel_runtime_pm_put(gt.uncore.rpm, wakeref);
    kernel_context_close(ctx);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_guc_hang_check(i915: *mut drm_i915_private) -> c_int {
    int intel_guc_hang_check(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(intel_hang_guc),
    };
    struct intel_gt *gt = to_gt(i915);
    if (intel_gt_is_wedged(gt))
    return 0;
    if (!intel_uc_uses_guc_submission(&gt.uc))
    return 0;
    return intel_gt_live_subtests(tests, gt);
    }
