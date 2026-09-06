//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/fuse.h
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
pub struct nvkm_fuse {
    pub func: *const nvkm_fuse_func,
    pub subdev: nvkm_subdev,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn nvkm_fuse_read(: *mut nvkm_fuse, addr: u32) -> u32;
}
extern "C" {
    pub fn nv50_fuse_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fuse) -> c_int;
}
extern "C" {
    pub fn gf100_fuse_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fuse) -> c_int;
}
extern "C" {
    pub fn gm107_fuse_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fuse) -> c_int;
}
