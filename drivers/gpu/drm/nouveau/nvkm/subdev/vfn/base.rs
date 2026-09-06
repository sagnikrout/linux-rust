//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/vfn/base.c
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

    static void *
    nvkm_vfn_dtor(struct nvkm_subdev *subdev)
    {
    return nvkm_vfn(subdev);
    }
    static const struct nvkm_subdev_func
    nvkm_vfn = {
    .dtor = nvkm_vfn_dtor,
    };
    int
    nvkm_vfn_new_(const struct nvkm_vfn_func *func, struct nvkm_device *device,
    enum nvkm_subdev_type type, int inst, u32 addr, struct nvkm_vfn **pvfn)
    {
    struct nvkm_vfn *vfn;
    int ret;
    if (!(vfn = *pvfn = kzalloc_obj(*vfn)))
    return -ENOMEM;
    nvkm_subdev_ctor(&nvkm_vfn, device, type, inst, &vfn.subdev);
    vfn.func = func;
    vfn.addr.priv = addr;
    vfn.addr.user = vfn.addr.priv + func.user.addr;
    if (vfn.func.intr) {
    ret = nvkm_intr_add(vfn.func.intr, vfn.func.intrs,
    &vfn.subdev, 8, &vfn.intr);
    if (ret)
    return ret;
    }
    vfn.user.ctor = nvkm_uvfn_new;
    vfn.user.base = func.user.base;
    return 0;
    }
