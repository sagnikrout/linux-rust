//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/therm/gp100.c
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
// Copyright 2017 Rhys Kidd
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
// Authors: Rhys Kidd
//

    static int
    gp100_temp_get(struct nvkm_therm *therm)
    {
    struct nvkm_device *device = therm.subdev.device;
    struct nvkm_subdev *subdev = &therm.subdev;
    let mut tsensor: u32 = nvkm_rd32(device, 0x020460);
    let mut inttemp: u32 = (tsensor & 0x0001fff8);
// device SHADOWed
    if (tsensor & 0x40000000)
    nvkm_trace(subdev, "reading temperature from SHADOWed sensor\n");
// device valid
    if (tsensor & 0x20000000)
    return (inttemp >> 8);
    else
    return -ENODEV;
    }
    static const struct nvkm_therm_func
    gp100_therm = {
    .temp_get = gp100_temp_get,
    .program_alarms = nvkm_therm_program_alarms_polling,
    };
    int
    gp100_therm_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_therm **ptherm)
    {
    if (nvkm_gsp_rm(device.gsp))
    return -ENODEV;
    return nvkm_therm_new_(&gp100_therm, device, type, inst, ptherm);
    }
