//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/volt.h
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
pub struct nvkm_volt {
    pub func: *const nvkm_volt_func,
    pub subdev: nvkm_subdev,
    pub vid_mask: u8,
    pub vid_nr: u8,
    pub uv: u32,
    pub vid: u8,
    pub vid: [}; 256],
    pub max_uv: u32,
    pub min_uv: u32,
//
// These are fully functional map entries creating a sw ceiling for
// the voltage. These all can describe different kind of curves, so
// that for any given temperature a different one can return the lowest
// value of all three.
//
    pub max0_id: u8,
    pub max1_id: u8,
    pub max2_id: u8,
    pub speedo: c_int,
}

extern "C" {
    pub fn nvkm_volt_map(volt: *mut nvkm_volt, id: u8, temperature: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_volt_map_min(volt: *mut nvkm_volt, id: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_volt_get(: *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn nv40_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn gf100_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn gf117_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn gk104_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn gk20a_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
extern "C" {
    pub fn gm20b_volt_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_volt) -> c_int;
}
