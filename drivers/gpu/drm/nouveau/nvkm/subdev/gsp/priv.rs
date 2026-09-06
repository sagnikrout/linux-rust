//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/priv.h
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
    pub fn nvkm_gsp_fwsec_frts(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_fwsec_sb(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_fwsec_sb_init(gsp: *mut nvkm_gsp) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_fwif {
    pub version: c_int,
    pub ): *const *const *const int (load)(struct nvkm_gsp , int ver, struct nvkm_gsp_fwif,
    pub func: *const nvkm_gsp_func,
    pub rm: *const nvkm_rm_impl,
    pub ver: *const c_char,
}

extern "C" {
    pub fn nvkm_gsp_dtor_fws(: *mut nvkm_gsp);
}
extern "C" {
    pub fn gv100_gsp_nofw(: *mut nvkm_gsp, _arg: c_int, : *const nvkm_gsp_fwif) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_load(: *mut nvkm_gsp, _arg: c_int, : *const nvkm_gsp_fwif) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_load_rm(: *mut nvkm_gsp, : *const nvkm_gsp_fwif) -> c_int;
}
extern "C" {
    pub fn gh100_gsp_load(: *mut nvkm_gsp, _arg: c_int, : *const nvkm_gsp_fwif) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_func {
    pub flcn: *const nvkm_falcon_func,
    pub fwsec: *const nvkm_falcon_fw_func,
    pub sig_section: *mut c_char,
    pub ): *mut *mut nvkm_falcon , nvkm_falcon_fw,
    pub booter: },
    pub ): *mut *mut int (ctor)(struct nvkm_gsp,
    pub ): *mut *mut void (dtor)(struct nvkm_gsp,
    pub fwsec_sb: },
    pub ): *mut *mut void (dtor)(struct nvkm_gsp,
    pub ): *mut *mut int (oneinit)(struct nvkm_gsp,
    pub ): *mut *mut int (init)(struct nvkm_gsp,
    pub suspend): *mut *mut *mut int (fini)(struct nvkm_gsp , enum nvkm_suspend_state,
    pub ): *mut *mut int (reset)(struct nvkm_gsp,
    pub gpu: *const nvkm_rm_gpu,
    pub rm: },
}

extern "C" {
    pub fn tu102_gsp_fwsec_sb_ctor(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_fwsec_sb_dtor(: *mut nvkm_gsp);
}
extern "C" {
    pub fn tu102_gsp_oneinit(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_init(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_fini(: *mut nvkm_gsp, suspend: nvkm_suspend_state) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_reset(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_wpr_heap_size(: *mut nvkm_gsp) -> u64;
}
extern "C" {
    pub fn ga102_gsp_reset(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gh100_gsp_oneinit(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gh100_gsp_init(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gh100_gsp_fini(: *mut nvkm_gsp, suspend: nvkm_suspend_state) -> c_int;
}
extern "C" {
    pub fn r535_gsp_dtor(: *mut nvkm_gsp);
}
extern "C" {
    pub fn r535_gsp_oneinit(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn r535_gsp_init(: *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn r535_gsp_fini(: *mut nvkm_gsp, suspend: nvkm_suspend_state) -> c_int;
}
