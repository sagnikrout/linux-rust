//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/i2c/auxch.h
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
pub struct nvkm_i2c_aux_func {
    pub address_only: bool,
    pub size): *mut *mut u32 addr, u8 data, u8,
    pub enhanced_framing): bool,
}

extern "C" {
    pub fn nvkm_i2c_aux_del(: *mut nvkm_i2c_aux);
}
extern "C" {
    pub fn nvkm_i2c_aux_init(: *mut nvkm_i2c_aux);
}
extern "C" {
    pub fn nvkm_i2c_aux_fini(: *mut nvkm_i2c_aux);
}
extern "C" {
    pub fn g94_i2c_aux_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_aux) -> c_int;
}
extern "C" {
    pub fn g94_i2c_aux_xfer(: *mut nvkm_i2c_aux, _arg: bool, _arg: u8, _arg: u32, : *mut u8, : *mut u8) -> c_int;
}
extern "C" {
    pub fn gf119_i2c_aux_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_aux) -> c_int;
}
extern "C" {
    pub fn gm200_i2c_aux_new(: *mut nvkm_i2c_pad, _arg: c_int, _arg: u8, : *mut nvkm_i2c_aux) -> c_int;
}

