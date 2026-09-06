//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/disp.c
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
// Copyright 2018 Red Hat Inc.
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

    void
    nvif_disp_dtor(struct nvif_disp *disp)
    {
    nvif_object_dtor(&disp.object);
    }
    int
    nvif_disp_ctor(struct nvif_device *device, const char *name, s32 oclass, struct nvif_disp *disp)
    {
    static const struct nvif_mclass disps[] = {
    { GB202_DISP, 0 },
    { AD102_DISP, 0 },
    { GA102_DISP, 0 },
    { TU102_DISP, 0 },
    { GV100_DISP, 0 },
    { GP102_DISP, 0 },
    { GP100_DISP, 0 },
    { GM200_DISP, 0 },
    { GM107_DISP, 0 },
    { GK110_DISP, 0 },
    { GK104_DISP, 0 },
    { GF110_DISP, 0 },
    { GT214_DISP, 0 },
    { GT206_DISP, 0 },
    { GT200_DISP, 0 },
    {   G82_DISP, 0 },
    {  NV50_DISP, 0 },
    {  NV04_DISP, 0 },
    {}
    };
    struct nvif_disp_v0 args;
    int cid, ret;
    cid = nvif_sclass(&device.object, disps, oclass);
    disp.object.client = core::ptr::null_mut();
    if (cid < 0) {
    NVIF_DEBUG(&device.object, "[NEW disp%04x] not supported", oclass);
    return cid;
    }
    args.version = 0;
    ret = nvif_object_ctor(&device.object, name ?: "nvifDisp", 0,
    disps[cid].oclass, &args, sizeof(args), &disp.object);
    NVIF_ERRON(ret, &device.object, "[NEW disp%04x]", disps[cid].oclass);
    if (ret)
    return ret;
    NVIF_DEBUG(&disp.object, "[NEW] conn_mask:%08x outp_mask:%08x head_mask:%08x",
    args.conn_mask, args.outp_mask, args.head_mask);
    disp.conn_mask = args.conn_mask;
    disp.outp_mask = args.outp_mask;
    disp.head_mask = args.head_mask;
    return 0;
    }
