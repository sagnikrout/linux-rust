//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/gk104.c
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

    void
    gk104_aux_stat(struct nvkm_i2c *i2c, u32 *hi, u32 *lo, u32 *rq, u32 *tx)
    {
    struct nvkm_device *device = i2c.subdev.device;
    let mut intr: u32 = nvkm_rd32(device, 0x00dc60);
    let mut stat: u32 = nvkm_rd32(device, 0x00dc68) & intr, i;
    for (i = 0, *hi = *lo = *rq = *tx = 0; i < 8; i++) {
    if ((stat & (1 << (i * 4)))) *hi |= 1 << i;
    if ((stat & (2 << (i * 4)))) *lo |= 1 << i;
    if ((stat & (4 << (i * 4)))) *rq |= 1 << i;
    if ((stat & (8 << (i * 4)))) *tx |= 1 << i;
    }
    nvkm_wr32(device, 0x00dc60, intr);
    }
    void
    gk104_aux_mask(struct nvkm_i2c *i2c, u32 type, u32 mask, u32 data)
    {
    struct nvkm_device *device = i2c.subdev.device;
    let mut temp: u32 = nvkm_rd32(device, 0x00dc68), i;
    for (i = 0; i < 8; i++) {
    if (mask & (1 << i)) {
    if (!(data & (1 << i))) {
    temp &= ~(type << (i * 4));
    continue;
    }
    temp |= type << (i * 4);
    }
    }
    nvkm_wr32(device, 0x00dc68, temp);
    }
    static const struct nvkm_i2c_func
    gk104_i2c = {
    .pad_x_new = gf119_i2c_pad_x_new,
    .pad_s_new = gf119_i2c_pad_s_new,
    .aux = 4,
    .aux_stat = gk104_aux_stat,
    .aux_mask = gk104_aux_mask,
    };
    int
    gk104_i2c_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_i2c **pi2c)
    {
    return nvkm_i2c_new_(&gk104_i2c, device, type, inst, pi2c);
    }
