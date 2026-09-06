//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/ltc/priv.h
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
pub struct nvkm_ltc_func {
    pub ): *mut *mut int (oneinit)(struct nvkm_ltc,
    pub ): *mut *mut void (init)(struct nvkm_ltc,
    pub ): *mut *mut void (intr)(struct nvkm_ltc,
    pub limit): *mut *mut *mut void (cbc_clear)(struct nvkm_ltc , u32 start, u32,
    pub ): *mut *mut void (cbc_wait)(struct nvkm_ltc,
    pub zbc_color: c_int,
    pub zbc_depth: c_int,
    pub u32[4]): *const *const *const void (zbc_clear_color)(struct nvkm_ltc , int,,
    pub u32): *const *const *const void (zbc_clear_depth)(struct nvkm_ltc , int,,
    pub u32): *const *const *const void (zbc_clear_stencil)(struct nvkm_ltc , int,,
    pub ): *mut *mut void (invalidate)(struct nvkm_ltc,
    pub ): *mut *mut void (flush)(struct nvkm_ltc,
}

extern "C" {
    pub fn gf100_ltc_oneinit(: *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gf100_ltc_oneinit_tag_ram(: *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gf100_ltc_intr(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gf100_ltc_cbc_clear(: *mut nvkm_ltc, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn gf100_ltc_cbc_wait(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gf100_ltc_zbc_clear_color(: *mut nvkm_ltc, _arg: c_int, u32[4]: const);
}
extern "C" {
    pub fn gf100_ltc_zbc_clear_depth(: *mut nvkm_ltc, _arg: c_int, u32: const);
}
extern "C" {
    pub fn gf100_ltc_invalidate(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gf100_ltc_flush(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gm107_ltc_intr(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gm107_ltc_intr_lts(: *mut nvkm_ltc, ltc: c_int, lts: c_int);
}
extern "C" {
    pub fn gm107_ltc_cbc_clear(: *mut nvkm_ltc, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn gm107_ltc_cbc_wait(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gm107_ltc_zbc_clear_color(: *mut nvkm_ltc, _arg: c_int, u32[4]: const);
}
extern "C" {
    pub fn gm107_ltc_zbc_clear_depth(: *mut nvkm_ltc, _arg: c_int, u32: const);
}
extern "C" {
    pub fn gp100_ltc_oneinit(: *mut nvkm_ltc) -> c_int;
}
extern "C" {
    pub fn gp100_ltc_init(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gp100_ltc_intr(: *mut nvkm_ltc);
}
extern "C" {
    pub fn gp102_ltc_zbc_clear_stencil(: *mut nvkm_ltc, _arg: c_int, u32: const);
}
