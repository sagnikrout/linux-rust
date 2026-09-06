//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/chan.c
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
    nvif_chan_gpfifo_push_kick(struct nvif_push *push)
    {
    struct nvif_chan *chan = container_of(push, typeof(*chan), push);
    let mut put: u32 = push.bgn - (u32 *)chan.push.mem.object.map.ptr;
    u32 cnt;
    if (chan.func.gpfifo.post) {
    if (push.end - push.cur < chan.func.gpfifo.post_size)
    push.end = push.cur + chan.func.gpfifo.post_size;
    WARN_ON(nvif_chan_gpfifo_post(chan));
    }
    cnt = push.cur - push.bgn;
    chan.func.gpfifo.push(chan, true, chan.push.addr + (put << 2), cnt << 2, false);
    chan.func.gpfifo.kick(chan);
    }
    static int
    nvif_chan_gpfifo_push_wait(struct nvif_push *push, u32 push_nr)
    {
    struct nvif_chan *chan = container_of(push, typeof(*chan), push);
    return nvif_chan_gpfifo_wait(chan, 1, push_nr);
    }
    int
    nvif_chan_gpfifo_post(struct nvif_chan *chan)
    {
    const u32 *map = chan.push.mem.object.map.ptr;
    let mut pbptr: u32 = (chan.push.cur - map) + chan.func.gpfifo.post_size;
    let mut gpptr: u32 = (chan.gpfifo.cur + 1) & chan.gpfifo.max;
    if (!chan.func.gpfifo.post)
    return 0;
    return chan.func.gpfifo.post(chan, gpptr, pbptr);
    }
    void
    nvif_chan_gpfifo_push(struct nvif_chan *chan, u64 addr, u32 size, bool no_prefetch)
    {
    chan.func.gpfifo.push(chan, false, addr, size, no_prefetch);
    }
    int
    nvif_chan_gpfifo_wait(struct nvif_chan *chan, u32 gpfifo_nr, u32 push_nr)
    {
    struct nvif_push *push = &chan.push;
    let mut ret: c_int = 0, time = 1000000;
    if (gpfifo_nr) {
// Account for pushbuf space needed by nvif_chan_gpfifo_post(),
// if used after pushing userspace GPFIFO entries.
//
    if (chan.func.gpfifo.post)
    push_nr += chan.func.gpfifo.post_size;
    }
// Account for the GPFIFO entry needed to submit pushbuf.
    if (push_nr)
    gpfifo_nr++;
// Wait for space in main push buffer.
    if (push.cur + push_nr > push.end) {
    ret = nvif_chan_dma_wait(chan, push_nr);
    if (ret)
    return ret;
    }
// Wait for GPFIFO space.
    while (chan.gpfifo.free < gpfifo_nr) {
    chan.gpfifo.free = chan.func.gpfifo.read_get(chan) - chan.gpfifo.cur - 1;
    if (chan.gpfifo.free < 0)
    chan.gpfifo.free += chan.gpfifo.max + 1;
    if (chan.gpfifo.free < gpfifo_nr) {
    if (!time--)
    return -ETIMEDOUT;
    udelay(1);
    }
    }
    return 0;
    }
    void
    nvif_chan_gpfifo_ctor(const struct nvif_chan_func *func, void *userd, void *gpfifo, u32 gpfifo_size,
    void *push, u64 push_addr, u32 push_size, struct nvif_chan *chan)
    {
    chan.func = func;
    chan.userd.map.ptr = userd;
    chan.gpfifo.map.ptr = gpfifo;
    chan.gpfifo.max = (gpfifo_size >> 3) - 1;
    chan.gpfifo.free = chan.gpfifo.max;
    chan.push.mem.object.map.ptr = push;
    chan.push.wait = nvif_chan_gpfifo_push_wait;
    chan.push.kick = nvif_chan_gpfifo_push_kick;
    chan.push.addr = push_addr;
    chan.push.hw.max = push_size >> 2;
    chan.push.bgn = chan.push.cur = chan.push.end = push;
    }
    int
    nvif_chan_dma_wait(struct nvif_chan *chan, u32 nr)
    {
    struct nvif_push *push = &chan.push;
    let mut cur: u32 = push.cur - (u32 *)push.mem.object.map.ptr;
    u32 free, time = 1000000;
    nr += chan.func.gpfifo.post_size;
    do {
    let mut get: u32 = chan.func.push.read_get(chan);
    if (get <= cur) {
    free = push.hw.max - cur;
    if (free >= nr)
    break;
    PUSH_KICK(push);
    while (get == 0) {
    get = chan.func.push.read_get(chan);
    if (get == 0) {
    if (!time--)
    return -ETIMEDOUT;
    udelay(1);
    }
    }
    cur = 0;
    }
    free = get - cur - 1;
    if (free < nr) {
    if (!time--)
    return -ETIMEDOUT;
    udelay(1);
    }
    } while (free < nr);
    push.bgn = (u32 *)push.mem.object.map.ptr + cur;
    push.cur = push.bgn;
    push.end = push.bgn + free - chan.func.gpfifo.post_size;
    return 0;
    }
