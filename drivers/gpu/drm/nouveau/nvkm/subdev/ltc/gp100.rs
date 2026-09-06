//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/ltc/gp100.c
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
// Copyright 2016 Red Hat Inc.
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

    void
    gp100_ltc_intr(struct nvkm_ltc *ltc)
    {
    struct nvkm_device *device = ltc.subdev.device;
    u32 mask;
    mask = nvkm_rd32(device, 0x0001c0);
    while (mask) {
    u32 s, c = __ffs(mask);
    for (s = 0; s < ltc.lts_nr; s++)
    gm107_ltc_intr_lts(ltc, c, s);
    mask &= ~(1 << c);
    }
    }
    int
    gp100_ltc_oneinit(struct nvkm_ltc *ltc)
    {
    struct nvkm_device *device = ltc.subdev.device;
    ltc.ltc_nr = nvkm_rd32(device, 0x12006c);
    ltc.lts_nr = nvkm_rd32(device, 0x17e280) >> 28;
// XXX: tagram allocation - TBD
    return 0;
    }
    void
    gp100_ltc_init(struct nvkm_ltc *ltc)
    {
// XXX: PMU LS call to setup tagram address
    }
    static const struct nvkm_ltc_func
    gp100_ltc = {
    .oneinit = gp100_ltc_oneinit,
    .init = gp100_ltc_init,
    .intr = gp100_ltc_intr,
    .cbc_clear = gm107_ltc_cbc_clear,
    .cbc_wait = gm107_ltc_cbc_wait,
    .zbc_color = 16,
    .zbc_depth = 16,
    .zbc_clear_color = gm107_ltc_zbc_clear_color,
    .zbc_clear_depth = gm107_ltc_zbc_clear_depth,
    .invalidate = gf100_ltc_invalidate,
    .flush = gf100_ltc_flush,
    };
    int
    gp100_ltc_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_ltc **pltc)
    {
    return nvkm_ltc_new_(&gp100_ltc, device, type, inst, pltc);
    }
