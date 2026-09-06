//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_mmu.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>
extern "C" {
    pub fn panfrost_mmu_map(mapping: *mut panfrost_gem_mapping) -> c_int;
}
extern "C" {
    pub fn panfrost_mmu_unmap(mapping: *mut panfrost_gem_mapping);
}
extern "C" {
    pub fn panfrost_mmu_init(pfdev: *mut panfrost_device) -> c_int;
}
extern "C" {
    pub fn panfrost_mmu_fini(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_mmu_reset(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_mmu_suspend_irq(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_mmu_as_get(pfdev: *mut panfrost_device, mmu: *mut panfrost_mmu) -> c_int;
}
extern "C" {
    pub fn panfrost_mmu_as_put(pfdev: *mut panfrost_device, mmu: *mut panfrost_mmu);
}
extern "C" {
    pub fn panfrost_mmu_ctx_put(mmu: *mut panfrost_mmu);
}
