//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/gv100_fence.c
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

    static int
    gv100_fence_emit32(struct nouveau_channel *chan, u64 virtual, u32 sequence)
    {
    struct nvif_push *push = &chan.chan.push;
    int ret;
    ret = PUSH_WAIT(push, 13);
    if (ret)
    return ret;
    PUSH_MTHD(push, NVC36F, SEM_ADDR_LO, lower_32_bits(virtual),
    SEM_ADDR_HI, upper_32_bits(virtual),
    SEM_PAYLOAD_LO, sequence);
    PUSH_MTHD(push, NVC36F, SEM_EXECUTE,
    NVDEF(NVC36F, SEM_EXECUTE, OPERATION, RELEASE) |
    NVDEF(NVC36F, SEM_EXECUTE, RELEASE_WFI, EN) |
    NVDEF(NVC36F, SEM_EXECUTE, PAYLOAD_SIZE, 32BIT) |
    NVDEF(NVC36F, SEM_EXECUTE, RELEASE_TIMESTAMP, DIS));
    PUSH_MTHD(push, NVC36F, MEM_OP_A, 0,
    MEM_OP_B, 0,
    MEM_OP_C, NVDEF(NVC36F, MEM_OP_C, MEMBAR_TYPE, SYS_MEMBAR),
    MEM_OP_D, NVDEF(NVC36F, MEM_OP_D, OPERATION, MEMBAR));
    PUSH_MTHD(push, NVC36F, NON_STALL_INTERRUPT, 0);
    PUSH_KICK(push);
    return 0;
    }
    static int
    gv100_fence_sync32(struct nouveau_channel *chan, u64 virtual, u32 sequence)
    {
    struct nvif_push *push = &chan.chan.push;
    int ret;
    ret = PUSH_WAIT(push, 6);
    if (ret)
    return ret;
    PUSH_MTHD(push, NVC36F, SEM_ADDR_LO, lower_32_bits(virtual),
    SEM_ADDR_HI, upper_32_bits(virtual),
    SEM_PAYLOAD_LO, sequence);
    PUSH_MTHD(push, NVC36F, SEM_EXECUTE,
    NVDEF(NVC36F, SEM_EXECUTE, OPERATION, ACQ_CIRC_GEQ) |
    NVDEF(NVC36F, SEM_EXECUTE, ACQUIRE_SWITCH_TSG, EN) |
    NVDEF(NVC36F, SEM_EXECUTE, PAYLOAD_SIZE, 32BIT));
    PUSH_KICK(push);
    return 0;
    }
    static int
    gv100_fence_context_new(struct nouveau_channel *chan)
    {
    struct nv84_fence_chan *fctx;
    int ret;
    ret = nv84_fence_context_new(chan);
    if (ret)
    return ret;
    fctx = chan.fence;
    fctx.base.emit32 = gv100_fence_emit32;
    fctx.base.sync32 = gv100_fence_sync32;
    return 0;
    }
    int
    gv100_fence_create(struct nouveau_drm *drm)
    {
    struct nv84_fence_priv *priv;
    int ret;
    ret = nv84_fence_create(drm);
    if (ret)
    return ret;
    priv = drm.fence;
    priv.base.context_new = gv100_fence_context_new;
    return 0;
    }
