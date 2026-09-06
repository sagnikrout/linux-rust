//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/intr.h
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
pub enum nvkm_intr_prio {
    NVKM_INTR_PRIO_VBLANK = 0,
    NVKM_INTR_PRIO_NORMAL,
    NVKM_INTR_PRIO_NR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_intr_type {
    NVKM_INTR_SUBDEV   = -1, /* lookup vector by requesting subdev, in mapping table. */
    NVKM_INTR_VECTOR_0 = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_intr {
    pub ): *mut *mut bool (pending)(struct nvkm_intr,
    pub ): *mut *mut void (unarm)(struct nvkm_intr,
    pub ): *mut *mut void (rearm)(struct nvkm_intr,
    pub mask): *mut *mut *mut void (block)(struct nvkm_intr , int leaf, u32,
    pub mask): *mut *mut *mut void (allow)(struct nvkm_intr , int leaf, u32,
    pub mask): *mut *mut *mut void (reset)(struct nvkm_intr , int leaf, u32,
    pub func: *mut },
    pub /: *mut *mut int type; / enum nvkm_subdev_type (+ve), enum nvkm_intr_type (-ve),
    pub inst: c_int,
    pub leaf: c_int,
    pub /: *mut *mut u32 mask; / 0-terminated.,
    pub /: *mut *mut bool legacy; / auto-create "legacy" nvkm_subdev_intr() handler,
    pub data: *mut },
    pub subdev: *mut nvkm_subdev,
    pub leaves: c_int,
    pub stat: *mut u32,
    pub mask: *mut u32,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_intr_ctor(: *mut nvkm_device);
}
extern "C" {
    pub fn nvkm_intr_dtor(: *mut nvkm_device);
}
extern "C" {
    pub fn nvkm_intr_install(: *mut nvkm_device) -> c_int;
}
extern "C" {
    pub fn nvkm_intr_unarm(: *mut nvkm_device);
}
extern "C" {
    pub fn nvkm_intr_rearm(: *mut nvkm_device);
}
extern "C" {
    pub fn nvkm_intr_block(: *mut nvkm_subdev, nvkm_intr_type: enum);
}
extern "C" {
    pub fn nvkm_intr_allow(: *mut nvkm_subdev, nvkm_intr_type: enum);
}
extern "C" {
    pub fn irqreturn_t(: *mut *mut nvkm_inth_func)(struct nvkm_inth) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_inth {
    pub intr: *mut nvkm_intr,
    pub leaf: c_int,
    pub mask: u32,
    pub func: nvkm_inth_func,
    pub allowed: core::sync::atomic::AtomicI32,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_inth_allow(: *mut nvkm_inth);
}
extern "C" {
    pub fn nvkm_inth_block(: *mut nvkm_inth);
}
