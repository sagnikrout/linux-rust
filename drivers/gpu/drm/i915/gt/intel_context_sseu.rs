//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/intel_context_sseu.c
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

    static int gen8_emit_rpcs_config(struct i915_request *rq,
    const struct intel_context *ce,
    const struct intel_sseu sseu)
    {
    u64 offset;
    u32 *cs;
    cs = intel_ring_begin(rq, 4);
    if (IS_ERR(cs))
    return PTR_ERR(cs);
    offset = i915_ggtt_offset(ce.state) +
    LRC_STATE_OFFSET + CTX_R_PWR_CLK_STATE * 4;
// cs++ = MI_STORE_DWORD_IMM_GEN4 | MI_USE_GGTT;
// cs++ = lower_32_bits(offset);
// cs++ = upper_32_bits(offset);
// cs++ = intel_sseu_make_rpcs(rq->engine->gt, &sseu);
    intel_ring_advance(rq, cs);
    return 0;
    }
    static int
    gen8_modify_rpcs(struct intel_context *ce, const struct intel_sseu sseu)
    {
    struct i915_request *rq;
    int ret;
    lockdep_assert_held(&ce.pin_mutex);
//
// If the context is not idle, we have to submit an ordered request to
// modify its context image via the kernel context (writing to our own
// image, or into the registers directory, does not stick). Pristine
// and idle contexts will be configured on pinning.
//
    if (!intel_context_pin_if_active(ce))
    return 0;
    rq = intel_engine_create_kernel_request(ce.engine);
    if (IS_ERR(rq)) {
    ret = PTR_ERR(rq);
    goto out_unpin;
    }
// Serialise with the remote context
    ret = intel_context_prepare_remote_request(ce, rq);
    if (ret == 0)
    ret = gen8_emit_rpcs_config(rq, ce, sseu);
    i915_request_add(rq);
    out_unpin:
    intel_context_unpin(ce);
    return ret;
    }
    int
    intel_context_reconfigure_sseu(struct intel_context *ce,
    const struct intel_sseu sseu)
    {
    int ret;
    GEM_BUG_ON(GRAPHICS_VER(ce.engine.i915) < 8);
    ret = intel_context_lock_pinned(ce);
    if (ret)
    return ret;
// Nothing to do if unmodified.
    if (!memcmp(&ce.sseu, &sseu, sizeof(sseu)))
    goto unlock;
    ret = gen8_modify_rpcs(ce, sseu);
    if (!ret)
    ce.sseu = sseu;
    unlock:
    intel_context_unlock_pinned(ce);
    return ret;
    }
