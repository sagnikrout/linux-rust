//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/chan906f.c
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

// Limits GPFIFO size to 1MiB, and "main" push buffer size to 64KiB.
pub const NVIF_CHAN906F_PBPTR_BITS: c_int = 15;

pub const NVIF_CHAN906F_SEM_RELEASE_SIZE: c_int = 5;
    static int
    nvif_chan906f_sem_release(struct nvif_chan *chan, u64 addr, u32 data)
    {
    struct nvif_push *push = &chan.push;
    int ret;
    ret = PUSH_WAIT(push, NVIF_CHAN906F_SEM_RELEASE_SIZE);
    if (ret)
    return ret;
    PUSH_MTHD(push, NV906F, SEMAPHOREA,
    NVVAL(NV906F, SEMAPHOREA, OFFSET_UPPER, upper_32_bits(addr)),
    SEMAPHOREB, lower_32_bits(addr),
    SEMAPHOREC, data,
    SEMAPHORED,
    NVDEF(NV906F, SEMAPHORED, OPERATION, RELEASE) |
    NVDEF(NV906F, SEMAPHORED, RELEASE_WFI, DIS) |
    NVDEF(NV906F, SEMAPHORED, RELEASE_SIZE, 16BYTE));
    return 0;
    }
    int
    nvif_chan906f_gpfifo_post(struct nvif_chan *chan, u32 gpptr, u32 pbptr)
    {
    return chan.func.sem.release(chan, chan.sema.addr,
    (gpptr << NVIF_CHAN906F_GPPTR_SHIFT) | pbptr);
    }
    u32
    nvif_chan906f_gpfifo_read_get(struct nvif_chan *chan)
    {
    return nvif_rd32(&chan.sema, 0) >> NVIF_CHAN906F_GPPTR_SHIFT;
    }
    u32
    nvif_chan906f_read_get(struct nvif_chan *chan)
    {
    return nvif_rd32(&chan.sema, 0) & NVIF_CHAN906F_PBPTR_MASK;
    }
    static const struct nvif_chan_func
    nvif_chan906f = {
    .push.read_get = nvif_chan906f_read_get,
    .gpfifo.read_get = nvif_chan906f_gpfifo_read_get,
    .gpfifo.push = nvif_chan506f_gpfifo_push,
    .gpfifo.kick = nvif_chan506f_gpfifo_kick,
    .gpfifo.post = nvif_chan906f_gpfifo_post,
    .gpfifo.post_size = NVIF_CHAN906F_SEM_RELEASE_SIZE,
    .sem.release = nvif_chan906f_sem_release,
    };
    int
    nvif_chan906f_ctor_(const struct nvif_chan_func *func, void *userd, void *gpfifo, u32 gpfifo_size,
    void *push, u64 push_addr, u32 push_size, void *sema, u64 sema_addr,
    struct nvif_chan *chan)
    {
    nvif_chan_gpfifo_ctor(func, userd, gpfifo, gpfifo_size, push, push_addr, push_size, chan);
    chan.sema.map.ptr = sema;
    chan.sema.addr = sema_addr;
    return 0;
    }
    int
    nvif_chan906f_ctor(struct nvif_chan *chan, void *userd, void *gpfifo, u32 gpfifo_size,
    void *push, u64 push_addr, u32 push_size, void *sema, u64 sema_addr)
    {
    return nvif_chan906f_ctor_(&nvif_chan906f, userd, gpfifo, gpfifo_size,
    push, push_addr, push_size, sema, sema_addr, chan);
    }
