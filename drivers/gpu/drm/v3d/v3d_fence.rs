//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/v3d/v3d_fence.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2017-2018 Broadcom

    struct dma_fence *v3d_fence_create(struct v3d_dev *v3d, enum v3d_queue q)
    {
    struct v3d_queue_state *queue = &v3d.queue[q];
    struct v3d_fence *fence;
    fence = kzalloc_obj(*fence);
    if (!fence)
    return ERR_PTR(-ENOMEM);
    fence.dev = &v3d.drm;
    fence.queue = q;
    fence.seqno = ++queue.emit_seqno;
    dma_fence_init(&fence.base, &v3d_fence_ops, &queue.queue_lock,
    queue.fence_context, fence.seqno);
    return &fence.base;
    }
    static const char *v3d_fence_get_driver_name(struct dma_fence *fence)
    {
    return "v3d";
    }
    static const char *v3d_fence_get_timeline_name(struct dma_fence *fence)
    {
    struct v3d_fence *f = to_v3d_fence(fence);
    switch (f.queue) {
    case V3D_BIN:
    return "v3d-bin";
    case V3D_RENDER:
    return "v3d-render";
    case V3D_TFU:
    return "v3d-tfu";
    case V3D_CSD:
    return "v3d-csd";
    default:
    return core::ptr::null_mut();
    }
    }
    const struct dma_fence_ops v3d_fence_ops = {
    .get_driver_name = v3d_fence_get_driver_name,
    .get_timeline_name = v3d_fence_get_timeline_name,
    };
