//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/gp102.c
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
// Copyright 2016 Red Hat Inc.
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

    int
    gp102_fb_vpr_scrub(struct nvkm_fb *fb)
    {
    return nvkm_falcon_fw_boot(&fb.vpr_scrubber, &fb.subdev, true, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0x00000000);
    }
    bool
    gp102_fb_vpr_scrub_required(struct nvkm_fb *fb)
    {
    struct nvkm_device *device = fb.subdev.device;
    nvkm_wr32(device, 0x100cd0, 0x2);
    return (nvkm_rd32(device, 0x100cd0) & 0x00000010) != 0;
    }
    u64
    gp102_fb_vidmem_size(struct nvkm_fb *fb)
    {
    let mut data: u32 = nvkm_rd32(fb.subdev.device, 0x100ce0);
    let mut lmag: u32 = (data & 0x000003f0) >> 4;
    let mut lsca: u32 = (data & 0x0000000f);
    let mut size: u64 = (u64)lmag << (lsca + 20);
    if (data & 0x40000000)
    return size / 16 * 15;
    return size;
    }
    int
    gp102_fb_oneinit(struct nvkm_fb *fb)
    {
    struct nvkm_subdev *subdev = &fb.subdev;
    nvkm_falcon_fw_ctor_hs(&gm200_flcn_fw, "mem-unlock", subdev, core::ptr::null_mut(), "nvdec/scrubber",
    0, &subdev.device.nvdec[0].falcon, &fb.vpr_scrubber);
    return gf100_fb_oneinit(fb);
    }
    static const struct nvkm_fb_func
    gp102_fb = {
    .dtor = gf100_fb_dtor,
    .oneinit = gp102_fb_oneinit,
    .init = gm200_fb_init,
    .init_remapper = gp100_fb_init_remapper,
    .init_page = gm200_fb_init_page,
    .sysmem.flush_page_init = gf100_fb_sysmem_flush_page_init,
    .vidmem.size = gp102_fb_vidmem_size,
    .vpr.scrub_required = gp102_fb_vpr_scrub_required,
    .vpr.scrub = gp102_fb_vpr_scrub,
    .ram_new = gp102_ram_new,
    };
    int
    gp102_fb_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst, struct nvkm_fb **pfb)
    {
    return gf100_fb_new_(&gp102_fb, device, type, inst, pfb);
    }
    MODULE_FIRMWARE("nvidia/gp102/nvdec/scrubber.bin");
    MODULE_FIRMWARE("nvidia/gp104/nvdec/scrubber.bin");
    MODULE_FIRMWARE("nvidia/gp106/nvdec/scrubber.bin");
    MODULE_FIRMWARE("nvidia/gp107/nvdec/scrubber.bin");
    MODULE_FIRMWARE("nvidia/gp108/nvdec/scrubber.bin");
