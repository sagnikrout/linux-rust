//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/dma/base.c
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

    static int
    nvkm_dma_oclass_new(struct nvkm_device *device,
    const struct nvkm_oclass *oclass, void *data, u32 size,
    struct nvkm_object **pobject)
    {
    struct nvkm_dma *dma = nvkm_dma(oclass.engine);
    struct nvkm_dmaobj *dmaobj = core::ptr::null_mut();
    int ret;
    ret = dma.func.class_new(dma, oclass, data, size, &dmaobj);
    if (dmaobj)
// pobject = &dmaobj->object;
    return ret;
    }
    static const struct nvkm_device_oclass
    nvkm_dma_oclass_base = {
    .ctor = nvkm_dma_oclass_new,
    };
    static int
    nvkm_dma_oclass_fifo_new(const struct nvkm_oclass *oclass, void *data, u32 size,
    struct nvkm_object **pobject)
    {
    return nvkm_dma_oclass_new(oclass.engine.subdev.device,
    oclass, data, size, pobject);
    }
    static const struct nvkm_sclass
    nvkm_dma_sclass[] = {
    { 0, 0, NV_DMA_FROM_MEMORY, core::ptr::null_mut(), nvkm_dma_oclass_fifo_new },
    { 0, 0, NV_DMA_TO_MEMORY, core::ptr::null_mut(), nvkm_dma_oclass_fifo_new },
    { 0, 0, NV_DMA_IN_MEMORY, core::ptr::null_mut(), nvkm_dma_oclass_fifo_new },
    };
    static int
    nvkm_dma_oclass_base_get(struct nvkm_oclass *sclass, int index,
    const struct nvkm_device_oclass **class)
    {
    let mut count: c_int = ARRAY_SIZE(nvkm_dma_sclass);
    if (index < count) {
    const struct nvkm_sclass *oclass = &nvkm_dma_sclass[index];
    sclass.base = oclass[0];
    sclass.engn = oclass;
// class = &nvkm_dma_oclass_base;
    return index;
    }
    return count;
    }
    static int
    nvkm_dma_oclass_fifo_get(struct nvkm_oclass *oclass, int index)
    {
    let mut count: c_int = ARRAY_SIZE(nvkm_dma_sclass);
    if (index < count) {
    oclass.base = nvkm_dma_sclass[index];
    return index;
    }
    return count;
    }
    static void *
    nvkm_dma_dtor(struct nvkm_engine *engine)
    {
    return nvkm_dma(engine);
    }
    static const struct nvkm_engine_func
    nvkm_dma = {
    .dtor = nvkm_dma_dtor,
    .base.sclass = nvkm_dma_oclass_base_get,
    .fifo.sclass = nvkm_dma_oclass_fifo_get,
    };
    int
    nvkm_dma_new_(const struct nvkm_dma_func *func, struct nvkm_device *device,
    enum nvkm_subdev_type type, int inst, struct nvkm_dma **pdma)
    {
    struct nvkm_dma *dma;
    if (!(dma = *pdma = kzalloc_obj(*dma)))
    return -ENOMEM;
    dma.func = func;
    return nvkm_engine_ctor(&nvkm_dma, device, type, inst, true, &dma.engine);
    }
