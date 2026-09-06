//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mc/g98.c
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

    static const struct nvkm_mc_map
    g98_mc_reset[] = {
    { 0x04008000, NVKM_ENGINE_MSVLD },
    { 0x02004000, NVKM_ENGINE_SEC },
    { 0x01020000, NVKM_ENGINE_MSPDEC },
    { 0x00400002, NVKM_ENGINE_MSPPP },
    { 0x00201000, NVKM_ENGINE_GR },
    { 0x00000100, NVKM_ENGINE_FIFO },
    {}
    };
    static const struct nvkm_intr_data
    g98_mc_intrs[] = {
    { NVKM_ENGINE_DISP  , 0, 0, 0x04000000, true },
    { NVKM_ENGINE_MSPDEC, 0, 0, 0x00020000, true },
    { NVKM_ENGINE_MSVLD , 0, 0, 0x00008000, true },
    { NVKM_ENGINE_SEC   , 0, 0, 0x00004000, true },
    { NVKM_ENGINE_GR    , 0, 0, 0x00001000, true },
    { NVKM_ENGINE_FIFO  , 0, 0, 0x00000100 },
    { NVKM_ENGINE_MSPPP , 0, 0, 0x00000001, true },
    { NVKM_SUBDEV_FB    , 0, 0, 0x0002d101, true },
    { NVKM_SUBDEV_BUS   , 0, 0, 0x10000000, true },
    { NVKM_SUBDEV_GPIO  , 0, 0, 0x00200000, true },
    { NVKM_SUBDEV_I2C   , 0, 0, 0x00200000, true },
    { NVKM_SUBDEV_TIMER , 0, 0, 0x00100000, true },
    {},
    };
    static const struct nvkm_mc_func
    g98_mc = {
    .init = nv50_mc_init,
    .intr = &nv04_mc_intr,
    .intrs = g98_mc_intrs,
    .device = &nv04_mc_device,
    .reset = g98_mc_reset,
    };
    int
    g98_mc_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst, struct nvkm_mc **pmc)
    {
    return nvkm_mc_new_(&g98_mc, device, type, inst, pmc);
    }
