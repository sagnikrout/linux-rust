//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bios/ramcfg.c
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//

    static u8
    nvbios_ramcfg_strap(struct nvkm_subdev *subdev)
    {
    return (nvkm_rd32(subdev.device, 0x101000) & 0x0000003c) >> 2;
    }
    u8
    nvbios_ramcfg_count(struct nvkm_bios *bios)
    {
    struct bit_entry bit_M;
    if (!bit_entry(bios, 'M', &bit_M)) {
    if (bit_M.version == 1 && bit_M.length >= 5)
    return nvbios_rd08(bios, bit_M.offset + 2);
    if (bit_M.version == 2 && bit_M.length >= 3)
    return nvbios_rd08(bios, bit_M.offset + 0);
    }
    return 0x00;
    }
    u8
    nvbios_ramcfg_index(struct nvkm_subdev *subdev)
    {
    struct nvkm_bios *bios = subdev.device.bios;
    let mut strap: u8 = nvbios_ramcfg_strap(subdev);
    let mut xlat: u32 = 0x00000000;
    struct bit_entry bit_M;
    struct nvbios_M0203E M0203E;
    u8 ver, hdr;
    if (!bit_entry(bios, 'M', &bit_M)) {
    if (bit_M.version == 1 && bit_M.length >= 5)
    xlat = nvbios_rd16(bios, bit_M.offset + 3);
    if (bit_M.version == 2 && bit_M.length >= 3) {
// XXX: is M ever shorter than this?
// if not - what is xlat used for now?
// also - sigh..
//
    if (bit_M.length >= 7 &&
    nvbios_M0203Em(bios, strap, &ver, &hdr, &M0203E))
    return M0203E.group;
    xlat = nvbios_rd16(bios, bit_M.offset + 1);
    }
    }
    if (xlat)
    strap = nvbios_rd08(bios, xlat + strap);
    return strap;
    }
