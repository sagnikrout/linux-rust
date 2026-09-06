//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/bar/priv.h
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
pub struct nvkm_bar_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_bar,
    pub ): *mut *mut int (oneinit)(struct nvkm_bar,
    pub ): *mut *mut void (init)(struct nvkm_bar,
    pub ): *mut *mut void (init)(struct nvkm_bar,
    pub ): *mut *mut void (fini)(struct nvkm_bar,
    pub ): *mut *mut void (wait)(struct nvkm_bar,
    pub ): *mut *mut *mut nvkm_vmm (vmm)(nvkm_bar,
    pub bar2: } bar1,,
    pub ): *mut *mut void (flush)(struct nvkm_bar,
}

extern "C" {
    pub fn nv50_bar_bar1_fini(: *mut nvkm_bar);
}
extern "C" {
    pub fn nv50_bar_bar2_fini(: *mut nvkm_bar);
}
extern "C" {
    pub fn g84_bar_flush(: *mut nvkm_bar);
}
extern "C" {
    pub fn gf100_bar_bar1_fini(: *mut nvkm_bar);
}
extern "C" {
    pub fn gf100_bar_bar2_fini(: *mut nvkm_bar);
}
extern "C" {
    pub fn gm107_bar_bar1_wait(: *mut nvkm_bar);
}
