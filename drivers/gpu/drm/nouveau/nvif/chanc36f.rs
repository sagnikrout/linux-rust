//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/chanc36f.c
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

    static void
    nvif_chanc36f_gpfifo_kick(struct nvif_chan *chan)
    {
    struct nvif_user *usermode = chan.usermode;
    nvif_wr32(&chan.userd, 0x8c, chan.gpfifo.cur);
    wmb(); /* ensure CPU writes are flushed to BAR1 */
    nvif_rd32(&chan.userd, 0); /* ensure BAR1 writes are flushed to vidmem */
    usermode.func.doorbell(usermode, chan.doorbell_token);
    }
pub const NVIF_CHANC36F_SEM_RELEASE_SIZE: c_int = 6;
    static int
    nvif_chanc36f_sem_release(struct nvif_chan *chan, u64 addr, u32 data)
    {
    struct nvif_push *push = &chan.push;
    int ret;
    ret = PUSH_WAIT(push, NVIF_CHANC36F_SEM_RELEASE_SIZE);
    if (ret)
    return ret;
    PUSH_MTHD(push, NVC36F, SEM_ADDR_LO, lower_32_bits(addr),
    SEM_ADDR_HI, upper_32_bits(addr),
    SEM_PAYLOAD_LO, data);
    PUSH_MTHD(push, NVC36F, SEM_EXECUTE,
    NVDEF(NVC36F, SEM_EXECUTE, OPERATION, RELEASE) |
    NVDEF(NVC36F, SEM_EXECUTE, RELEASE_WFI, DIS) |
    NVDEF(NVC36F, SEM_EXECUTE, PAYLOAD_SIZE, 32BIT) |
    NVDEF(NVC36F, SEM_EXECUTE, RELEASE_TIMESTAMP, DIS));
    return 0;
    }
    static const struct nvif_chan_func
    nvif_chanc36f = {
    .push.read_get = nvif_chan906f_read_get,
    .gpfifo.read_get = nvif_chan906f_gpfifo_read_get,
    .gpfifo.push = nvif_chan506f_gpfifo_push,
    .gpfifo.kick = nvif_chanc36f_gpfifo_kick,
    .gpfifo.post = nvif_chan906f_gpfifo_post,
    .gpfifo.post_size = NVIF_CHANC36F_SEM_RELEASE_SIZE,
    .sem.release = nvif_chanc36f_sem_release,
    };
    int
    nvif_chanc36f_ctor(struct nvif_chan *chan, void *userd, void *gpfifo, u32 gpfifo_size,
    void *push, u64 push_addr, u32 push_size, void *sema, u64 sema_addr,
    struct nvif_user *usermode, u32 doorbell_token)
    {
    int ret;
    ret = nvif_chan906f_ctor_(&nvif_chanc36f, userd, gpfifo, gpfifo_size,
    push, push_addr, push_size, sema, sema_addr, chan);
    if (ret)
    return ret;
    chan.usermode = usermode;
    chan.doorbell_token = doorbell_token;
    return 0;
    }
