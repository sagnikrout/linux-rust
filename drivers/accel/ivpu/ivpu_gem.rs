//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_gem.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020-2025 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_bo {
    pub base: drm_gem_shmem_object,
    pub ctx: *mut ivpu_mmu_context,
    pub bo_list_node: list_head,
    pub mm_node: drm_mm_node,
    pub vpu_addr: u64,
    pub flags: u32,
    pub /: *mut *mut u32 job_status; / Valid only for command buffer,
    pub ctx_id: u32,
    pub mmu_mapped: bool,
}

extern "C" {
    pub fn ivpu_bo_bind(bo: *mut ivpu_bo) -> c_int;
}
extern "C" {
    pub fn ivpu_bo_unbind_all_bos_from_context(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context);
}
extern "C" {
    pub fn ivpu_bo_free(bo: *mut ivpu_bo);
}
extern "C" {
    pub fn ivpu_bo_create_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_bo_info_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_bo_wait_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn ivpu_bo_list(dev: *mut drm_device, p: *mut drm_printer);
}
extern "C" {
    pub fn ivpu_bo_list_print(dev: *mut drm_device);
}
extern "C" {
    pub fn container_of(_arg: obj, ivpu_bo: struct, _arg: base.base) -> return;
}
extern "C" {
    pub fn to_ivpu_device(_arg: bo->base.base.dev) -> return;
}
extern "C" {
    pub fn ivpu_bo_vaddr(bo->vpu_addr: bo) + (vpu_addr -) -> return;
}
