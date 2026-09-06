//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvc0_fence.c
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

    static int
    nvc0_fence_emit32(struct nouveau_channel *chan, u64 virtual, u32 sequence)
    {
    struct nvif_push *push = &chan.chan.push;
    let mut ret: c_int = PUSH_WAIT(push, 6);
    if (ret == 0) {
    PUSH_MTHD(push, NV906F, SEMAPHOREA,
    NVVAL(NV906F, SEMAPHOREA, OFFSET_UPPER, upper_32_bits(virtual)),
    SEMAPHOREB, lower_32_bits(virtual),
    SEMAPHOREC, sequence,
    SEMAPHORED,
    NVDEF(NV906F, SEMAPHORED, OPERATION, RELEASE) |
    NVDEF(NV906F, SEMAPHORED, RELEASE_WFI, EN) |
    NVDEF(NV906F, SEMAPHORED, RELEASE_SIZE, 16BYTE),
    NON_STALL_INTERRUPT, 0);
    PUSH_KICK(push);
    }
    return ret;
    }
    static int
    nvc0_fence_sync32(struct nouveau_channel *chan, u64 virtual, u32 sequence)
    {
    struct nvif_push *push = &chan.chan.push;
    let mut ret: c_int = PUSH_WAIT(push, 5);
    if (ret == 0) {
    PUSH_MTHD(push, NV906F, SEMAPHOREA,
    NVVAL(NV906F, SEMAPHOREA, OFFSET_UPPER, upper_32_bits(virtual)),
    SEMAPHOREB, lower_32_bits(virtual),
    SEMAPHOREC, sequence,
    SEMAPHORED,
    NVDEF(NV906F, SEMAPHORED, OPERATION, ACQ_GEQ) |
    NVDEF(NV906F, SEMAPHORED, ACQUIRE_SWITCH, ENABLED));
    PUSH_KICK(push);
    }
    return ret;
    }
    static int
    nvc0_fence_context_new(struct nouveau_channel *chan)
    {
    let mut ret: c_int = nv84_fence_context_new(chan);
    if (ret == 0) {
    struct nv84_fence_chan *fctx = chan.fence;
    fctx.base.emit32 = nvc0_fence_emit32;
    fctx.base.sync32 = nvc0_fence_sync32;
    }
    return ret;
    }
    int
    nvc0_fence_create(struct nouveau_drm *drm)
    {
    let mut ret: c_int = nv84_fence_create(drm);
    if (ret == 0) {
    struct nv84_fence_priv *priv = drm.fence;
    priv.base.context_new = nvc0_fence_context_new;
    }
    return ret;
    }
