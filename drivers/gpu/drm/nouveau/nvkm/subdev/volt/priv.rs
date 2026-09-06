//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/volt/priv.h
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
pub struct nvkm_volt_func {
    pub ): *mut *mut int (oneinit)(struct nvkm_volt,
    pub ): *mut *mut int (volt_get)(struct nvkm_volt,
    pub uv): *mut *mut *mut int (volt_set)(struct nvkm_volt , u32,
    pub ): *mut *mut int (vid_get)(struct nvkm_volt,
    pub vid): *mut *mut *mut int (vid_set)(struct nvkm_volt , u8,
    pub condition): *mut *mut *mut int (set_id)(struct nvkm_volt , u8 id, int,
    pub ): *mut *mut int (speedo_read)(struct nvkm_volt,
}

extern "C" {
    pub fn nvkm_voltgpio_init(: *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn nvkm_voltgpio_get(: *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn nvkm_voltgpio_set(: *mut nvkm_volt, _arg: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_voltpwm_init(volt: *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn nvkm_voltpwm_get(volt: *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn nvkm_voltpwm_set(volt: *mut nvkm_volt, uv: u32) -> c_int;
}
extern "C" {
    pub fn gf100_volt_oneinit(: *mut nvkm_volt) -> c_int;
}
