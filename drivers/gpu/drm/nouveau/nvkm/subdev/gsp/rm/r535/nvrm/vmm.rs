//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/r535/nvrm/vmm.h
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.

// Excerpt of RM headers from https://github.com/NVIDIA/open-gpu-kernel-modules/tree/535.113.01

extern "C" {
    pub fn NV_ALIGN_BYTES(_arg: 8) -> NvU64   vaSize;
}
extern "C" {
    pub fn NV_ALIGN_BYTES(_arg: 8) -> NvU64   vaStartInternal;
}
extern "C" {
    pub fn NV_ALIGN_BYTES(_arg: 8) -> NvU64   vaLimitInternal;
}
extern "C" {
    pub fn NV_ALIGN_BYTES(_arg: 8) -> NvU64   vaBase;
}
pub const NV_VASPACE_ALLOCATION_INDEX_GPU_NEW: c_uint = 0x00 //<! Create new VASpace, by default;

pub const SPLIT_VAS_SERVER_RM_MANAGED_VA_START: c_uint = 0x100000000ULL  // 4GB;
pub const SPLIT_VAS_SERVER_RM_MANAGED_VA_SIZE: c_uint = 0x20000000ULL  // 512MB;

// !
// [in] GPU sub-device handle - this API only supports unicast.
// Pass 0 to use subDeviceId instead.
//
// !
// [in] GPU sub-device ID. Ignored if hSubDevice is non-zero.
//
// !
// [in] Page size (VA coverage) of the level to reserve.
// This need not be a leaf (page table) page size - it can be
// the coverage of an arbitrary level (including root page directory).
//
// !
// [in] First GPU virtual address of the range to reserve.
// This must be aligned to pageSize.
//
// !
// [in] Last GPU virtual address of the range to reserve.
// This (+1) must be aligned to pageSize.
//
// !
// [in] Number of PDE levels to copy.
//
// !
// [in] Per-level information.
//
// !
// Physical address of this page level instance.
//
// !
// Size in bytes allocated for this level instance.
//
// !
// Aperture in which this page level instance resides.
//
// !
// Page shift corresponding to the level
//

