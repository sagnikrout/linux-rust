//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sa.h
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
//
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn __xe_sa_bo_manager_init(_arg: tile, _arg: size, _arg: SZ_4K, _arg: align, _arg: 0) -> return;
}
//
// xe_sa_bo_new() - Make a suballocation.
// @sa_manager: the &xe_sa_manager
// @size: number of bytes we want to suballocate
//
// Try to make a suballocation of size @size.
//
// Return: a &drm_suballoc, or an ERR_PTR.
//
extern "C" {
    pub fn __xe_sa_bo_new(_arg: sa_manager, _arg: size, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn xe_sa_bo_init(sa_manager: *mut xe_sa_manager, sa: *mut drm_suballoc, size: usize) -> c_int;
}
extern "C" {
    pub fn xe_sa_bo_flush_write(sa_bo: *mut drm_suballoc);
}
extern "C" {
    pub fn xe_sa_bo_sync_read(sa_bo: *mut drm_suballoc);
}
extern "C" {
    pub fn xe_sa_bo_free(sa_bo: *mut drm_suballoc, fence: *mut dma_fence);
}
extern "C" {
    pub fn container_of(_arg: mng, xe_sa_manager: struct, _arg: base) -> return;
}
//
// xe_sa_manager_gpu_addr - Retrieve GPU address of a back storage BO
// within suballocator.
// @sa_manager: the &xe_sa_manager struct instance
// Return: GGTT address of the back storage BO.
//
extern "C" {
    pub fn xe_bo_ggtt_addr(_arg: sa_manager->bo) -> return;
}
extern "C" {
    pub fn xe_sa_bo_swap_shadow(sa_manager: *mut xe_sa_manager);
}
extern "C" {
    pub fn xe_sa_bo_sync_shadow(sa_bo: *mut drm_suballoc);
}
//
// xe_sa_bo_swap_guard() - Retrieve the SA BO swap guard within sub-allocator.
// @sa_manager: the &xe_sa_manager
//
// Return: Sub alloctor swap guard mutex.
//
