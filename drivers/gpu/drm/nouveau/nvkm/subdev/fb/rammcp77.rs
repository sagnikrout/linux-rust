//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/rammcp77.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp77_ram {
    pub base: nvkm_ram,
    pub poller_base: u64,
}

    static int
    mcp77_ram_init(struct nvkm_ram *base)
    {
    struct mcp77_ram *ram = mcp77_ram(base);
    struct nvkm_device *device = ram.base.fb.subdev.device;
    let mut dniso: u32 = ((ram.base.size - (ram.poller_base + 0x00)) >> 5) - 1;
    let mut hostnb: u32 = ((ram.base.size - (ram.poller_base + 0x20)) >> 5) - 1;
    let mut flush: u32 = ((ram.base.size - (ram.poller_base + 0x40)) >> 5) - 1;
// Enable NISO poller for various clients and set their associated
// read address, only for MCP77/78 and MCP79/7A. (fd#27501)
//
    nvkm_wr32(device, 0x100c18, dniso);
    nvkm_mask(device, 0x100c14, 0x00000000, 0x00000001);
    nvkm_wr32(device, 0x100c1c, hostnb);
    nvkm_mask(device, 0x100c14, 0x00000000, 0x00000002);
    nvkm_wr32(device, 0x100c24, flush);
    nvkm_mask(device, 0x100c14, 0x00000000, 0x00010000);
    return 0;
    }
    static const struct nvkm_ram_func
    mcp77_ram_func = {
    .init = mcp77_ram_init,
    };
    int
    mcp77_ram_new(struct nvkm_fb *fb, struct nvkm_ram **pram)
    {
    struct nvkm_device *device = fb.subdev.device;
    u32 rsvd_head = ( 256 * 1024); /* vga memory */
    u32 rsvd_tail = (1024 * 1024) + 0x1000; /* vbios etc + poller mem */
    let mut base: u64 = (u64)nvkm_rd32(device, 0x100e10) << 12;
    let mut size: u64 = (u64)nvkm_rd32(device, 0x100e14) << 12;
    struct mcp77_ram *ram;
    int ret;
    if (!(ram = kzalloc_obj(*ram)))
    return -ENOMEM;
// pram = &ram->base;
    ret = nvkm_ram_ctor(&mcp77_ram_func, fb, NVKM_RAM_TYPE_STOLEN,
    size, &ram.base);
    if (ret)
    return ret;
    ram.poller_base = size - rsvd_tail;
    ram.base.stolen = base;
    nvkm_mm_fini(&ram.base.vram);
    return nvkm_mm_init(&ram.base.vram, NVKM_RAM_MM_NORMAL,
    rsvd_head >> NVKM_RAM_MM_SHIFT,
    (size - rsvd_head - rsvd_tail) >>
    NVKM_RAM_MM_SHIFT, 1);
    }
