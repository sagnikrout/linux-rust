//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_ttm_stolen_mgr.h
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

//
// struct xe_ttm_stolen_mgr - Xe TTM stolen memory manager
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ttm_stolen_mgr {
// @io_base: PCI base offset for CPU I/O access
    pub io_base: resource_size_t,
// @stolen_base: GPU base offset
    pub stolen_base: resource_size_t,
// @mapping: I/O memory mapping for CPU access
    pub mapping: *mut void __iomem,
}

extern "C" {
    pub fn xe_ttm_stolen_mgr_init(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_ttm_stolen_io_mem_reserve(xe: *mut xe_device, mem: *mut ttm_resource) -> c_int;
}
extern "C" {
    pub fn xe_ttm_stolen_cpu_access_needs_ggtt(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_ttm_stolen_io_offset(bo: *mut xe_bo, offset: u32) -> u64;
}
extern "C" {
    pub fn xe_ttm_stolen_gpu_offset(xe: *mut xe_device) -> u64;
}
