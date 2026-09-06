//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/sec/g98.c
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
// Authors: Ben Skeggs
//

    static const struct nvkm_enum g98_sec_isr_error_name[] = {
    { 0x0000, "ILLEGAL_MTHD" },
    { 0x0001, "INVALID_BITFIELD" },
    { 0x0002, "INVALID_ENUM" },
    { 0x0003, "QUERY" },
    {}
    };
    static void
    g98_sec_intr(struct nvkm_falcon *sec, struct nvkm_chan *chan)
    {
    struct nvkm_subdev *subdev = &sec.engine.subdev;
    struct nvkm_device *device = subdev.device;
    let mut ssta: u32 = nvkm_rd32(device, 0x087040) & 0x0000ffff;
    let mut addr: u32 = nvkm_rd32(device, 0x087040) >> 16;
    let mut mthd: u32 = (addr & 0x07ff) << 2;
    let mut subc: u32 = (addr & 0x3800) >> 11;
    let mut data: u32 = nvkm_rd32(device, 0x087044);
    const struct nvkm_enum *en =
    nvkm_enum_find(g98_sec_isr_error_name, ssta);
    nvkm_error(subdev, "DISPATCH_ERROR %04x [%s] ch %d [%010llx %s] "
    "subc %d mthd %04x data %08x\n", ssta,
    en ? en.name : "UNKNOWN", chan ? chan.id : -1,
    chan ? chan.inst.addr : 0,
    chan ? chan.name : "unknown",
    subc, mthd, data);
    }
    static const struct nvkm_falcon_func
    g98_sec = {
    .code.data = g98_sec_code,
    .code.size = sizeof(g98_sec_code),
    .data.data = g98_sec_data,
    .data.size = sizeof(g98_sec_data),
    .intr = g98_sec_intr,
    .sclass = {
    { -1, -1, G98_SEC },
    {}
    }
    };
    int
    g98_sec_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_engine **pengine)
    {
    return nvkm_falcon_new_(&g98_sec, device, type, inst, true, 0x087000, pengine);
    }
