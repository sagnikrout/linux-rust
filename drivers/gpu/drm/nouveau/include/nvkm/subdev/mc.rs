//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/mc.h
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
pub struct nvkm_mc {
    pub func: *const nvkm_mc_func,
    pub subdev: nvkm_subdev,
    pub intr: nvkm_intr,
}

extern "C" {
    pub fn nvkm_mc_enable(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int);
}
extern "C" {
    pub fn nvkm_mc_disable(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int);
}
extern "C" {
    pub fn nvkm_mc_enabled(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int) -> bool;
}
extern "C" {
    pub fn nvkm_mc_reset(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int);
}
extern "C" {
    pub fn nvkm_mc_intr_mask(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, enable: bool);
}
extern "C" {
    pub fn nvkm_mc_unk260(: *mut nvkm_device, data: u32);
}
extern "C" {
    pub fn nv04_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn nv11_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn nv17_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn nv44_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn nv50_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn g84_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn g98_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gt215_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gf100_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gk104_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gk20a_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gp100_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn gp10b_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
extern "C" {
    pub fn ga100_mc_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_mc) -> c_int;
}
