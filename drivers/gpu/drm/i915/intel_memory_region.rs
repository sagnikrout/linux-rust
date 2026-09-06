//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/intel_memory_region.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_memory_type {
    INTEL_MEMORY_SYSTEM = I915_MEMORY_CLASS_SYSTEM,
    INTEL_MEMORY_LOCAL = I915_MEMORY_CLASS_DEVICE,
    INTEL_MEMORY_STOLEN_SYSTEM,
    INTEL_MEMORY_STOLEN_LOCAL,
    INTEL_MEMORY_MOCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_region_id {
    INTEL_REGION_SMEM = 0,
    INTEL_REGION_LMEM_0,
    INTEL_REGION_LMEM_1,
    INTEL_REGION_LMEM_2,
    INTEL_REGION_LMEM_3,
    INTEL_REGION_STOLEN_SMEM,
    INTEL_REGION_STOLEN_LMEM,
    INTEL_REGION_UNKNOWN, /* Should be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_memory_region_ops {
    pub mem): *mut *mut int (init)(struct intel_memory_region,
    pub mem): *mut *mut int (release)(struct intel_memory_region,
    pub flags): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_memory_region {
    pub i915: *mut drm_i915_private,
    pub ops: *const intel_memory_region_ops,
    pub iomap: io_mapping,
    pub region: resource,
    pub io: resource,
    pub min_page_size: resource_size_t,
    pub total: resource_size_t,
    pub type: u16,
    pub instance: u16,
    pub id: intel_region_id,
    pub name: [c_char; 16],
    pub uabi_name: [c_char; 20],
    pub /: *mut *mut bool private; / not for userspace,
    pub /: *mut *mut mutex lock; / Protects access to objects,
    pub list: list_head,
    pub objects: },
    pub is_range_manager: bool,
    pub region_private: *mut c_void,
}

extern "C" {
    pub fn intel_memory_type_is_local(mem_type: intel_memory_type) -> bool;
}
extern "C" {
    pub fn intel_memory_region_destroy(mem: *mut intel_memory_region);
}
extern "C" {
    pub fn intel_memory_regions_hw_probe(i915: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn intel_memory_regions_driver_release(i915: *mut drm_i915_private);
}
