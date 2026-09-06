//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/chan506f.c
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

    void
    nvif_chan506f_gpfifo_kick(struct nvif_chan *chan)
    {
    wmb();
    nvif_wr32(&chan.userd, 0x8c, chan.gpfifo.cur);
    }
    void
    nvif_chan506f_gpfifo_push(struct nvif_chan *chan, bool main, u64 addr, u32 size, bool no_prefetch)
    {
    let mut gpptr: u32 = chan.gpfifo.cur << 3;
    if (WARN_ON(!chan.gpfifo.free))
    return;
    nvif_wr32(&chan.gpfifo, gpptr + 0, lower_32_bits(addr));
    nvif_wr32(&chan.gpfifo, gpptr + 4, upper_32_bits(addr) |
    (main ? 0 : BIT(9)) |
    (size >> 2) << 10 |
    (no_prefetch ? BIT(31) : 0));
    chan.gpfifo.cur = (chan.gpfifo.cur + 1) & chan.gpfifo.max;
    chan.gpfifo.free--;
    if (!chan.gpfifo.free)
    chan.push.end = chan.push.cur;
    }
    static u32
    nvif_chan506f_gpfifo_read_get(struct nvif_chan *chan)
    {
    return nvif_rd32(&chan.userd, 0x88);
    }
    static u32
    nvif_chan506f_read_get(struct nvif_chan *chan)
    {
    let mut tlgetlo: u32 = nvif_rd32(&chan.userd, 0x58);
    let mut tlgethi: u32 = nvif_rd32(&chan.userd, 0x5c);
    struct nvif_push *push = &chan.push;
// Update cached GET pointer if TOP_LEVEL_GET is valid.
    if (tlgethi & BIT(31)) {
    let mut tlget: u64 = ((u64)(tlgethi & 0xff) << 32) | tlgetlo;
    push.hw.get = (tlget - push.addr) >> 2;
    }
    return push.hw.get;
    }
    static const struct nvif_chan_func
    nvif_chan506f = {
    .push.read_get = nvif_chan506f_read_get,
    .gpfifo.read_get = nvif_chan506f_gpfifo_read_get,
    .gpfifo.push = nvif_chan506f_gpfifo_push,
    .gpfifo.kick = nvif_chan506f_gpfifo_kick,
    };
    int
    nvif_chan506f_ctor(struct nvif_chan *chan, void *userd, void *gpfifo, u32 gpfifo_size,
    void *push, u64 push_addr, u32 push_size)
    {
    nvif_chan_gpfifo_ctor(&nvif_chan506f, userd, gpfifo, gpfifo_size,
    push, push_addr, push_size, chan);
    return 0;
    }
