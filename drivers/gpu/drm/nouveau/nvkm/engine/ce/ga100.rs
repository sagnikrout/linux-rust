//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/ce/ga100.c
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
// Copyright 2021 Red Hat Inc.
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

    static irqreturn_t
    ga100_ce_intr(struct nvkm_inth *inth)
    {
    struct nvkm_subdev *subdev = container_of(inth, typeof(*subdev), inth);
// TODO
    nvkm_error(subdev, "intr\n");
    return IRQ_NONE;
    }
    int
    ga100_ce_nonstall(struct nvkm_engine *engine)
    {
    struct nvkm_subdev *subdev = &engine.subdev;
    struct nvkm_device *device = subdev.device;
    return nvkm_rd32(device, 0x104424 + (subdev.inst * 0x80)) & 0x00000fff;
    }
    int
    ga100_ce_fini(struct nvkm_engine *engine, enum nvkm_suspend_state suspend)
    {
    nvkm_inth_block(&engine.subdev.inth);
    return 0;
    }
    int
    ga100_ce_init(struct nvkm_engine *engine)
    {
    nvkm_inth_allow(&engine.subdev.inth);
    return 0;
    }
    int
    ga100_ce_oneinit(struct nvkm_engine *engine)
    {
    struct nvkm_subdev *subdev = &engine.subdev;
    struct nvkm_device *device = subdev.device;
    u32 vector;
    vector = nvkm_rd32(device, 0x10442c + (subdev.inst * 0x80)) & 0x00000fff;
    return nvkm_inth_add(&device.vfn.intr, vector, NVKM_INTR_PRIO_NORMAL,
    subdev, ga100_ce_intr, &subdev.inth);
    }
    static const struct nvkm_engine_func
    ga100_ce = {
    .oneinit = ga100_ce_oneinit,
    .init = ga100_ce_init,
    .fini = ga100_ce_fini,
    .nonstall = ga100_ce_nonstall,
    .cclass = &gv100_ce_cclass,
    .sclass = {
    { -1, -1, AMPERE_DMA_COPY_A },
    {}
    }
    };
    int
    ga100_ce_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_engine **pengine)
    {
    if (nvkm_gsp_rm(device.gsp))
    return -ENODEV;
    return nvkm_engine_new_(&ga100_ce, device, type, inst, true, pengine);
    }
