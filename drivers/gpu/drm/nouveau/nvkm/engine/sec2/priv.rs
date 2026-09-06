//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/sec2/priv.h
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
pub struct nvkm_sec2_func {
    pub flcn: *const nvkm_falcon_func,
    pub unit_unload: u8,
    pub unit_acr: u8,
    pub ): *mut *mut *mut *mut nvkm_intr (intr_vector)(nvkm_sec2 , enum nvkm_intr_type,
    pub ): *mut *mut irqreturn_t (intr)(struct nvkm_inth,
    pub ): *mut *mut int (initmsg)(struct nvkm_sec2,
}

extern "C" {
    pub fn gp102_sec2_intr(: *mut nvkm_inth) -> irqreturn_t;
}
extern "C" {
    pub fn gp102_sec2_initmsg(: *mut nvkm_sec2) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_sec2_fwif {
    pub version: c_int,
    pub ): *const *const *const int (load)(struct nvkm_sec2 , int ver, struct nvkm_sec2_fwif,
    pub func: *const nvkm_sec2_func,
    pub acr: *const nvkm_acr_lsf_func,
}

extern "C" {
    pub fn gp102_sec2_nofw(: *mut nvkm_sec2, _arg: c_int, : *const nvkm_sec2_fwif) -> c_int;
}
extern "C" {
    pub fn gp102_sec2_load(: *mut nvkm_sec2, _arg: c_int, : *const nvkm_sec2_fwif) -> c_int;
}
extern "C" {
    pub fn gp102_sec2_acr_bld_write_1(: *mut nvkm_acr, _arg: u32, : *mut nvkm_acr_lsfw);
}
extern "C" {
    pub fn gp102_sec2_acr_bld_patch_1(: *mut nvkm_acr, _arg: u32, _arg: i64);
}
