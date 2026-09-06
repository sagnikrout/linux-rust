//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_ttm_vram_mgr.h
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
    pub fn xe_ttm_vram_mgr_init(xe: *mut xe_device, vram: *mut xe_vram_region) -> c_int;
}
extern "C" {
    pub fn xe_ttm_vram_get_avail(man: *mut ttm_resource_manager) -> u64;
}
extern "C" {
    pub fn xe_ttm_vram_get_cpu_visible_size(man: *mut ttm_resource_manager) -> u64;
}
extern "C" {
    pub fn container_of(_arg: res, xe_ttm_vram_mgr_resource: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: man, xe_ttm_vram_mgr: struct, _arg: manager) -> return;
}
