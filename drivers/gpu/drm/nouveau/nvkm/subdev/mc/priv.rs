//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mc/priv.h
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
pub struct nvkm_mc_map {
    pub stat: u32,
    pub type: nvkm_subdev_type,
    pub inst: c_int,
    pub noauto: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_mc_func {
    pub ): *mut *mut void (init)(struct nvkm_mc,
    pub intr: *const nvkm_intr_func,
    pub intrs: *const nvkm_intr_data,
    pub intr_nonstall: bool,
    pub mask): *mut *mut *mut bool (enabled)(struct nvkm_mc , u32,
    pub mask): *mut *mut *mut void (enable)(struct nvkm_mc , u32,
    pub mask): *mut *mut *mut void (disable)(struct nvkm_mc , u32,
    pub device: *mut },
    pub reset: *const nvkm_mc_map,
    pub u32): *mut *mut *mut void (unk260)(struct nvkm_mc ,,
}

extern "C" {
    pub fn nv04_mc_init(: *mut nvkm_mc);
}
extern "C" {
    pub fn nv04_mc_intr_pending(: *mut nvkm_intr) -> bool;
}
extern "C" {
    pub fn nv04_mc_intr_unarm(: *mut nvkm_intr);
}
extern "C" {
    pub fn nv04_mc_intr_rearm(: *mut nvkm_intr);
}
extern "C" {
    pub fn nv44_mc_init(: *mut nvkm_mc);
}
extern "C" {
    pub fn nv50_mc_init(: *mut nvkm_mc);
}
extern "C" {
    pub fn gf100_mc_unk260(: *mut nvkm_mc, _arg: u32);
}
extern "C" {
    pub fn gk104_mc_init(: *mut nvkm_mc);
}
