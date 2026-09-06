//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/therm/fanpwm.c
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
// Martin Peres
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fanpwm {
    pub base: nvkm_fan,
    pub func: dcb_gpio_func,
}

    static int
    nvkm_fanpwm_get(struct nvkm_therm *therm)
    {
    struct nvkm_fanpwm *fan = (void *)therm.fan;
    struct nvkm_device *device = therm.subdev.device;
    struct nvkm_gpio *gpio = device.gpio;
    let mut card_type: c_int = device.card_type;
    u32 divs, duty;
    int ret;
    ret = therm.func.pwm_get(therm, fan.func.line, &divs, &duty);
    if (ret == 0 && divs) {
    divs = max(divs, duty);
    if (card_type <= NV_40 || (fan.func.log[0] & 1))
    duty = divs - duty;
    return (duty * 100) / divs;
    }
    return nvkm_gpio_get(gpio, 0, fan.func.func, fan.func.line) * 100;
    }
    static int
    nvkm_fanpwm_set(struct nvkm_therm *therm, int percent)
    {
    struct nvkm_fanpwm *fan = (void *)therm.fan;
    let mut card_type: c_int = therm.subdev.device.card_type;
    u32 divs, duty;
    int ret;
    divs = fan.base.perf.pwm_divisor;
    if (fan.base.bios.pwm_freq) {
    divs = 1;
    if (therm.func.pwm_clock)
    divs = therm.func.pwm_clock(therm, fan.func.line);
    divs /= fan.base.bios.pwm_freq;
    }
    duty = ((divs * percent) + 99) / 100;
    if (card_type <= NV_40 || (fan.func.log[0] & 1))
    duty = divs - duty;
    ret = therm.func.pwm_set(therm, fan.func.line, divs, duty);
    if (ret == 0)
    ret = therm.func.pwm_ctrl(therm, fan.func.line, true);
    return ret;
    }
    int
    nvkm_fanpwm_create(struct nvkm_therm *therm, struct dcb_gpio_func *func)
    {
    struct nvkm_device *device = therm.subdev.device;
    struct nvkm_bios *bios = device.bios;
    struct nvkm_fanpwm *fan;
    let mut info: nvbios_therm_fan = {};
    u32 divs, duty;
    nvbios_fan_parse(bios, &info);
    if (!nvkm_boolopt(device.cfgopt, "NvFanPWM", func.param) ||
    !therm.func.pwm_ctrl || info.type == NVBIOS_THERM_FAN_TOGGLE ||
    therm.func.pwm_get(therm, func.line, &divs, &duty) == -ENODEV)
    return -ENODEV;
    fan = kzalloc_obj(*fan);
    if (!fan)
    return -ENOMEM;
    therm.fan = &fan.base;
    fan.base.type = "PWM";
    fan.base.get = nvkm_fanpwm_get;
    fan.base.set = nvkm_fanpwm_set;
    fan.func = *func;
    return 0;
    }
