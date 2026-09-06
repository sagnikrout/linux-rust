//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/devinit.h
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
pub struct nvkm_devinit {
    pub func: *const nvkm_devinit_func,
    pub subdev: nvkm_subdev,
    pub post: bool,
    pub force_post: bool,
}

extern "C" {
    pub fn nvkm_devinit_mmio(: *mut nvkm_devinit, addr: u32) -> u32;
}
extern "C" {
    pub fn nvkm_devinit_pll_set(: *mut nvkm_devinit, type: u32, khz: u32) -> c_int;
}
extern "C" {
    pub fn nvkm_devinit_meminit(: *mut nvkm_devinit);
}
extern "C" {
    pub fn nvkm_devinit_post(: *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv04_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv05_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv10_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv1a_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv20_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn nv50_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn g84_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn g98_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn gt215_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn mcp89_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn gf100_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn gm107_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn gm200_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn gv100_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn tu102_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
extern "C" {
    pub fn ga100_devinit_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_devinit) -> c_int;
}
