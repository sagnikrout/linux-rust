//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/priv.h
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
pub struct nvkm_pmu_func {
    pub flcn: *const nvkm_falcon_func,
    pub data: *mut u32,
    pub size: u32,
    pub code: },
    pub data: *mut u32,
    pub size: u32,
    pub data: },
    pub ): *mut *mut bool (enabled)(struct nvkm_pmu,
    pub ): *mut *mut void (reset)(struct nvkm_pmu,
    pub ): *mut *mut int (init)(struct nvkm_pmu,
    pub ): *mut *mut void (fini)(struct nvkm_pmu,
    pub ): *mut *mut void (intr)(struct nvkm_pmu,
    pub data1): u32 message, u32 data0, u32,
    pub ): *mut *mut void (recv)(struct nvkm_pmu,
    pub ): *mut *mut int (initmsg)(struct nvkm_pmu,
    pub bool): *mut *mut *mut void (pgob)(struct nvkm_pmu ,,
}

extern "C" {
    pub fn gt215_pmu_init(: *mut nvkm_pmu) -> c_int;
}
extern "C" {
    pub fn gt215_pmu_fini(: *mut nvkm_pmu);
}
extern "C" {
    pub fn gt215_pmu_intr(: *mut nvkm_pmu);
}
extern "C" {
    pub fn gt215_pmu_recv(: *mut nvkm_pmu);
}
extern "C" {
    pub fn gt215_pmu_send(: *mut nvkm_pmu, _arg: u32[2], _arg: u32, _arg: u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf100_pmu_enabled(: *mut nvkm_pmu) -> bool;
}
extern "C" {
    pub fn gf100_pmu_reset(: *mut nvkm_pmu);
}
extern "C" {
    pub fn gp102_pmu_reset(pmu: *mut nvkm_pmu);
}
extern "C" {
    pub fn gk110_pmu_pgob(: *mut nvkm_pmu, _arg: bool);
}
extern "C" {
    pub fn gm200_pmu_flcn_bind_inst(: *mut nvkm_falcon, _arg: c_int, _arg: u64);
}
extern "C" {
    pub fn gm20b_pmu_acr_bld_patch(: *mut nvkm_acr, _arg: u32, _arg: i64);
}
extern "C" {
    pub fn gm20b_pmu_acr_bld_write(: *mut nvkm_acr, _arg: u32, : *mut nvkm_acr_lsfw);
}
extern "C" {
    pub fn gm20b_pmu_acr_bootstrap_falcon(: *mut nvkm_falcon, nvkm_acr_lsf_id: enum) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_pmu_fwif {
    pub version: c_int,
    pub ): *const *const *const int (load)(struct nvkm_pmu , int ver, struct nvkm_pmu_fwif,
    pub func: *const nvkm_pmu_func,
    pub acr: *const nvkm_acr_lsf_func,
}

extern "C" {
    pub fn gf100_pmu_nofw(: *mut nvkm_pmu, _arg: c_int, : *const nvkm_pmu_fwif) -> c_int;
}
extern "C" {
    pub fn gm200_pmu_nofw(: *mut nvkm_pmu, _arg: c_int, : *const nvkm_pmu_fwif) -> c_int;
}
extern "C" {
    pub fn gm20b_pmu_load(: *mut nvkm_pmu, _arg: c_int, : *const nvkm_pmu_fwif) -> c_int;
}
