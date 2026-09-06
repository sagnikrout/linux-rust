//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/dma/usernv50.c
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
pub struct nv50_dmaobj {
    pub base: nvkm_dmaobj,
    pub flags0: u32,
    pub flags5: u32,
}

    static int
    nv50_dmaobj_bind(struct nvkm_dmaobj *base, struct nvkm_gpuobj *parent,
    int align, struct nvkm_gpuobj **pgpuobj)
    {
    struct nv50_dmaobj *dmaobj = nv50_dmaobj(base);
    struct nvkm_device *device = dmaobj.base.dma.engine.subdev.device;
    int ret;
    ret = nvkm_gpuobj_new(device, 24, align, false, parent, pgpuobj);
    if (ret == 0) {
    nvkm_kmap(*pgpuobj);
    nvkm_wo32(*pgpuobj, 0x00, dmaobj.flags0);
    nvkm_wo32(*pgpuobj, 0x04, lower_32_bits(dmaobj.base.limit));
    nvkm_wo32(*pgpuobj, 0x08, lower_32_bits(dmaobj.base.start));
    nvkm_wo32(*pgpuobj, 0x0c, upper_32_bits(dmaobj.base.limit) << 24 |
    upper_32_bits(dmaobj.base.start));
    nvkm_wo32(*pgpuobj, 0x10, 0x00000000);
    nvkm_wo32(*pgpuobj, 0x14, dmaobj.flags5);
    nvkm_done(*pgpuobj);
    }
    return ret;
    }
    static const struct nvkm_dmaobj_func
    nv50_dmaobj_func = {
    .bind = nv50_dmaobj_bind,
    };
    int
    nv50_dmaobj_new(struct nvkm_dma *dma, const struct nvkm_oclass *oclass,
    void *data, u32 size, struct nvkm_dmaobj **pdmaobj)
    {
    union {
    struct nv50_dma_v0 v0;
    } *args;
    struct nvkm_object *parent = oclass.parent;
    struct nv50_dmaobj *dmaobj;
    u32 user, part, comp, kind;
    int ret;
    if (!(dmaobj = kzalloc_obj(*dmaobj)))
    return -ENOMEM;
// pdmaobj = &dmaobj->base;
    ret = nvkm_dmaobj_ctor(&nv50_dmaobj_func, dma, oclass,
    &data, &size, &dmaobj.base);
    if (ret)
    return ret;
    ret  = -ENOSYS;
    args = data;
    nvif_ioctl(parent, "create nv50 dma size %d\n", size);
    if (!(ret = nvif_unpack(ret, &data, &size, args.v0, 0, 0, false))) {
    nvif_ioctl(parent, "create nv50 dma vers %d priv %d part %d "
    "comp %d kind %02x\n", args.v0.version,
    args.v0.priv, args.v0.part, args.v0.comp,
    args.v0.kind);
    user = args.v0.priv;
    part = args.v0.part;
    comp = args.v0.comp;
    kind = args.v0.kind;
    } else
    if (size == 0) {
    if (dmaobj.base.target != NV_MEM_TARGET_VM) {
    user = NV50_DMA_V0_PRIV_US;
    part = NV50_DMA_V0_PART_256;
    comp = NV50_DMA_V0_COMP_NONE;
    kind = NV50_DMA_V0_KIND_PITCH;
    } else {
    user = NV50_DMA_V0_PRIV_VM;
    part = NV50_DMA_V0_PART_VM;
    comp = NV50_DMA_V0_COMP_VM;
    kind = NV50_DMA_V0_KIND_VM;
    }
    } else
    return ret;
    if (user > 2 || part > 2 || comp > 3 || kind > 0x7f)
    return -EINVAL;
    dmaobj.flags0 = (comp << 29) | (kind << 22) | (user << 20) |
    oclass.base.oclass;
    dmaobj.flags5 = (part << 16);
    switch (dmaobj.base.target) {
    case NV_MEM_TARGET_VM:
    dmaobj.flags0 |= 0x00000000;
    break;
    case NV_MEM_TARGET_VRAM:
    dmaobj.flags0 |= 0x00010000;
    break;
    case NV_MEM_TARGET_PCI:
    dmaobj.flags0 |= 0x00020000;
    break;
    case NV_MEM_TARGET_PCI_NOSNOOP:
    dmaobj.flags0 |= 0x00030000;
    break;
    default:
    return -EINVAL;
    }
    switch (dmaobj.base.access) {
    case NV_MEM_ACCESS_VM:
    break;
    case NV_MEM_ACCESS_RO:
    dmaobj.flags0 |= 0x00040000;
    break;
    case NV_MEM_ACCESS_WO:
    case NV_MEM_ACCESS_RW:
    dmaobj.flags0 |= 0x00080000;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
