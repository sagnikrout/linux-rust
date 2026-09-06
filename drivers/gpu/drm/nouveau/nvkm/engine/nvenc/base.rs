//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/nvenc/base.c
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
// Copyright 2019 Red Hat Inc.
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

    static void *
    nvkm_nvenc_dtor(struct nvkm_engine *engine)
    {
    struct nvkm_nvenc *nvenc = nvkm_nvenc(engine);
    nvkm_falcon_dtor(&nvenc.falcon);
    return nvenc;
    }
    static const struct nvkm_engine_func
    nvkm_nvenc = {
    .dtor = nvkm_nvenc_dtor,
    .sclass = { {} },
    };
    int
    nvkm_nvenc_new_(const struct nvkm_nvenc_fwif *fwif, struct nvkm_device *device,
    enum nvkm_subdev_type type, int inst, struct nvkm_nvenc **pnvenc)
    {
    struct nvkm_nvenc *nvenc;
    int ret;
    if (!(nvenc = *pnvenc = kzalloc_obj(*nvenc)))
    return -ENOMEM;
    ret = nvkm_engine_ctor(&nvkm_nvenc, device, type, inst, true,
    &nvenc.engine);
    if (ret)
    return ret;
    fwif = nvkm_firmware_load(&nvenc.engine.subdev, fwif, "Nvenc", nvenc);
    if (IS_ERR(fwif))
    return -ENODEV;
    nvenc.func = fwif.func;
    return nvkm_falcon_ctor(nvenc.func.flcn, &nvenc.engine.subdev,
    nvenc.engine.subdev.name, 0, &nvenc.falcon);
    }
