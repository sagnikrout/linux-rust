//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/bus.h
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
pub struct nvkm_i2c_bus_func {
    pub ): *mut *mut void (init)(struct nvkm_i2c_bus,
    pub state): *mut *mut *mut void (drive_scl)(struct nvkm_i2c_bus , int,
    pub state): *mut *mut *mut void (drive_sda)(struct nvkm_i2c_bus , int,
    pub ): *mut *mut int (sense_scl)(struct nvkm_i2c_bus,
    pub ): *mut *mut int (sense_sda)(struct nvkm_i2c_bus,
    pub num): *mut *mut *mut *mut int (xfer)(struct nvkm_i2c_bus , struct i2c_msg , int,
}

extern "C" {
    pub fn nvkm_i2c_bus_del(: *mut nvkm_i2c_bus);
}
extern "C" {
    pub fn nvkm_i2c_bus_init(: *mut nvkm_i2c_bus);
}
extern "C" {
    pub fn nvkm_i2c_bus_fini(: *mut nvkm_i2c_bus);
}
extern "C" {
    pub fn nvkm_i2c_bit_xfer(: *mut nvkm_i2c_bus, : *mut i2c_msg, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv4e_i2c_bus_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_bus) -> c_int;
}
extern "C" {
    pub fn nv50_i2c_bus_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_bus) -> c_int;
}
extern "C" {
    pub fn gf119_i2c_bus_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_bus) -> c_int;
}

