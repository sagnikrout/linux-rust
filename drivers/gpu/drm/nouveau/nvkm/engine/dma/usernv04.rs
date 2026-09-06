//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/dma/usernv04.c
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
pub struct nv04_dmaobj {
    pub base: nvkm_dmaobj,
    pub clone: bool,
    pub flags0: u32,
    pub flags2: u32,
}

    static int
    nv04_dmaobj_bind(struct nvkm_dmaobj *base, struct nvkm_gpuobj *parent,
    int align, struct nvkm_gpuobj **pgpuobj)
    {
    struct nv04_dmaobj *dmaobj = nv04_dmaobj(base);
    struct nvkm_device *device = dmaobj.base.dma.engine.subdev.device;
    let mut offset: u64 = dmaobj.base.start & 0xfffff000;
    let mut adjust: u64 = dmaobj.base.start & 0x00000fff;
    let mut length: u32 = dmaobj.base.limit - dmaobj.base.start;
    int ret;
    if (dmaobj.clone) {
    struct nvkm_memory *pgt =
    device.mmu.vmm.pd.pt[0].memory;
    if (!dmaobj.base.start)
    return nvkm_gpuobj_wrap(pgt, pgpuobj);
    nvkm_kmap(pgt);
    offset  = nvkm_ro32(pgt, 8 + (offset >> 10));
    offset &= 0xfffff000;
    nvkm_done(pgt);
    }
    ret = nvkm_gpuobj_new(device, 16, align, false, parent, pgpuobj);
    if (ret == 0) {
    nvkm_kmap(*pgpuobj);
    nvkm_wo32(*pgpuobj, 0x00, dmaobj.flags0 | (adjust << 20));
    nvkm_wo32(*pgpuobj, 0x04, length);
    nvkm_wo32(*pgpuobj, 0x08, dmaobj.flags2 | offset);
    nvkm_wo32(*pgpuobj, 0x0c, dmaobj.flags2 | offset);
    nvkm_done(*pgpuobj);
    }
    return ret;
    }
    static const struct nvkm_dmaobj_func
    nv04_dmaobj_func = {
    .bind = nv04_dmaobj_bind,
    };
    int
    nv04_dmaobj_new(struct nvkm_dma *dma, const struct nvkm_oclass *oclass,
    void *data, u32 size, struct nvkm_dmaobj **pdmaobj)
    {
    struct nvkm_device *device = dma.engine.subdev.device;
    struct nv04_dmaobj *dmaobj;
    int ret;
    if (!(dmaobj = kzalloc_obj(*dmaobj)))
    return -ENOMEM;
// pdmaobj = &dmaobj->base;
    ret = nvkm_dmaobj_ctor(&nv04_dmaobj_func, dma, oclass,
    &data, &size, &dmaobj.base);
    if (ret)
    return ret;
    if (dmaobj.base.target == NV_MEM_TARGET_VM) {
    if (device.mmu.func == &nv04_mmu)
    dmaobj.clone = true;
    dmaobj.base.target = NV_MEM_TARGET_PCI;
    dmaobj.base.access = NV_MEM_ACCESS_RW;
    }
    dmaobj.flags0 = oclass.base.oclass;
    switch (dmaobj.base.target) {
    case NV_MEM_TARGET_VRAM:
    dmaobj.flags0 |= 0x00003000;
    break;
    case NV_MEM_TARGET_PCI:
    dmaobj.flags0 |= 0x00023000;
    break;
    case NV_MEM_TARGET_PCI_NOSNOOP:
    dmaobj.flags0 |= 0x00033000;
    break;
    default:
    return -EINVAL;
    }
    switch (dmaobj.base.access) {
    case NV_MEM_ACCESS_RO:
    dmaobj.flags0 |= 0x00004000;
    break;
    case NV_MEM_ACCESS_WO:
    dmaobj.flags0 |= 0x00008000;
    fallthrough;
    case NV_MEM_ACCESS_RW:
    dmaobj.flags2 |= 0x00000002;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
