//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/acr.h
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
pub enum nvkm_acr_lsf_id {
    NVKM_ACR_LSF_PMU = 0,
    NVKM_ACR_LSF_GSPLITE = 1,
    NVKM_ACR_LSF_FECS = 2,
    NVKM_ACR_LSF_GPCCS = 3,
    NVKM_ACR_LSF_NVDEC = 4,
    NVKM_ACR_LSF_SEC2 = 7,
    NVKM_ACR_LSF_MINION = 10,
    NVKM_ACR_LSF_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr {
    pub func: *const nvkm_acr_func,
    pub subdev: nvkm_subdev,
    pub hsfw: list_head,
    pub lsf: list_head lsfw,,
    pub managed_falcons: u64,
    pub wpr: *mut nvkm_memory,
    pub wpr_start: u64,
    pub wpr_end: u64,
    pub shadow_start: u64,
    pub inst: *mut nvkm_memory,
    pub vmm: *mut nvkm_vmm,
    pub done: bool,
    pub rtos: *mut nvkm_acr_lsf,
    pub wpr_fw: *const firmware,
    pub wpr_comp: bool,
    pub wpr_prev: u64,
}

extern "C" {
    pub fn nvkm_acr_managed_falcon(: *mut nvkm_device, nvkm_acr_lsf_id: enum) -> bool;
}
extern "C" {
    pub fn nvkm_acr_bootstrap_falcons(: *mut nvkm_device, mask: c_ulong) -> c_int;
}
extern "C" {
    pub fn gm200_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gm20b_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gp102_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gp108_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gp10b_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn gv100_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn tu102_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
extern "C" {
    pub fn ga102_acr_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_acr) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_lsfw {
    pub func: *const nvkm_acr_lsf_func,
    pub falcon: *mut nvkm_falcon,
    pub id: nvkm_acr_lsf_id,
    pub head: list_head,
    pub img: nvkm_blob,
    pub sig: *const firmware,
    pub secure_bootloader: bool,
    pub bootloader_size: u32,
    pub bootloader_imem_offset: u32,
    pub app_size: u32,
    pub app_start_offset: u32,
    pub app_imem_entry: u32,
    pub app_resident_code_offset: u32,
    pub app_resident_code_size: u32,
    pub app_resident_data_offset: u32,
    pub app_resident_data_size: u32,
    pub app_imem_offset: u32,
    pub app_dmem_offset: u32,
    pub ucode_size: u32,
    pub data_size: u32,
    pub fuse_ver: u32,
    pub engine_id: u32,
    pub ucode_id: u32,
    pub sig_size: u32,
    pub sig_nr: u32,
    pub sigs: *mut u8,
    pub lsb: u32,
    pub img: u32,
    pub bld: u32,
    pub offset: },
    pub bl_data_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_acr_lsf_func {
// The (currently) map directly to LSB header flags.
pub const NVKM_ACR_LSF_LOAD_CODE_AT_0: c_uint = 0x00000001;
pub const NVKM_ACR_LSF_DMACTL_REQ_CTX: c_uint = 0x00000004;
pub const NVKM_ACR_LSF_FORCE_PRIV_LOAD: c_uint = 0x00000008;
    pub flags: u32,
    pub bl_entry: u32,
    pub bld_size: u32,
    pub ): *mut *mut *mut void (bld_write)(struct nvkm_acr , u32 bld, struct nvkm_acr_lsfw,
    pub adjust): *mut *mut *mut void (bld_patch)(struct nvkm_acr , u32 bld, s64,
    pub bootstrap_falcons: u64,
    pub nvkm_acr_lsf_id): *mut *mut *mut int (bootstrap_falcon)(struct nvkm_falcon , enum,
    pub mask): *mut *mut *mut int (bootstrap_multiple_falcons)(struct nvkm_falcon , u32,
}
