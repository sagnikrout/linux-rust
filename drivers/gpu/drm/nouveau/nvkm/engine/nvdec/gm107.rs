//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/nvdec/gm107.c
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
// Copyright (c) 2017, NVIDIA CORPORATION. All rights reserved.
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
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

    static const struct nvkm_falcon_func
    gm107_nvdec_flcn = {
    .disable = gm200_flcn_disable,
    .enable = gm200_flcn_enable,
    .reset_pmc = true,
    .reset_wait_mem_scrubbing = gm200_flcn_reset_wait_mem_scrubbing,
    .debug = 0xd00,
    .imem_pio = &gm200_flcn_imem_pio,
    .dmem_pio = &gm200_flcn_dmem_pio,
    };
    static const struct nvkm_nvdec_func
    gm107_nvdec = {
    .flcn = &gm107_nvdec_flcn,
    };
    static int
    gm107_nvdec_nofw(struct nvkm_nvdec *nvdec, int ver,
    const struct nvkm_nvdec_fwif *fwif)
    {
    return 0;
    }
    const struct nvkm_nvdec_fwif
    gm107_nvdec_fwif[] = {
    { -1, gm107_nvdec_nofw, &gm107_nvdec },
    {}
    };
    int
    gm107_nvdec_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_nvdec **pnvdec)
    {
    return nvkm_nvdec_new_(gm107_nvdec_fwif, device, type, inst, 0, pnvdec);
    }
