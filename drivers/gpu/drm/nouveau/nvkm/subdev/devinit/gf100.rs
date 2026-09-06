//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/devinit/gf100.c
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
// Copyright 2013 Red Hat Inc.
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

    int
    gf100_devinit_pll_set(struct nvkm_devinit *init, u32 type, u32 freq)
    {
    struct nvkm_subdev *subdev = &init.subdev;
    struct nvkm_device *device = subdev.device;
    struct nvbios_pll info;
    int N, fN, M, P;
    int ret;
    ret = nvbios_pll_parse(device.bios, type, &info);
    if (ret)
    return ret;
    ret = gt215_pll_calc(subdev, &info, freq, &N, &fN, &M, &P);
    if (ret < 0)
    return ret;
    switch (info.type) {
    case PLL_VPLL0:
    case PLL_VPLL1:
    case PLL_VPLL2:
    case PLL_VPLL3:
    nvkm_mask(device, info.reg + 0x0c, 0x00000000, 0x00000100);
    nvkm_wr32(device, info.reg + 0x04, (P << 16) | (N << 8) | M);
    nvkm_wr32(device, info.reg + 0x10, fN << 16);
    break;
    default:
    nvkm_warn(subdev, "%08x/%dKhz unimplemented\n", type, freq);
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static void
    gf100_devinit_disable(struct nvkm_devinit *init)
    {
    struct nvkm_device *device = init.subdev.device;
    let mut r022500: u32 = nvkm_rd32(device, 0x022500);
    if (r022500 & 0x00000001)
    nvkm_subdev_disable(device, NVKM_ENGINE_DISP, 0);
    if (r022500 & 0x00000002) {
    nvkm_subdev_disable(device, NVKM_ENGINE_MSPDEC, 0);
    nvkm_subdev_disable(device, NVKM_ENGINE_MSPPP, 0);
    }
    if (r022500 & 0x00000004)
    nvkm_subdev_disable(device, NVKM_ENGINE_MSVLD, 0);
    if (r022500 & 0x00000008)
    nvkm_subdev_disable(device, NVKM_ENGINE_MSENC, 0);
    if (r022500 & 0x00000100)
    nvkm_subdev_disable(device, NVKM_ENGINE_CE, 0);
    if (r022500 & 0x00000200)
    nvkm_subdev_disable(device, NVKM_ENGINE_CE, 1);
    }
    void
    gf100_devinit_preinit(struct nvkm_devinit *base)
    {
    struct nv50_devinit *init = nv50_devinit(base);
    struct nvkm_subdev *subdev = &init.base.subdev;
    struct nvkm_device *device = subdev.device;
//
// This bit is set by devinit, and flips back to 0 on suspend. We
// can use it as a reliable way to know whether we should run devinit.
//
    base.post = ((nvkm_rd32(device, 0x2240c) & BIT(1)) == 0);
    }
    static const struct nvkm_devinit_func
    gf100_devinit = {
    .preinit = gf100_devinit_preinit,
    .init = nv50_devinit_init,
    .post = nv04_devinit_post,
    .pll_set = gf100_devinit_pll_set,
    .disable = gf100_devinit_disable,
    };
    int
    gf100_devinit_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_devinit **pinit)
    {
    return nv50_devinit_new_(&gf100_devinit, device, type, inst, pinit);
    }
