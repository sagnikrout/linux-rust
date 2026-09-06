//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/exynos_drm_gem.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// exynos_drm_gem.h
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Authoer: Inki Dae <inki.dae@samsung.com>
//

//
// exynos drm buffer structure.
//
// @base: a gem object.
// - a new handle to this gem object would be created
// by drm_gem_handle_create().
// @flags: indicate memory type to allocated buffer and cache attruibute.
// @cookie: cookie returned by dma_alloc_attrs
// @kvaddr: kernel virtual address to allocated memory region (for fbdev)
// @dma_addr: bus address(accessed by dma) to allocated memory region.
// - this address could be physical address without IOMMU and
// device address with IOMMU.
// @dma_attrs: attrs passed dma mapping framework
// @sgt: Imported sg_table.
//
// P.S. this object would be transferred to user as kms_bo.handle so
// user can access the buffer through kms_bo.handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_gem {
    pub base: drm_gem_object,
    pub flags: c_uint,
    pub cookie: *mut c_void,
    pub kvaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub dma_attrs: c_ulong,
    pub sgt: *mut sg_table,
}

// destroy a buffer with gem object
extern "C" {
    pub fn exynos_drm_gem_destroy(exynos_gem: *mut exynos_drm_gem);
}
// create a new buffer with gem object
//
// request gem object creation and buffer allocation as the size
// that it is calculated with framebuffer information such as width,
// height and bpp.
//
// get fake-offset of gem object that can be used with mmap.
//
// get exynos drm object from gem handle, this function could be used for
// other drivers such as 2d/3d acceleration drivers.
// with this function call, gem object reference count would be increased.
//
// put exynos drm object acquired from exynos_drm_gem_get(),
// gem object reference count would be decreased.
//
// get buffer information to memory region allocated by gem.
// create memory region for drm framebuffer.
// low-level interface prime helpers
