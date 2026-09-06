//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/padgm200.c
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
// Copyright 2014 Red Hat Inc.
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

    static void
    gm200_i2c_pad_mode(struct nvkm_i2c_pad *pad, enum nvkm_i2c_pad_mode mode)
    {
    struct nvkm_subdev *subdev = &pad.i2c.subdev;
    struct nvkm_device *device = subdev.device;
    let mut base: u32 = (pad.id - NVKM_I2C_PAD_HYBRID(0)) * 0x50;
    switch (mode) {
    case NVKM_I2C_PAD_OFF:
    nvkm_mask(device, 0x00d97c + base, 0x00000001, 0x00000001);
    break;
    case NVKM_I2C_PAD_I2C:
    nvkm_mask(device, 0x00d970 + base, 0x0000c003, 0x0000c001);
    nvkm_mask(device, 0x00d97c + base, 0x00000001, 0x00000000);
    break;
    case NVKM_I2C_PAD_AUX:
    nvkm_mask(device, 0x00d970 + base, 0x0000c003, 0x00000002);
    nvkm_mask(device, 0x00d97c + base, 0x00000001, 0x00000000);
    break;
    default:
    WARN_ON(1);
    break;
    }
    }
    static const struct nvkm_i2c_pad_func
    gm200_i2c_pad_s_func = {
    .bus_new_4 = gf119_i2c_bus_new,
    .aux_new_6 = gm200_i2c_aux_new,
    .mode = gm200_i2c_pad_mode,
    };
    int
    gm200_i2c_pad_s_new(struct nvkm_i2c *i2c, int id, struct nvkm_i2c_pad **ppad)
    {
    return nvkm_i2c_pad_new_(&gm200_i2c_pad_s_func, i2c, id, ppad);
    }
    static const struct nvkm_i2c_pad_func
    gm200_i2c_pad_x_func = {
    .bus_new_4 = gf119_i2c_bus_new,
    .aux_new_6 = gm200_i2c_aux_new,
    };
    int
    gm200_i2c_pad_x_new(struct nvkm_i2c *i2c, int id, struct nvkm_i2c_pad **ppad)
    {
    return nvkm_i2c_pad_new_(&gm200_i2c_pad_x_func, i2c, id, ppad);
    }
