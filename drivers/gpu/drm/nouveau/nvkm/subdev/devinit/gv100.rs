//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/devinit/gv100.c
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
// Copyright 2018 Red Hat Inc.
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

    static int
    gv100_devinit_pll_set(struct nvkm_devinit *init, u32 type, u32 freq)
    {
    struct nvkm_subdev *subdev = &init.subdev;
    struct nvkm_device *device = subdev.device;
    struct nvbios_pll info;
    let mut head: c_int = type - PLL_VPLL0;
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
    nvkm_wr32(device, 0x00ef10 + (head * 0x40), fN << 16);
    nvkm_wr32(device, 0x00ef04 + (head * 0x40), (P << 16) |
    (N <<  8) |
    (M <<  0));
    break;
    default:
    nvkm_warn(subdev, "%08x/%dKhz unimplemented\n", type, freq);
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct nvkm_devinit_func
    gv100_devinit = {
    .preinit = gf100_devinit_preinit,
    .init = nv50_devinit_init,
    .post = gm200_devinit_post,
    .pll_set = gv100_devinit_pll_set,
    .disable = gm107_devinit_disable,
    };
    int
    gv100_devinit_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_devinit **pinit)
    {
    return nv50_devinit_new_(&gv100_devinit, device, type, inst, pinit);
    }
