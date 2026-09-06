//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/instmem.h
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
pub struct nvkm_instmem {
    pub func: *const nvkm_instmem_func,
    pub subdev: nvkm_subdev,
    pub suspend: bool,
    pub lock: spinlock_t,
    pub list: list_head,
    pub boot: list_head,
    pub reserved: u32,
// <=nv4x: protects NV_PRAMIN/BAR2 MM
// >=nv50: protects BAR2 MM & LRU
//
    pub mutex: mutex,
    pub vbios: *mut nvkm_memory,
    pub ramht: *mut nvkm_ramht,
    pub ramro: *mut nvkm_memory,
    pub ramfc: *mut nvkm_memory,
}

extern "C" {
    pub fn nvkm_instmem_rd32(: *mut nvkm_instmem, addr: u32) -> u32;
}
extern "C" {
    pub fn nvkm_instmem_wr32(: *mut nvkm_instmem, addr: u32, data: u32);
}
extern "C" {
    pub fn nvkm_instobj_wrap(: *mut nvkm_device, : *mut nvkm_memory, : *mut nvkm_memory) -> c_int;
}
extern "C" {
    pub fn nv04_instmem_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_instmem) -> c_int;
}
extern "C" {
    pub fn nv40_instmem_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_instmem) -> c_int;
}
extern "C" {
    pub fn nv50_instmem_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_instmem) -> c_int;
}
extern "C" {
    pub fn gk20a_instmem_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_instmem) -> c_int;
}
extern "C" {
    pub fn gh100_instmem_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_instmem) -> c_int;
}
