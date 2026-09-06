//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/therm/priv.h
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
// Copyright 2012 The Nouveau community
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fan {
    pub parent: *mut nvkm_therm,
    pub type: *const c_char,
    pub bios: nvbios_therm_fan,
    pub perf: nvbios_perf_fan,
    pub alarm: nvkm_alarm,
    pub lock: spinlock_t,
    pub percent: c_int,
    pub ): *mut *mut int (get)(struct nvkm_therm,
    pub percent): *mut *mut *mut int (set)(struct nvkm_therm , int,
    pub tach: dcb_gpio_func,
}

extern "C" {
    pub fn nvkm_therm_fan_mode(: *mut nvkm_therm, mode: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_attr_get(: *mut nvkm_therm, nvkm_therm_attr_type: enum) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_attr_set(: *mut nvkm_therm, nvkm_therm_attr_type: enum, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_ic_ctor(: *mut nvkm_therm);
}
extern "C" {
    pub fn nvkm_therm_sensor_ctor(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_ctor(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_init(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_fini(: *mut nvkm_therm, suspend: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_get(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_set(: *mut nvkm_therm, now: bool, percent: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_user_get(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_fan_user_set(: *mut nvkm_therm, percent: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_sensor_init(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_sensor_fini(: *mut nvkm_therm, suspend: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_therm_sensor_preinit(: *mut nvkm_therm);
}
extern "C" {
    pub fn nvkm_therm_program_alarms_polling(: *mut nvkm_therm);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_therm_func {
    pub ): *mut *mut void (init)(struct nvkm_therm,
    pub ): *mut *mut void (fini)(struct nvkm_therm,
    pub ): *mut *mut void (intr)(struct nvkm_therm,
    pub bool): *mut *mut *mut int (pwm_ctrl)(struct nvkm_therm , int line,,
    pub ): *mut *mut *mut *mut int (pwm_get)(struct nvkm_therm , int line, u32 , u32,
    pub u32): *mut *mut *mut int (pwm_set)(struct nvkm_therm , int line, u32,,
    pub line): *mut *mut *mut int (pwm_clock)(struct nvkm_therm , int,
    pub ): *mut *mut int (temp_get)(struct nvkm_therm,
    pub ): *mut *mut int (fan_sense)(struct nvkm_therm,
    pub ): *mut *mut void (program_alarms)(struct nvkm_therm,
    pub ): *const nvkm_therm_clkgate_pack,
    pub ): *mut *mut void (clkgate_enable)(struct nvkm_therm,
    pub bool): *mut *mut *mut void (clkgate_fini)(struct nvkm_therm ,,
}

extern "C" {
    pub fn nv40_therm_intr(: *mut nvkm_therm);
}
extern "C" {
    pub fn nv50_fan_pwm_ctrl(: *mut nvkm_therm, _arg: c_int, _arg: bool) -> c_int;
}
extern "C" {
    pub fn nv50_fan_pwm_get(: *mut nvkm_therm, _arg: c_int, : *mut u32, : *mut u32) -> c_int;
}
extern "C" {
    pub fn nv50_fan_pwm_set(: *mut nvkm_therm, _arg: c_int, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nv50_fan_pwm_clock(: *mut nvkm_therm, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn g84_temp_get(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn g84_sensor_setup(: *mut nvkm_therm);
}
extern "C" {
    pub fn g84_therm_fini(: *mut nvkm_therm);
}
extern "C" {
    pub fn gt215_therm_fan_sense(: *mut nvkm_therm) -> c_int;
}
extern "C" {
    pub fn g84_therm_init(: *mut nvkm_therm);
}
extern "C" {
    pub fn gf119_fan_pwm_ctrl(: *mut nvkm_therm, _arg: c_int, _arg: bool) -> c_int;
}
extern "C" {
    pub fn gf119_fan_pwm_get(: *mut nvkm_therm, _arg: c_int, : *mut u32, : *mut u32) -> c_int;
}
extern "C" {
    pub fn gf119_fan_pwm_set(: *mut nvkm_therm, _arg: c_int, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf119_fan_pwm_clock(: *mut nvkm_therm, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gf119_therm_init(: *mut nvkm_therm);
}
extern "C" {
    pub fn gk104_therm_init(: *mut nvkm_therm);
}
extern "C" {
    pub fn gk104_clkgate_enable(: *mut nvkm_therm);
}
extern "C" {
    pub fn gk104_clkgate_fini(: *mut nvkm_therm, _arg: bool);
}
extern "C" {
    pub fn nvkm_fanpwm_create(: *mut nvkm_therm, : *mut dcb_gpio_func) -> c_int;
}
extern "C" {
    pub fn nvkm_fantog_create(: *mut nvkm_therm, : *mut dcb_gpio_func) -> c_int;
}
extern "C" {
    pub fn nvkm_fannil_create(: *mut nvkm_therm) -> c_int;
}
