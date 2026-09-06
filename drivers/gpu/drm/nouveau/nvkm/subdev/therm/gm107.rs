//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/therm/gm107.c
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
// Copyright 2014 Martin Peres
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
// Authors: Martin Peres
//

    static int
    gm107_fan_pwm_ctrl(struct nvkm_therm *therm, int line, bool enable)
    {
// nothing to do, it seems hardwired
    return 0;
    }
    static int
    gm107_fan_pwm_get(struct nvkm_therm *therm, int line, u32 *divs, u32 *duty)
    {
    struct nvkm_device *device = therm.subdev.device;
// divs = nvkm_rd32(device, 0x10eb20) & 0x1fff;
// duty = nvkm_rd32(device, 0x10eb24) & 0x1fff;
    return 0;
    }
    static int
    gm107_fan_pwm_set(struct nvkm_therm *therm, int line, u32 divs, u32 duty)
    {
    struct nvkm_device *device = therm.subdev.device;
    nvkm_mask(device, 0x10eb10, 0x1fff, divs); /* keep the high bits */
    nvkm_wr32(device, 0x10eb14, duty | 0x80000000);
    return 0;
    }
    static int
    gm107_fan_pwm_clock(struct nvkm_therm *therm, int line)
    {
    return therm.subdev.device.crystal * 1000;
    }
    static const struct nvkm_therm_func
    gm107_therm = {
    .init = gf119_therm_init,
    .fini = g84_therm_fini,
    .pwm_ctrl = gm107_fan_pwm_ctrl,
    .pwm_get = gm107_fan_pwm_get,
    .pwm_set = gm107_fan_pwm_set,
    .pwm_clock = gm107_fan_pwm_clock,
    .temp_get = g84_temp_get,
    .fan_sense = gt215_therm_fan_sense,
    .program_alarms = nvkm_therm_program_alarms_polling,
    };
    int
    gm107_therm_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_therm **ptherm)
    {
    return nvkm_therm_new_(&gm107_therm, device, type, inst, ptherm);
    }
