//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/rocket/rocket_gem.h
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
// Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rocket_gem_object {
    pub base: drm_gem_shmem_object,
    pub driver_priv: *mut rocket_file_priv,
    pub domain: *mut rocket_iommu_domain,
    pub mm: drm_mm_node,
    pub size: usize,
    pub offset: u32,
}

extern "C" {
    pub fn rocket_ioctl_create_bo(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn rocket_ioctl_prep_bo(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn rocket_ioctl_fini_bo(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: to_drm_gem_shmem_obj(obj), rocket_gem_object: struct, _arg: base) -> return;
}
