//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/ramgm200.c
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
// Copyright 2017 Red Hat Inc.
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//

    u32
    gm200_ram_probe_fbp_amount(const struct nvkm_ram_func *func, u32 fbpao,
    struct nvkm_device *device, int fbp, int *pltcs)
    {
    let mut ltcs: u32 = nvkm_rd32(device, 0x022450);
    let mut fbpas: u32 = nvkm_rd32(device, 0x022458);
    let mut fbpa: u32 = fbp * fbpas;
    let mut size: u32 = 0;
    if (!(nvkm_rd32(device, 0x021d38) & BIT(fbp))) {
    let mut ltco: u32 = nvkm_rd32(device, 0x021d70 + (fbp * 4));
    let mut ltcm: u32 = ~ltco & ((1 << ltcs) - 1);
    while (fbpas--) {
    if (!(fbpao & (1 << fbpa)))
    size += func.probe_fbpa_amount(device, fbpa);
    fbpa++;
    }
// pltcs = hweight32(ltcm);
    }
    return size;
    }
    static const struct nvkm_ram_func
    gm200_ram = {
    .upper = 0x1000000000ULL,
    .probe_fbp = gm107_ram_probe_fbp,
    .probe_fbp_amount = gm200_ram_probe_fbp_amount,
    .probe_fbpa_amount = gf100_ram_probe_fbpa_amount,
    .dtor = gk104_ram_dtor,
    .init = gk104_ram_init,
    .calc = gk104_ram_calc,
    .prog = gk104_ram_prog,
    .tidy = gk104_ram_tidy,
    };
    int
    gm200_ram_new(struct nvkm_fb *fb, struct nvkm_ram **pram)
    {
    return gk104_ram_new_(&gm200_ram, fb, pram);
    }
