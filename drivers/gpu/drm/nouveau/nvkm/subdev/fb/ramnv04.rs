//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/ramnv04.c
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
// Copyright 2013 Red Hat Inc.
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

    const struct nvkm_ram_func
    nv04_ram_func = {
    };
    int
    nv04_ram_new(struct nvkm_fb *fb, struct nvkm_ram **pram)
    {
    struct nvkm_device *device = fb.subdev.device;
    let mut boot0: u32 = nvkm_rd32(device, NV04_PFB_BOOT_0);
    u64 size;
    enum nvkm_ram_type type;
    if (boot0 & 0x00000100) {
    size  = ((boot0 >> 12) & 0xf) * 2 + 2;
    size *= 1024 * 1024;
    } else {
    switch (boot0 & NV04_PFB_BOOT_0_RAM_AMOUNT) {
    case NV04_PFB_BOOT_0_RAM_AMOUNT_32MB:
    size = 32 * 1024 * 1024;
    break;
    case NV04_PFB_BOOT_0_RAM_AMOUNT_16MB:
    size = 16 * 1024 * 1024;
    break;
    case NV04_PFB_BOOT_0_RAM_AMOUNT_8MB:
    size = 8 * 1024 * 1024;
    break;
    case NV04_PFB_BOOT_0_RAM_AMOUNT_4MB:
    size = 4 * 1024 * 1024;
    break;
    }
    }
    if ((boot0 & 0x00000038) <= 0x10)
    type = NVKM_RAM_TYPE_SGRAM;
    else
    type = NVKM_RAM_TYPE_SDRAM;
    return nvkm_ram_new_(&nv04_ram_func, fb, type, size, pram);
    }
