//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/vfn.h
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
pub struct nvkm_vfn {
    pub func: *const nvkm_vfn_func,
    pub subdev: nvkm_subdev,
    pub priv: u32,
    pub user: u32,
    pub addr: },
    pub intr: nvkm_intr,
    pub user: nvkm_device_oclass,
}

extern "C" {
    pub fn gv100_vfn_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_vfn) -> c_int;
}
extern "C" {
    pub fn tu102_vfn_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_vfn) -> c_int;
}
extern "C" {
    pub fn ga100_vfn_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_vfn) -> c_int;
}
