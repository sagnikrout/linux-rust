//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/shadowramin.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv {
    pub bios: *mut nvkm_bios,
    pub bar0: u32,
}

    static u32
    pramin_read(void *data, u32 offset, u32 length, struct nvkm_bios *bios)
    {
    struct nvkm_device *device = bios.subdev.device;
    u32 i;
    if (offset + length <= 0x00100000) {
    for (i = offset; i < offset + length; i += 4)
// (u32 *)&bios->data[i] = nvkm_rd32(device, 0x700000 + i);
    return length;
    }
    return 0;
    }
    static void
    pramin_fini(void *data)
    {
    struct priv *priv = data;
    if (priv) {
    struct nvkm_device *device = priv.bios.subdev.device;
    nvkm_wr32(device, 0x001700, priv.bar0);
    kfree(priv);
    }
    }
    static void *
    pramin_init(struct nvkm_bios *bios, const char *name)
    {
    struct nvkm_subdev *subdev = &bios.subdev;
    struct nvkm_device *device = subdev.device;
    struct priv *priv = core::ptr::null_mut();
    let mut addr: u64 = 0;
// PRAMIN always potentially available prior to nv50
    if (device.card_type < NV_50)
    return core::ptr::null_mut();
// we can't get the bios image pointer without PDISP
    if (device.card_type >= GA100)
    addr = nvkm_rd32(device, 0x820c04);
    else
    if (device.card_type >= GM100)
    addr = nvkm_rd32(device, 0x021c04);
    else
    if (device.card_type >= NV_C0)
    addr = nvkm_rd32(device, 0x022500);
    if (addr & 0x00000001) {
    nvkm_debug(subdev, "... display disabled\n");
    return ERR_PTR(-ENODEV);
    }
// check that the window is enabled and in vram, particularly
// important as we don't want to be touching vram on an
// uninitialised board
//
    if (device.card_type >= GV100)
    addr = nvkm_rd32(device, 0x625f04);
    else
    addr = nvkm_rd32(device, 0x619f04);
    if (!(addr & 0x00000008)) {
    nvkm_debug(subdev, "... not enabled\n");
    return ERR_PTR(-ENODEV);
    }
    if ( (addr & 0x00000003) != 1) {
    nvkm_debug(subdev, "... not in vram\n");
    return ERR_PTR(-ENODEV);
    }
// some alternate method inherited from xf86-video-nv...
    addr = (addr & 0xffffff00) << 8;
    if (!addr) {
    addr  = (u64)nvkm_rd32(device, 0x001700) << 16;
    addr += 0xf0000;
    }
// modify bar0 PRAMIN window to cover the bios image
    if (!(priv = kmalloc_obj(*priv))) {
    nvkm_error(subdev, "... out of memory\n");
    return ERR_PTR(-ENOMEM);
    }
    priv.bios = bios;
    priv.bar0 = nvkm_rd32(device, 0x001700);
    nvkm_wr32(device, 0x001700, addr >> 16);
    return priv;
    }
    const struct nvbios_source
    nvbios_ramin = {
    .name = "PRAMIN",
    .init = pramin_init,
    .fini = pramin_fini,
    .read = pramin_read,
    .rw = true,
    };
