//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fuse/nv50.c
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
// Copyright 2014 Martin Peres
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
// Authors: Martin Peres
//

    static u32
    nv50_fuse_read(struct nvkm_fuse *fuse, u32 addr)
    {
    struct nvkm_device *device = fuse.subdev.device;
    unsigned long flags;
    u32 fuse_enable, val;
// racy if another part of nvkm start writing to this reg
    spin_lock_irqsave(&fuse.lock, flags);
    fuse_enable = nvkm_mask(device, 0x001084, 0x800, 0x800);
    val = nvkm_rd32(device, 0x021000 + addr);
    nvkm_wr32(device, 0x001084, fuse_enable);
    spin_unlock_irqrestore(&fuse.lock, flags);
    return val;
    }
    static const struct nvkm_fuse_func
    nv50_fuse = {
    .read = &nv50_fuse_read,
    };
    int
    nv50_fuse_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_fuse **pfuse)
    {
    return nvkm_fuse_new_(&nv50_fuse, device, type, inst, pfuse);
    }
