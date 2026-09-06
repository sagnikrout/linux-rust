//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/privring/gk104.c
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

    static void
    gk104_privring_intr_hub(struct nvkm_subdev *privring, int i)
    {
    struct nvkm_device *device = privring.device;
    let mut addr: u32 = nvkm_rd32(device, 0x122120 + (i * 0x0800));
    let mut data: u32 = nvkm_rd32(device, 0x122124 + (i * 0x0800));
    let mut stat: u32 = nvkm_rd32(device, 0x122128 + (i * 0x0800));
    nvkm_debug(privring, "HUB%d: %06x %08x (%08x)\n", i, addr, data, stat);
    }
    static void
    gk104_privring_intr_rop(struct nvkm_subdev *privring, int i)
    {
    struct nvkm_device *device = privring.device;
    let mut addr: u32 = nvkm_rd32(device, 0x124120 + (i * 0x0800));
    let mut data: u32 = nvkm_rd32(device, 0x124124 + (i * 0x0800));
    let mut stat: u32 = nvkm_rd32(device, 0x124128 + (i * 0x0800));
    nvkm_debug(privring, "ROP%d: %06x %08x (%08x)\n", i, addr, data, stat);
    }
    static void
    gk104_privring_intr_gpc(struct nvkm_subdev *privring, int i)
    {
    struct nvkm_device *device = privring.device;
    let mut addr: u32 = nvkm_rd32(device, 0x128120 + (i * 0x0800));
    let mut data: u32 = nvkm_rd32(device, 0x128124 + (i * 0x0800));
    let mut stat: u32 = nvkm_rd32(device, 0x128128 + (i * 0x0800));
    nvkm_debug(privring, "GPC%d: %06x %08x (%08x)\n", i, addr, data, stat);
    }
    void
    gk104_privring_intr(struct nvkm_subdev *privring)
    {
    struct nvkm_device *device = privring.device;
    let mut intr0: u32 = nvkm_rd32(device, 0x120058);
    let mut intr1: u32 = nvkm_rd32(device, 0x12005c);
    let mut hubnr: u32 = nvkm_rd32(device, 0x120070);
    let mut ropnr: u32 = nvkm_rd32(device, 0x120074);
    let mut gpcnr: u32 = nvkm_rd32(device, 0x120078);
    u32 i;
    for (i = 0; (intr0 & 0x0000ff00) && i < hubnr; i++) {
    let mut stat: u32 = 0x00000100 << i;
    if (intr0 & stat) {
    gk104_privring_intr_hub(privring, i);
    intr0 &= ~stat;
    }
    }
    for (i = 0; (intr0 & 0xffff0000) && i < ropnr; i++) {
    let mut stat: u32 = 0x00010000 << i;
    if (intr0 & stat) {
    gk104_privring_intr_rop(privring, i);
    intr0 &= ~stat;
    }
    }
    for (i = 0; intr1 && i < gpcnr; i++) {
    let mut stat: u32 = 0x00000001 << i;
    if (intr1 & stat) {
    gk104_privring_intr_gpc(privring, i);
    intr1 &= ~stat;
    }
    }
    nvkm_mask(device, 0x12004c, 0x0000003f, 0x00000002);
    nvkm_msec(device, 2000,
    if (!(nvkm_rd32(device, 0x12004c) & 0x0000003f))
    break;
    );
    }
    static int
    gk104_privring_init(struct nvkm_subdev *privring)
    {
    struct nvkm_device *device = privring.device;
    nvkm_mask(device, 0x122318, 0x0003ffff, 0x00001000);
    nvkm_mask(device, 0x12231c, 0x0003ffff, 0x00000200);
    nvkm_mask(device, 0x122310, 0x0003ffff, 0x00000800);
    nvkm_mask(device, 0x122348, 0x0003ffff, 0x00000100);
    nvkm_mask(device, 0x1223b0, 0x0003ffff, 0x00000fff);
    nvkm_mask(device, 0x122348, 0x0003ffff, 0x00000200);
    nvkm_mask(device, 0x122358, 0x0003ffff, 0x00002880);
    return 0;
    }
    static const struct nvkm_subdev_func
    gk104_privring = {
    .preinit = gk104_privring_init,
    .init = gk104_privring_init,
    .intr = gk104_privring_intr,
    };
    int
    gk104_privring_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_subdev **pprivring)
    {
    return nvkm_subdev_new_(&gk104_privring, device, type, inst, pprivring);
    }
