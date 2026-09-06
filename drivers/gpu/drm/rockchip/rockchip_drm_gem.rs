//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rockchip_drm_gem.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:Mark Yao <mark.yao@rock-chips.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_gem_object {
    pub base: drm_gem_object,
    pub flags: c_uint,
    pub kvaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
// Used when IOMMU is disabled
    pub dma_attrs: c_ulong,
// Used when IOMMU is enabled
    pub mm: drm_mm_node,
    pub num_pages: c_ulong,
    pub pages: *mut page,
    pub sgt: *mut sg_table,
    pub size: usize,
}

extern "C" {
    pub fn rockchip_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn rockchip_gem_free_object(obj: *mut drm_gem_object);
}
