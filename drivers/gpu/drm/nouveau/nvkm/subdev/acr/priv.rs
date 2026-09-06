//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/acr/priv.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_fwif {
    pub version: c_int,
    pub ): *const nvkm_acr_fwif,
    pub func: *const nvkm_acr_func,
}

extern "C" {
    pub fn gm200_acr_nofw(: *mut nvkm_acr, _arg: c_int, : *const nvkm_acr_fwif) -> c_int;
}
extern "C" {
    pub fn gm20b_acr_load(: *mut nvkm_acr, _arg: c_int, : *const nvkm_acr_fwif) -> c_int;
}
extern "C" {
    pub fn gp102_acr_load(: *mut nvkm_acr, _arg: c_int, : *const nvkm_acr_fwif) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_func {
    pub load: *const nvkm_acr_hsf_fwif,
    pub ahesasc: *const nvkm_acr_hsf_fwif,
    pub asb: *const nvkm_acr_hsf_fwif,
    pub unload: *const nvkm_acr_hsf_fwif,
    pub ): *mut *mut int (wpr_parse)(struct nvkm_acr,
    pub ): *mut *mut u32 (wpr_layout)(struct nvkm_acr,
    pub wpr_size): *mut *mut *mut int (wpr_alloc)(struct nvkm_acr , u32,
    pub rtos): *mut *mut *mut int (wpr_build)(struct nvkm_acr , struct nvkm_acr_lsf,
    pub adjust): *mut *mut *mut int (wpr_patch)(struct nvkm_acr , s64,
    pub limit): *mut *mut *mut *mut void (wpr_check)(struct nvkm_acr , u64 start, u64,
    pub ): *mut *mut int (init)(struct nvkm_acr,
    pub ): *mut *mut void (fini)(struct nvkm_acr,
    pub bootstrap_falcons: u64,
}

extern "C" {
    pub fn gm200_acr_wpr_parse(: *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gm200_acr_wpr_layout(: *mut nvkm_acr) -> u32;
}
extern "C" {
    pub fn gm200_acr_wpr_build(: *mut nvkm_acr, : *mut nvkm_acr_lsf) -> c_int;
}
extern "C" {
    pub fn gm200_acr_wpr_patch(: *mut nvkm_acr, _arg: i64) -> c_int;
}
extern "C" {
    pub fn gm200_acr_wpr_check(: *mut nvkm_acr, : *mut u64, : *mut u64);
}
extern "C" {
    pub fn gm200_acr_init(: *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gm20b_acr_wpr_alloc(: *mut nvkm_acr, wpr_size: u32) -> c_int;
}
extern "C" {
    pub fn gp102_acr_wpr_parse(: *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gp102_acr_wpr_layout(: *mut nvkm_acr) -> u32;
}
extern "C" {
    pub fn gp102_acr_wpr_alloc(: *mut nvkm_acr, wpr_size: u32) -> c_int;
}
extern "C" {
    pub fn gp102_acr_wpr_build(: *mut nvkm_acr, : *mut nvkm_acr_lsf) -> c_int;
}
extern "C" {
    pub fn gp102_acr_wpr_build_lsb(: *mut nvkm_acr, : *mut nvkm_acr_lsfw) -> c_int;
}
extern "C" {
    pub fn gp102_acr_wpr_patch(: *mut nvkm_acr, _arg: i64) -> c_int;
}
extern "C" {
    pub fn tu102_acr_init(: *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn ga100_acr_wpr_check(: *mut nvkm_acr, : *mut u64, : *mut u64);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_hsfw {
    pub fw: nvkm_falcon_fw,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_acr_hsf_id {
    NVKM_ACR_HSF_PMU,
    NVKM_ACR_HSF_SEC2,
    NVKM_ACR_HSF_GSP,
    } falcon_id;
    u32 boot_mbox0;
    u32 intr_clear;

    struct list_head head;
}

    pub name): *const *const int nvkm_acr_hsfw_boot(struct nvkm_acr , char,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_hsf_fwif {
    pub version: c_int,
    pub ): *const nvkm_acr_hsf_fwif,
    pub func: *const nvkm_falcon_fw_func,
    pub falcon_id: nvkm_acr_hsf_id,
    pub boot_mbox0: u32,
    pub intr_clear: u32,
}

extern "C" {
    pub fn gm200_acr_hsfw_load_bld(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn gp102_acr_load_setup(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn gp108_acr_hsfw_load_bld(: *mut nvkm_falcon_fw) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_lsf {
    pub func: *const nvkm_acr_lsf_func,
    pub falcon: *mut nvkm_falcon,
    pub id: nvkm_acr_lsf_id,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_acr_lsfw_del(: *mut nvkm_acr_lsfw);
}
extern "C" {
    pub fn nvkm_acr_lsfw_del_all(: *mut nvkm_acr);
}
