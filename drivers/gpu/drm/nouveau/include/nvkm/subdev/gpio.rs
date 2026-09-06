//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/gpio.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gpio_ntfy_req {
    pub mask: u8,
    pub line: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gpio_ntfy_rep {
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gpio {
    pub func: *const nvkm_gpio_func,
    pub subdev: nvkm_subdev,

    pub event: nvkm_event,
}

extern "C" {
    pub fn nvkm_gpio_reset(: *mut nvkm_gpio, func: u8);
}
extern "C" {
    pub fn nvkm_gpio_set(: *mut nvkm_gpio, idx: c_int, tag: u8, line: u8, state: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_gpio_get(: *mut nvkm_gpio, idx: c_int, tag: u8, line: u8) -> c_int;
}
extern "C" {
    pub fn nv10_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
extern "C" {
    pub fn nv50_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
extern "C" {
    pub fn g94_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
extern "C" {
    pub fn gf119_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
extern "C" {
    pub fn gk104_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
extern "C" {
    pub fn ga102_gpio_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gpio) -> c_int;
}
