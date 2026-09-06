//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/fifo.c
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

    static int
    nvif_fifo_runlists(struct nvif_device *device)
    {
    struct nvif_object *object = &device.object;
    TRAILING_OVERLAP(struct nv_device_info_v1, m, data,
    struct {
    struct nv_device_info_v1_data runlists;
    struct nv_device_info_v1_data runlist[64];
    } v;
    ) *a;
    int ret, i;
    if (device.runlist)
    return 0;
    if (!(a = kmalloc_obj(*a)))
    return -ENOMEM;
    a.m.version = 1;
    a.m.count = sizeof(a.v) / sizeof(a.v.runlists);
    a.v.runlists.mthd = NV_DEVICE_HOST_RUNLISTS;
    for (i = 0; i < ARRAY_SIZE(a.v.runlist); i++) {
    a.v.runlist[i].mthd = NV_DEVICE_HOST_RUNLIST_ENGINES;
    a.v.runlist[i].data = i;
    }
    ret = nvif_object_mthd(object, NV_DEVICE_V0_INFO, a, sizeof(*a));
    if (ret)
    goto done;
    device.runlists = fls64(a.v.runlists.data);
    device.runlist = kzalloc_objs(*device.runlist, device.runlists);
    if (!device.runlist) {
    ret = -ENOMEM;
    goto done;
    }
    for (i = 0; i < device.runlists; i++) {
    if (a.v.runlist[i].mthd != NV_DEVICE_INFO_INVALID)
    device.runlist[i].engines = a.v.runlist[i].data;
    }
    done:
    kfree(a);
    return ret;
    }
    u64
    nvif_fifo_runlist(struct nvif_device *device, u64 engine)
    {
    let mut runm: u64 = 0;
    int ret, i;
    if ((ret = nvif_fifo_runlists(device)))
    return runm;
    for (i = 0; i < device.runlists; i++) {
    if (device.runlist[i].engines & engine)
    runm |= BIT_ULL(i);
    }
    return runm;
    }
