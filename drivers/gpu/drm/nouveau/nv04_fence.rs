//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nv04_fence.c
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


//
// Copyright 2012 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Ben Skeggs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_fence_chan {
    pub base: nouveau_fence_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv04_fence_priv {
    pub base: nouveau_fence_priv,
}

    static int
    nv04_fence_emit(struct nouveau_fence *fence)
    {
    struct nvif_push *push = &unrcu_pointer(fence.channel).chan.push;
    let mut ret: c_int = PUSH_WAIT(push, 2);
    if (ret == 0) {
    PUSH_NVSQ(push, NV_SW, 0x0150, fence.base.seqno);
    PUSH_KICK(push);
    }
    return ret;
    }
    static int
    nv04_fence_sync(struct nouveau_fence *fence,
    struct nouveau_channel *prev, struct nouveau_channel *chan)
    {
    return -ENODEV;
    }
    static u32
    nv04_fence_read(struct nouveau_channel *chan)
    {
    let mut args: nv04_nvsw_get_ref_v0 = {};
    WARN_ON(nvif_object_mthd(&chan.nvsw, NV04_NVSW_GET_REF,
    &args, sizeof(args)));
    return args.ref;
    }
    static void
    nv04_fence_context_del(struct nouveau_channel *chan)
    {
    struct nv04_fence_chan *fctx = chan.fence;
    nouveau_fence_context_del(&fctx.base);
    chan.fence = core::ptr::null_mut();
    nouveau_fence_context_free(&fctx.base);
    }
    static int
    nv04_fence_context_new(struct nouveau_channel *chan)
    {
    struct nv04_fence_chan *fctx = kzalloc_obj(*fctx);
    if (fctx) {
    nouveau_fence_context_new(chan, &fctx.base);
    fctx.base.emit = nv04_fence_emit;
    fctx.base.sync = nv04_fence_sync;
    fctx.base.read = nv04_fence_read;
    chan.fence = fctx;
    return 0;
    }
    return -ENOMEM;
    }
    static void
    nv04_fence_destroy(struct nouveau_drm *drm)
    {
    struct nv04_fence_priv *priv = drm.fence;
    drm.fence = core::ptr::null_mut();
    kfree(priv);
    }
    int
    nv04_fence_create(struct nouveau_drm *drm)
    {
    struct nv04_fence_priv *priv;
    priv = drm.fence = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.base.dtor = nv04_fence_destroy;
    priv.base.context_new = nv04_fence_context_new;
    priv.base.context_del = nv04_fence_context_del;
    return 0;
    }
