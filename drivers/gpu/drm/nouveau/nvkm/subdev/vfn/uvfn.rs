//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/vfn/uvfn.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_uvfn {
    pub object: nvkm_object,
    pub vfn: *mut nvkm_vfn,
}

    static int
    nvkm_uvfn_map(struct nvkm_object *object, void *argv, u32 argc,
    enum nvkm_object_map *type, u64 *addr, u64 *size)
    {
    struct nvkm_vfn *vfn = nvkm_uvfn(object).vfn;
    struct nvkm_device *device = vfn.subdev.device;
// addr = device->func->resource_addr(device, NVKM_BAR0_PRI) + vfn->addr.user;
// size = vfn->func->user.size;
// type = NVKM_OBJECT_MAP_IO;
    return 0;
    }
    static const struct nvkm_object_func
    nvkm_uvfn = {
    .map = nvkm_uvfn_map,
    };
    int
    nvkm_uvfn_new(struct nvkm_device *device, const struct nvkm_oclass *oclass,
    void *argv, u32 argc, struct nvkm_object **pobject)
    {
    struct nvkm_uvfn *uvfn;
    if (argc != 0)
    return -ENOSYS;
    if (!(uvfn = kzalloc_obj(*uvfn)))
    return -ENOMEM;
    nvkm_object_ctor(&nvkm_uvfn, oclass, &uvfn.object);
    uvfn.vfn = device.vfn;
// pobject = &uvfn->object;
    return 0;
    }
