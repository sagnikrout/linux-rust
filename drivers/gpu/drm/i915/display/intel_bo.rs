//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_bo.h
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
// Copyright © 2024 Intel Corporation

extern "C" {
    pub fn intel_bo_is_tiled(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn intel_bo_is_userptr(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn intel_bo_is_shmem(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn intel_bo_is_protected(obj: *mut drm_gem_object) -> bool;
}
extern "C" {
    pub fn intel_bo_key_check(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn intel_bo_fb_mmap(obj: *mut drm_gem_object, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn intel_bo_read_from_page(obj: *mut drm_gem_object, offset: u64, dst: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn intel_bo_describe(m: *mut seq_file, obj: *mut drm_gem_object);
}
extern "C" {
    pub fn intel_bo_framebuffer_fini(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn intel_bo_framebuffer_init(obj: *mut drm_gem_object, mode_cmd: *mut drm_mode_fb_cmd2) -> c_int;
}
extern "C" {
    pub fn intel_bo_fbdev_pitch_align(display: *mut intel_display, stride: u32) -> u32;
}
extern "C" {
    pub fn intel_bo_fbdev_destroy(obj: *mut drm_gem_object);
}
