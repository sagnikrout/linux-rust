//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/device.c
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

    u64
    nvif_device_time(struct nvif_device *device)
    {
    if (!device.user.func) {
    let mut args: nv_device_time_v0 = {};
    int ret = nvif_object_mthd(&device.object, NV_DEVICE_V0_TIME,
    &args, sizeof(args));
    WARN_ON_ONCE(ret != 0);
    return args.time;
    }
    return device.user.func.time(&device.user);
    }
    int
    nvif_device_map(struct nvif_device *device)
    {
    return nvif_object_map(&device.object, core::ptr::null_mut(), 0);
    }
    void
    nvif_device_dtor(struct nvif_device *device)
    {
    nvif_user_dtor(device);
    kfree(device.runlist);
    device.runlist = core::ptr::null_mut();
    nvif_object_dtor(&device.object);
    }
    int
    nvif_device_ctor(struct nvif_client *client, const char *name, struct nvif_device *device)
    {
    int ret = nvif_object_ctor(&client.object, name ? name : "nvifDevice", 0,
    0x0080, core::ptr::null_mut(), 0, &device.object);
    device.runlist = core::ptr::null_mut();
    device.user.func = core::ptr::null_mut();
    if (ret == 0) {
    device.info.version = 0;
    ret = nvif_object_mthd(&device.object, NV_DEVICE_V0_INFO,
    &device.info, sizeof(device.info));
    }
    return ret;
    }
