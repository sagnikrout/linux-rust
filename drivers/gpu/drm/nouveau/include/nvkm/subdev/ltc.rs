//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/ltc.h
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

pub const NVKM_LTC_MAX_ZBC_COLOR_CNT: c_int = 32;
pub const NVKM_LTC_MAX_ZBC_DEPTH_CNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ltc {
    pub func: *const nvkm_ltc_func,
    pub subdev: nvkm_subdev,
    pub ltc_nr: u32,
    pub lts_nr: u32,
    pub /: *mut *mut mutex mutex; / serialises CBC operations,
    pub num_tags: u32,
    pub tag_base: u32,
    pub tag_ram: *mut nvkm_memory,
    pub zbc_color_min: c_int,
    pub zbc_color_max: c_int,
    pub zbc_color: [u32; NVKM_LTC_MAX_ZBC_COLOR_CNT][4],
    pub zbc_depth_min: c_int,
    pub zbc_depth_max: c_int,
    pub zbc_depth: [u32; NVKM_LTC_MAX_ZBC_DEPTH_CNT],
    pub zbc_stencil: [u32; NVKM_LTC_MAX_ZBC_DEPTH_CNT],
}

extern "C" {
    pub fn nvkm_ltc_tags_clear(: *mut nvkm_device, first: u32, count: u32);
}
extern "C" {
    pub fn nvkm_ltc_zbc_color_get(: *mut nvkm_ltc, index: c_int, u32[4]: const) -> c_int;
}
extern "C" {
    pub fn nvkm_ltc_zbc_depth_get(: *mut nvkm_ltc, index: c_int, u32: const) -> c_int;
}
extern "C" {
    pub fn nvkm_ltc_zbc_stencil_get(: *mut nvkm_ltc, index: c_int, u32: const) -> c_int;
}
extern "C" {
    pub fn nvkm_ltc_invalidate(: *mut nvkm_ltc);
}
extern "C" {
    pub fn nvkm_ltc_flush(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gf100_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gk104_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gm107_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gm200_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gp100_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gp102_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gp10b_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn ga102_ltc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_ltc) -> c_int;
}
