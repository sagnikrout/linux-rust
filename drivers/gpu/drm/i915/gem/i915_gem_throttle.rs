//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gem/i915_gem_throttle.c
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
// Copyright © 2014-2016 Intel Corporation
//

//
// 20ms is a fairly arbitrary limit (greater than the average frame time)
// chosen to prevent the CPU getting more than a frame ahead of the GPU
// (when using lax throttling for the frontbuffer). We also use it to
// offer free GPU waitboosts for severely congested workloads.
//

//
// Throttle our rendering by waiting until the ring has completed our requests
// emitted over 20 msec ago.
//
// Note that if we were to use the current jiffies each time around the loop,
// we wouldn't escape the function with any frames outstanding if the time to
// render a frame was over 20ms.
//
// This should get us reasonable parallelism between CPU and GPU but also
// relatively low latency when blocking on a particular request to finish.
//
    int
    i915_gem_throttle_ioctl(struct drm_device *dev, void *data,
    struct drm_file *file)
    {
    let mut recent_enough: c_ulong = jiffies - DRM_I915_THROTTLE_JIFFIES;
    struct drm_i915_file_private *file_priv = file.driver_priv;
    struct drm_i915_private *i915 = to_i915(dev);
    struct i915_gem_context *ctx;
    unsigned long idx;
    long ret;
// ABI: return -EIO if already wedged
    ret = intel_gt_terminally_wedged(to_gt(i915));
    if (ret)
    return ret;
    rcu_read_lock();
    xa_for_each(&file_priv.context_xa, idx, ctx) {
    struct i915_gem_engines_iter it;
    struct intel_context *ce;
    if (!kref_get_unless_zero(&ctx.ref))
    continue;
    rcu_read_unlock();
    for_each_gem_engine(ce,
    i915_gem_context_lock_engines(ctx),
    it) {
    struct i915_request *rq, *target = core::ptr::null_mut();
    if (!ce.timeline)
    continue;
    mutex_lock(&ce.timeline.mutex);
    list_for_each_entry_reverse(rq,
    &ce.timeline.requests,
    link) {
    if (i915_request_completed(rq))
    break;
    if (time_after(rq.emitted_jiffies,
    recent_enough))
    continue;
    target = i915_request_get(rq);
    break;
    }
    mutex_unlock(&ce.timeline.mutex);
    if (!target)
    continue;
    ret = i915_request_wait(target,
    I915_WAIT_INTERRUPTIBLE,
    MAX_SCHEDULE_TIMEOUT);
    i915_request_put(target);
    if (ret < 0)
    break;
    }
    i915_gem_context_unlock_engines(ctx);
    i915_gem_context_put(ctx);
    rcu_read_lock();
    }
    rcu_read_unlock();
    return ret < 0 ? ret : 0;
    }
