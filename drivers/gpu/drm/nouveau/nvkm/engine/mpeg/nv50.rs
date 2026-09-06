//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/mpeg/nv50.c
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

//
// PMPEG context
//
    static int
    nv50_mpeg_cclass_bind(struct nvkm_object *object, struct nvkm_gpuobj *parent,
    int align, struct nvkm_gpuobj **pgpuobj)
    {
    int ret = nvkm_gpuobj_new(object.engine.subdev.device, 128 * 4,
    align, true, parent, pgpuobj);
    if (ret == 0) {
    nvkm_kmap(*pgpuobj);
    nvkm_wo32(*pgpuobj, 0x70, 0x00801ec1);
    nvkm_wo32(*pgpuobj, 0x7c, 0x0000037c);
    nvkm_done(*pgpuobj);
    }
    return ret;
    }
    const struct nvkm_object_func
    nv50_mpeg_cclass = {
    .bind = nv50_mpeg_cclass_bind,
    };
//
// PMPEG engine/subdev functions
//
    void
    nv50_mpeg_intr(struct nvkm_engine *mpeg)
    {
    struct nvkm_subdev *subdev = &mpeg.subdev;
    struct nvkm_device *device = subdev.device;
    let mut stat: u32 = nvkm_rd32(device, 0x00b100);
    let mut type: u32 = nvkm_rd32(device, 0x00b230);
    let mut mthd: u32 = nvkm_rd32(device, 0x00b234);
    let mut data: u32 = nvkm_rd32(device, 0x00b238);
    let mut show: u32 = stat;
    if (stat & 0x01000000) {
// happens on initial binding of the object
    if (type == 0x00000020 && mthd == 0x0000) {
    nvkm_wr32(device, 0x00b308, 0x00000100);
    show &= ~0x01000000;
    }
    }
    if (show) {
    nvkm_info(subdev, "%08x %08x %08x %08x\n",
    stat, type, mthd, data);
    }
    nvkm_wr32(device, 0x00b100, stat);
    nvkm_wr32(device, 0x00b230, 0x00000001);
    }
    int
    nv50_mpeg_init(struct nvkm_engine *mpeg)
    {
    struct nvkm_subdev *subdev = &mpeg.subdev;
    struct nvkm_device *device = subdev.device;
    nvkm_wr32(device, 0x00b32c, 0x00000000);
    nvkm_wr32(device, 0x00b314, 0x00000100);
    nvkm_wr32(device, 0x00b0e0, 0x0000001a);
    nvkm_wr32(device, 0x00b220, 0x00000044);
    nvkm_wr32(device, 0x00b300, 0x00801ec1);
    nvkm_wr32(device, 0x00b390, 0x00000000);
    nvkm_wr32(device, 0x00b394, 0x00000000);
    nvkm_wr32(device, 0x00b398, 0x00000000);
    nvkm_mask(device, 0x00b32c, 0x00000001, 0x00000001);
    nvkm_wr32(device, 0x00b100, 0xffffffff);
    nvkm_wr32(device, 0x00b140, 0xffffffff);
    if (nvkm_msec(device, 2000,
    if (!(nvkm_rd32(device, 0x00b200) & 0x00000001))
    break;
    ) < 0) {
    nvkm_error(subdev, "timeout %08x\n",
    nvkm_rd32(device, 0x00b200));
    return -EBUSY;
    }
    return 0;
    }
    static const struct nvkm_engine_func
    nv50_mpeg = {
    .init = nv50_mpeg_init,
    .intr = nv50_mpeg_intr,
    .cclass = &nv50_mpeg_cclass,
    .sclass = {
    { -1, -1, NV31_MPEG, &nv31_mpeg_object },
    {}
    }
    };
    int
    nv50_mpeg_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_engine **pmpeg)
    {
    return nvkm_engine_new_(&nv50_mpeg, device, type, inst, true, pmpeg);
    }
