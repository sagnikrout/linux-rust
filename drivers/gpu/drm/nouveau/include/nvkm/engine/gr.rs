//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/engine/gr.h
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
pub struct nvkm_gr_zcull_info {
    pub width_align_pixels: __u32,
    pub height_align_pixels: __u32,
    pub pixel_squares_by_aliquots: __u32,
    pub aliquot_total: __u32,
    pub zcull_region_byte_multiplier: __u32,
    pub zcull_region_header_size: __u32,
    pub zcull_subregion_header_size: __u32,
    pub subregion_count: __u32,
    pub subregion_width_align_pixels: __u32,
    pub subregion_height_align_pixels: __u32,
    pub ctxsw_size: __u32,
    pub ctxsw_align: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gr {
    pub func: *const nvkm_gr_func,
    pub engine: nvkm_engine,
    pub zcull_info: nvkm_gr_zcull_info,
    pub has_zcull_info: bool,
}

extern "C" {
    pub fn nvkm_gr_units(: *mut nvkm_gr) -> u64;
}
extern "C" {
    pub fn nvkm_gr_tlb_flush(: *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nvkm_gr_ctxsw_pause(: *mut nvkm_device) -> c_int;
}
extern "C" {
    pub fn nvkm_gr_ctxsw_resume(: *mut nvkm_device) -> c_int;
}
extern "C" {
    pub fn nvkm_gr_ctxsw_inst(: *mut nvkm_device) -> u32;
}
extern "C" {
    pub fn nv04_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv10_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv15_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv17_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv20_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv25_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv2a_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv30_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv34_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv35_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv40_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv44_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn nv50_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn g84_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gt200_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn mcp79_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gt215_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn mcp89_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf100_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf104_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf108_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf110_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf117_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gf119_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gk104_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gk110_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gk110b_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gk208_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gm107_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gm200_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gm20b_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp100_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp102_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp104_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp107_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp108_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gp10b_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn gv100_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn tu102_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn ga102_gr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_gr) -> c_int;
}
