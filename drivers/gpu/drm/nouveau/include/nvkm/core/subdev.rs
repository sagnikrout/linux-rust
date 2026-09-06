//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/subdev.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_subdev_type {

    NVKM_SUBDEV_NR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_subdev {
    pub func: *const nvkm_subdev_func,
    pub device: *mut nvkm_device,
    pub type: nvkm_subdev_type,
    pub inst: c_int,
    pub name: [c_char; 16],
    pub debug: u32,
    pub refcount: refcount_t,
    pub mutex: mutex,
    pub enabled: bool,
    pub use: },
    pub inth: nvkm_inth,
    pub head: list_head,
    pub pself: *mut c_void,
    pub oneinit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_subdev_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_subdev,
    pub ): *mut *mut int (preinit)(struct nvkm_subdev,
    pub ): *mut *mut int (oneinit)(struct nvkm_subdev,
    pub data): *mut *mut *mut int (info)(struct nvkm_subdev , u64 mthd, u64,
    pub ): *mut *mut int (init)(struct nvkm_subdev,
    pub suspend): *mut *mut *mut int (fini)(struct nvkm_subdev , enum nvkm_suspend_state,
    pub ): *mut *mut void (intr)(struct nvkm_subdev,
}

extern "C" {
    pub fn nvkm_subdev_disable(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int);
}
extern "C" {
    pub fn nvkm_subdev_del(: *mut nvkm_subdev);
}
extern "C" {
    pub fn nvkm_subdev_ref(: *mut nvkm_subdev) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_unref(: *mut nvkm_subdev);
}
extern "C" {
    pub fn nvkm_subdev_preinit(: *mut nvkm_subdev) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_oneinit(: *mut nvkm_subdev) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_init(: *mut nvkm_subdev) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_fini(: *mut nvkm_subdev, suspend: nvkm_suspend_state) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_info(: *mut nvkm_subdev, _arg: u64, : *mut u64) -> c_int;
}
extern "C" {
    pub fn nvkm_subdev_intr(: *mut nvkm_subdev);
}
// subdev logging

