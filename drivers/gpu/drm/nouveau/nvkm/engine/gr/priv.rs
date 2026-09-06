//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/priv.h
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

extern "C" {
    pub fn nv04_gr_idle(: *mut nvkm_gr) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gr_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_gr,
    pub ): *mut *mut int (oneinit)(struct nvkm_gr,
    pub ): *mut *mut int (init)(struct nvkm_gr,
    pub bool): *mut *mut *mut int (fini)(struct nvkm_gr ,,
    pub ): *mut *mut int (reset)(struct nvkm_gr,
    pub ): *mut *mut int (nonstall)(struct nvkm_gr,
    pub ): *mut *mut void (intr)(struct nvkm_gr,
    pub ): *mut *mut *mut void (tile)(struct nvkm_gr , int region, struct nvkm_fb_tile,
    pub ): *mut *mut int (tlb_flush)(struct nvkm_gr,
    pub ): *const *const nvkm_oclass , nvkm_object,
    pub ): *mut *mut *mut int (object_get)(struct nvkm_gr , int, struct nvkm_sclass,
// Returns chipset-specific counts of units packed into an u64.
//
    pub ): *mut *mut u64 (units)(struct nvkm_gr,
    pub ): *mut *mut bool (chsw_load)(struct nvkm_gr,
    pub ): *mut *mut int (pause)(struct nvkm_gr,
    pub ): *mut *mut int (resume)(struct nvkm_gr,
    pub ): *mut *mut u32 (inst)(struct nvkm_gr,
    pub ctxsw: },
    pub sclass: [nvkm_sclass; ],
}
