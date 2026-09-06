//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/top/ga100.c
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
// Copyright 2021 Red Hat Inc.
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
    ga100_top_parse(struct nvkm_top *top)
    {
    struct nvkm_subdev *subdev = &top.subdev;
    struct nvkm_device *device = subdev.device;
    struct nvkm_top_device *info = core::ptr::null_mut();
    u32 data, type, inst;
    int i, n, size = nvkm_rd32(device, 0x0224fc) >> 20;
    for (i = 0, n = 0; i < size; i++) {
    if (!info) {
    if (!(info = nvkm_top_device_new(top)))
    return -ENOMEM;
    type = ~0;
    inst = 0;
    }
    data = nvkm_rd32(device, 0x022800 + (i * 0x04));
    nvkm_trace(subdev, "%02x: %08x\n", i, data);
    if (!data && n == 0)
    continue;
    switch (n++) {
    case 0:
    type	      = (data & 0x3f000000) >> 24;
    inst	      = (data & 0x000f0000) >> 16;
    info.fault   = (data & 0x0000007f);
    break;
    case 1:
    info.addr    = (data & 0x00fff000);
    info.reset   = (data & 0x0000001f);
    break;
    case 2:
    info.runlist = (data & 0x00fffc00);
    info.engine  = (data & 0x00000003);
    break;
    default:
    break;
    }
    if (data & 0x80000000)
    continue;
    n = 0;
// Translate engine type to NVKM engine identifier.

    switch (type) {
    case 0x00000000: O_(NVKM_ENGINE_GR    ,    0); break;
    case 0x0000000d: O_(NVKM_ENGINE_SEC2  ,    0); break;
    case 0x0000000e: I_(NVKM_ENGINE_NVENC , inst); break;
    case 0x00000010: I_(NVKM_ENGINE_NVDEC , inst); break;
    case 0x00000012: I_(NVKM_SUBDEV_IOCTRL, inst); break;
    case 0x00000013: I_(NVKM_ENGINE_CE    , inst); break;
    case 0x00000014: O_(NVKM_SUBDEV_GSP   ,    0); break;
    case 0x00000015: I_(NVKM_ENGINE_NVJPG , inst); break;
    case 0x00000016: O_(NVKM_ENGINE_OFA   ,    0); break;
    case 0x00000017: O_(NVKM_SUBDEV_FLA   ,    0); break;
    break;
    default:
    break;
    }
    nvkm_debug(subdev, "%02x.%d (%8s): addr %06x fault %2d "
    "runlist %6x engine %2d reset %2d\n", type, inst,
    info.type == NVKM_SUBDEV_NR ? "????????" : nvkm_subdev_type[info.type],
    info.addr, info.fault, info.runlist < 0 ? 0 : info.runlist,
    info.engine, info.reset);
    info = core::ptr::null_mut();
    }
    return 0;
    }
    static const struct nvkm_top_func
    ga100_top = {
    .parse = ga100_top_parse,
    };
    int
    ga100_top_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_top **ptop)
    {
    if (nvkm_gsp_rm(device.gsp))
    return -ENODEV;
    return nvkm_top_new_(&ga100_top, device, type, inst, ptop);
    }
