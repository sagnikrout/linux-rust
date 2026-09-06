//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem_vram_helper.h
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

//
// Buffer-object helpers
//
// struct drm_gem_vram_object - GEM object backed by VRAM
// @bo:		TTM buffer object
// @map:	Mapping information for @bo
// @placement:	TTM placement information. Supported placements are %TTM_PL_VRAM
// and %TTM_PL_SYSTEM
// @placements:	TTM placement information.
//
// The type struct drm_gem_vram_object represents a GEM object that is
// backed by VRAM. It can be used for simple framebuffer devices with
// dedicated memory. The buffer object can be evicted to system memory if
// video memory becomes scarce.
//
// GEM VRAM objects perform reference counting for pin and mapping
// operations. So a buffer object that has been pinned N times with
// drm_gem_vram_pin() must be unpinned N times with
// drm_gem_vram_unpin(). The same applies to pairs of
// drm_gem_vram_kmap() and drm_gem_vram_kunmap(), as well as pairs of
// drm_gem_vram_vmap() and drm_gem_vram_vunmap().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_vram_object {
    pub bo: ttm_buffer_object,
    pub map: iosys_map,
//
// @vmap_use_count:
//
// Reference count on the virtual address.
// The address are un-mapped when the count reaches zero.
//
    pub vmap_use_count: c_uint,
// Supported placements are %TTM_PL_VRAM and %TTM_PL_SYSTEM
    pub placement: ttm_placement,
    pub placements: [ttm_place; 2],
}

//
// drm_gem_vram_of_bo - Returns the container of type
// &struct drm_gem_vram_object for field bo.
// @bo:		the VRAM buffer object
// Returns:	The containing GEM VRAM object
//
extern "C" {
    pub fn container_of(_arg: bo, drm_gem_vram_object: struct, _arg: bo) -> return;
}
//
// drm_gem_vram_of_gem - Returns the container of type
// &struct drm_gem_vram_object for field gem.
// @gem:	the GEM object
// Returns:	The containing GEM VRAM object
//
extern "C" {
    pub fn container_of(_arg: gem, drm_gem_vram_object: struct, _arg: bo.base) -> return;
}
extern "C" {
    pub fn drm_gem_vram_put(gbo: *mut drm_gem_vram_object);
}
extern "C" {
    pub fn drm_gem_vram_offset(gbo: *mut drm_gem_vram_object) -> i64;
}
extern "C" {
    pub fn drm_gem_vram_vmap(gbo: *mut drm_gem_vram_object, map: *mut iosys_map) -> c_int;
}
//
// Helpers for struct drm_driver
//
// Helpers for struct drm_plane_helper_funcs
//
// DRM_GEM_VRAM_PLANE_HELPER_FUNCS - Initializes struct drm_plane_helper_funcs
// for VRAM handling
//
// Drivers may use GEM BOs as VRAM helpers for the framebuffer memory. This
// macro initializes struct drm_plane_helper_funcs to use the respective helper
// functions.
//

//
// define DRM_GEM_VRAM_DRIVER - default callback functions for
// &struct drm_driver
//
// Drivers that use VRAM MM and GEM VRAM can use this macro to initialize
// &struct drm_driver with default functions.
//

//
// VRAM memory manager
//
// struct drm_vram_mm - An instance of VRAM MM
// @vram_base:	Base address of the managed video memory
// @vram_size:	Size of the managed video memory in bytes
// @bdev:	The TTM BO device.
//
// The fields &struct drm_vram_mm.vram_base and
// &struct drm_vram_mm.vrm_size are managed by VRAM MM, but are
// available for public read access. Use the field
// &struct drm_vram_mm.bdev to access the TTM BO device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vram_mm {
    pub vram_base: u64,
    pub vram_size: usize,
    pub bdev: ttm_device,
}

//
// drm_vram_mm_of_bdev() - Returns the container of type &struct ttm_device for
// field bdev.
// @bdev:	the TTM BO device
//
// Returns:
// The containing instance of &struct drm_vram_mm
//
extern "C" {
    pub fn container_of(_arg: bdev, drm_vram_mm: struct, _arg: bdev) -> return;
}
extern "C" {
    pub fn drm_vram_mm_debugfs_init(minor: *mut drm_minor);
}
//
// Helpers for integration with struct drm_device
//
// Mode-config helpers
//
