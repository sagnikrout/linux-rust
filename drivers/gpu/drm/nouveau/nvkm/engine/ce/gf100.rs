//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/ce/gf100.c
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

    static void
    gf100_ce_init(struct nvkm_falcon *ce)
    {
    nvkm_wr32(ce.engine.subdev.device, ce.addr + 0x084, ce.engine.subdev.inst);
    }
    static const struct nvkm_falcon_func
    gf100_ce0 = {
    .code.data = gf100_ce_code,
    .code.size = sizeof(gf100_ce_code),
    .data.data = gf100_ce_data,
    .data.size = sizeof(gf100_ce_data),
    .init = gf100_ce_init,
    .intr = gt215_ce_intr,
    .sclass = {
    { -1, -1, FERMI_DMA },
    {}
    }
    };
    static const struct nvkm_falcon_func
    gf100_ce1 = {
    .code.data = gf100_ce_code,
    .code.size = sizeof(gf100_ce_code),
    .data.data = gf100_ce_data,
    .data.size = sizeof(gf100_ce_data),
    .init = gf100_ce_init,
    .intr = gt215_ce_intr,
    .sclass = {
    { -1, -1, FERMI_DECOMPRESS },
    {}
    }
    };
    int
    gf100_ce_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_engine **pengine)
    {
    return nvkm_falcon_new_(inst ? &gf100_ce1 : &gf100_ce0, device, type, inst, true,
    0x104000 + (inst * 0x1000), pengine);
    }
