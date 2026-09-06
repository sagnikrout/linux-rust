//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bus.h
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
pub struct nvkm_bus {
    pub func: *const nvkm_bus_func,
    pub subdev: nvkm_subdev,
}

// interface to sequencer
extern "C" {
    pub fn nvkm_hwsq_init(: *mut nvkm_subdev, : *mut nvkm_hwsq) -> c_int;
}
extern "C" {
    pub fn nvkm_hwsq_fini(: *mut nvkm_hwsq, exec: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_hwsq_wr32(: *mut nvkm_hwsq, addr: u32, data: u32);
}
extern "C" {
    pub fn nvkm_hwsq_setf(: *mut nvkm_hwsq, flag: u8, data: c_int);
}
extern "C" {
    pub fn nvkm_hwsq_wait(: *mut nvkm_hwsq, flag: u8, data: u8);
}
extern "C" {
    pub fn nvkm_hwsq_wait_vblank(: *mut nvkm_hwsq);
}
extern "C" {
    pub fn nvkm_hwsq_nsec(: *mut nvkm_hwsq, nsec: u32);
}
extern "C" {
    pub fn nv04_bus_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_bus) -> c_int;
}
extern "C" {
    pub fn nv31_bus_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_bus) -> c_int;
}
extern "C" {
    pub fn nv50_bus_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_bus) -> c_int;
}
extern "C" {
    pub fn g94_bus_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_bus) -> c_int;
}
extern "C" {
    pub fn gf100_bus_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_bus) -> c_int;
}
