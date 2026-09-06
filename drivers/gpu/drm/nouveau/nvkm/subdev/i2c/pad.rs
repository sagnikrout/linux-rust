//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/pad.h
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
pub struct nvkm_i2c_pad {
    pub func: *const nvkm_i2c_pad_func,
    pub i2c: *mut nvkm_i2c,

    pub id: c_int,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_i2c_pad_mode {
    NVKM_I2C_PAD_OFF,
    NVKM_I2C_PAD_I2C,
    NVKM_I2C_PAD_AUX,
    } mode;
    struct mutex mutex;
    struct list_head head;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_i2c_pad_func {
    pub ): *mut nvkm_i2c_bus,
    pub ): *mut nvkm_i2c_bus,
    pub ): *mut nvkm_i2c_aux,
    pub nvkm_i2c_pad_mode): *mut *mut *mut void (mode)(struct nvkm_i2c_pad , enum,
}

extern "C" {
    pub fn nvkm_i2c_pad_del(: *mut nvkm_i2c_pad);
}
extern "C" {
    pub fn nvkm_i2c_pad_init(: *mut nvkm_i2c_pad);
}
extern "C" {
    pub fn nvkm_i2c_pad_fini(: *mut nvkm_i2c_pad);
}
extern "C" {
    pub fn nvkm_i2c_pad_mode(: *mut nvkm_i2c_pad, nvkm_i2c_pad_mode: enum);
}
extern "C" {
    pub fn nvkm_i2c_pad_acquire(: *mut nvkm_i2c_pad, nvkm_i2c_pad_mode: enum) -> c_int;
}
extern "C" {
    pub fn nvkm_i2c_pad_release(: *mut nvkm_i2c_pad);
}
extern "C" {
    pub fn g94_i2c_pad_mode(: *mut nvkm_i2c_pad, nvkm_i2c_pad_mode: enum);
}
extern "C" {
    pub fn nv04_i2c_pad_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn nv4e_i2c_pad_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn nv50_i2c_pad_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn g94_i2c_pad_x_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn gf119_i2c_pad_x_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn gm200_i2c_pad_x_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn g94_i2c_pad_s_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn gf119_i2c_pad_s_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn gm200_i2c_pad_s_new(: *mut nvkm_i2c, _arg: c_int, : *mut nvkm_i2c_pad) -> c_int;
}
extern "C" {
    pub fn anx9805_pad_new(: *mut nvkm_i2c_bus, _arg: c_int, _arg: u8, : *mut nvkm_i2c_pad) -> c_int;
}

