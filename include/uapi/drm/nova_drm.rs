//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/nova_drm.h
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

// DISCLAIMER: Do not use, this is not a stable uAPI.
//
// This uAPI serves only testing purposes as long as this driver is still in
// development. It is required to implement and test infrastructure which is
// upstreamed in the context of this driver. See also [1].
//
// [1] https://lore.kernel.org/dri-devel/Zfsj0_tb-0-tNrJy@cassiopeiae/T/#u
//

//
// NOVA_GETPARAM_VRAM_BAR_SIZE
//
// Query the VRAM BAR size in bytes.
//
pub const NOVA_GETPARAM_VRAM_BAR_SIZE: c_uint = 0x1;
//
// struct drm_nova_getparam - query GPU and driver metadata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_getparam {
//
// @param: The identifier of the parameter to query.
//
    pub param: __u64,
//
// @value: The value for the specified parameter.
//
    pub value: __u64,
}

//
// struct drm_nova_gem_create - create a new DRM GEM object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_gem_create {
//
// @handle: The handle of the new DRM GEM object.
//
    pub handle: __u32,
//
// @pad: 32 bit padding, should be 0.
//
    pub pad: __u32,
//
// @size: The size of the new DRM GEM object.
//
    pub size: __u64,
}

//
// struct drm_nova_gem_info - query DRM GEM object metadata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_gem_info {
//
// @handle: The handle of the DRM GEM object to query.
//
    pub handle: __u32,
//
// @pad: 32 bit padding, should be 0.
//
    pub pad: __u32,
//
// @size: The size of the DRM GEM obejct.
//
    pub size: __u64,
}

pub const DRM_NOVA_GETPARAM: c_uint = 0x00;
pub const DRM_NOVA_GEM_CREATE: c_uint = 0x01;
pub const DRM_NOVA_GEM_INFO: c_uint = 0x02;
// Note: this is an enum so that it can be resolved by Rust bindgen.

