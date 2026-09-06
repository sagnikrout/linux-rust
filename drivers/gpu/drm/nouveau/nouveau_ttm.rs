//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_ttm.h
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
    pub fn container_of(_arg: bd, nouveau_drm: struct, _arg: ttm.bdev) -> return;
}
extern "C" {
    pub fn nouveau_ttm_init(drm: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nouveau_ttm_fini(drm: *mut nouveau_drm);
}
extern "C" {
    pub fn nouveau_ttm_global_init(: *mut nouveau_drm) -> c_int;
}
extern "C" {
    pub fn nouveau_ttm_global_release(: *mut nouveau_drm);
}
extern "C" {
    pub fn nouveau_sgdma_bind(bdev: *mut ttm_device, ttm: *mut ttm_tt, reg: *mut ttm_resource) -> c_int;
}
extern "C" {
    pub fn nouveau_sgdma_unbind(bdev: *mut ttm_device, ttm: *mut ttm_tt);
}
extern "C" {
    pub fn nouveau_sgdma_destroy(bdev: *mut ttm_device, ttm: *mut ttm_tt);
}
