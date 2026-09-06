//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/engine.h
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
pub struct nvkm_engine {
    pub func: *const nvkm_engine_func,
    pub subdev: nvkm_subdev,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_engine_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_engine,
    pub ): *mut *mut void (preinit)(struct nvkm_engine,
    pub ): *mut *mut int (oneinit)(struct nvkm_engine,
    pub data): *mut *mut *mut int (info)(struct nvkm_engine , u64 mthd, u64,
    pub ): *mut *mut int (init)(struct nvkm_engine,
    pub suspend): *mut *mut *mut int (fini)(struct nvkm_engine , enum nvkm_suspend_state,
    pub ): *mut *mut int (reset)(struct nvkm_engine,
    pub ): *mut *mut int (nonstall)(struct nvkm_engine,
    pub ): *mut *mut void (intr)(struct nvkm_engine,
    pub ): *mut *mut *mut void (tile)(struct nvkm_engine , int region, struct nvkm_fb_tile,
    pub ): *mut *mut bool (chsw_load)(struct nvkm_engine,
    pub ): *const nvkm_device_oclass,
    pub base: },
    pub ): *mut nvkm_object,
    pub index): *mut *mut *mut int (sclass)(struct nvkm_oclass , int,
    pub fifo: },
    pub cclass: *const nvkm_object_func,
    pub sclass: [nvkm_sclass; ],
}

extern "C" {
    pub fn nvkm_engine_unref(: *mut nvkm_engine);
}
extern "C" {
    pub fn nvkm_engine_reset(: *mut nvkm_engine) -> c_int;
}
extern "C" {
    pub fn nvkm_engine_tile(: *mut nvkm_engine, region: c_int);
}
extern "C" {
    pub fn nvkm_engine_chsw_load(: *mut nvkm_engine) -> bool;
}
