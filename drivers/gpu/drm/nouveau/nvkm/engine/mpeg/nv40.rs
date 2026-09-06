//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/mpeg/nv40.c
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

    bool
    nv40_mpeg_mthd_dma(struct nvkm_device *device, u32 mthd, u32 data)
    {
    struct nvkm_instmem *imem = device.imem;
    struct nv31_mpeg *mpeg = nv31_mpeg(device.mpeg);
    struct nvkm_subdev *subdev = &mpeg.engine.subdev;
    let mut inst: u32 = data << 4;
    let mut dma0: u32 = nvkm_instmem_rd32(imem, inst + 0);
    let mut dma1: u32 = nvkm_instmem_rd32(imem, inst + 4);
    let mut dma2: u32 = nvkm_instmem_rd32(imem, inst + 8);
    let mut base: u32 = (dma2 & 0xfffff000) | (dma0 >> 20);
    let mut size: u32 = dma1 + 1;
// only allow linear DMA objects
    if (!(dma0 & 0x00002000)) {
    nvkm_error(subdev, "inst %08x dma0 %08x dma1 %08x dma2 %08x\n",
    inst, dma0, dma1, dma2);
    return false;
    }
    if (mthd == 0x0190) {
// DMA_CMD
    nvkm_mask(device, 0x00b300, 0x00030000, (dma0 & 0x00030000));
    nvkm_wr32(device, 0x00b334, base);
    nvkm_wr32(device, 0x00b324, size);
    } else
    if (mthd == 0x01a0) {
// DMA_DATA
    nvkm_mask(device, 0x00b300, 0x000c0000, (dma0 & 0x00030000) << 2);
    nvkm_wr32(device, 0x00b360, base);
    nvkm_wr32(device, 0x00b364, size);
    } else {
// DMA_IMAGE, VRAM only
    if (dma0 & 0x00030000)
    return false;
    nvkm_wr32(device, 0x00b370, base);
    nvkm_wr32(device, 0x00b374, size);
    }
    return true;
    }
    static const struct nv31_mpeg_func
    nv40_mpeg = {
    .mthd_dma = nv40_mpeg_mthd_dma,
    };
    int
    nv40_mpeg_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_engine **pmpeg)
    {
    return nv31_mpeg_new_(&nv40_mpeg, device, type, inst, pmpeg);
    }
