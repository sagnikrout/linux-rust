//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ethosu/ethosu_gem.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2025 Arm, Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_validated_cmdstream_info {
    pub cmd_size: u32,
    pub region_size: [u64; NPU_BASEP_REGION_MAX],
    pub output_region: [bool; NPU_BASEP_REGION_MAX],
}

//
// struct ethosu_gem_object - Driver specific GEM object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_gem_object {
// @base: Inherit from drm_gem_shmem_object.
    pub base: drm_gem_dma_object,
    pub info: *mut ethosu_validated_cmdstream_info,
// @flags: Combination of drm_ethosu_bo_flags flags.
    pub flags: u32,
}

extern "C" {
    pub fn container_of(_arg: to_drm_gem_dma_obj(obj), ethosu_gem_object: struct, _arg: base) -> return;
}
