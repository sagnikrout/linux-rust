//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/image.c
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
// Copyright 2014 Red Hat Inc.
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//

    static bool
    nvbios_imagen(struct nvkm_bios *bios, struct nvbios_image *image)
    {
    struct nvkm_subdev *subdev = &bios.subdev;
    struct nvbios_pcirT pcir;
    struct nvbios_npdeT npde;
    u8  ver;
    u16 hdr;
    u32 data;
    switch ((data = nvbios_rd16(bios, image.base + 0x00))) {
    case 0xaa55:
    case 0xbb77:
    case 0x4e56: /* NV */
    break;
    default:
    nvkm_debug(subdev, "%08x: ROM signature (%04x) unknown\n",
    image.base, data);
    return false;
    }
    if (!(data = nvbios_pcirTp(bios, image.base, &ver, &hdr, &pcir)))
    return false;
    image.size = pcir.image_size;
    image.type = pcir.image_type;
    image.last = pcir.last;
    if (image.type != 0x70) {
    if (!(data = nvbios_npdeTp(bios, image.base, &npde)))
    return true;
    image.size = npde.image_size;
    image.last = npde.last;
    } else {
    image.last = true;
    }
    return true;
    }
    bool
    nvbios_image(struct nvkm_bios *bios, int idx, struct nvbios_image *image)
    {
    let mut imaged_addr: u32 = bios.imaged_addr;
    memset(image, 0x00, sizeof(*image));
    bios.imaged_addr = 0;
    do {
    image.base += image.size;
    if (image.last || !nvbios_imagen(bios, image)) {
    bios.imaged_addr = imaged_addr;
    return false;
    }
    } while(idx--);
    bios.imaged_addr = imaged_addr;
    return true;
    }
