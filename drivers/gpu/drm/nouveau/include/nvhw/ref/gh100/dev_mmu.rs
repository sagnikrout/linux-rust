//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/ref/gh100/dev_mmu.h
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
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

// Macro flag: #define __gh100_dev_mmu_h__

pub const NV_MMU_PTE_APERTURE_VIDEO_MEMORY: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_PTE_APERTURE_PEER_MEMORY: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_PTE_APERTURE_SYSTEM_COHERENT_MEMORY: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_PTE_APERTURE_SYSTEM_NON_COHERENT_MEMORY: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_PTE_KIND_INVALID: c_uint = 0x07 /* R---V */;
pub const NV_MMU_PTE_KIND_PITCH: c_uint = 0x00 /* R---V */;
pub const NV_MMU_PTE_KIND_GENERIC_MEMORY: c_uint = 0x6 /* R---V */;
pub const NV_MMU_PTE_KIND_Z16: c_uint = 0x1 /* R---V */;
pub const NV_MMU_PTE_KIND_S8: c_uint = 0x2 /* R---V */;
pub const NV_MMU_PTE_KIND_S8Z24: c_uint = 0x3 /* R---V */;
pub const NV_MMU_PTE_KIND_ZF32_X24S8: c_uint = 0x4 /* R---V */;
pub const NV_MMU_PTE_KIND_Z24S8: c_uint = 0x5 /* R---V */;
pub const NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE: c_uint = 0x8 /* R---V */;
pub const NV_MMU_PTE_KIND_GENERIC_MEMORY_COMPRESSIBLE_DISABLE_PLC: c_uint = 0x9 /* R---V */;
pub const NV_MMU_PTE_KIND_S8_COMPRESSIBLE_DISABLE_PLC: c_uint = 0xA /* R---V */;
pub const NV_MMU_PTE_KIND_Z16_COMPRESSIBLE_DISABLE_PLC: c_uint = 0xB /* R---V */;
pub const NV_MMU_PTE_KIND_S8Z24_COMPRESSIBLE_DISABLE_PLC: c_uint = 0xC /* R---V */;
pub const NV_MMU_PTE_KIND_ZF32_X24S8_COMPRESSIBLE_DISABLE_PLC: c_uint = 0xD /* R---V */;
pub const NV_MMU_PTE_KIND_Z24S8_COMPRESSIBLE_DISABLE_PLC: c_uint = 0xE /* R---V */;
pub const NV_MMU_PTE_KIND_SMSKED_MESSAGE: c_uint = 0xF /* R---V */;

pub const NV_MMU_VER3_PDE_IS_PTE_TRUE: c_uint = 0x1 /* RW--V */;
pub const NV_MMU_VER3_PDE_IS_PTE_FALSE: c_uint = 0x0 /* RW--V */;

pub const NV_MMU_VER3_PDE_VALID_TRUE: c_uint = 0x1 /* RW--V */;
pub const NV_MMU_VER3_PDE_VALID_FALSE: c_uint = 0x0 /* RW--V */;

pub const NV_MMU_VER3_PDE_APERTURE_INVALID: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PDE_APERTURE_VIDEO_MEMORY: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PDE_APERTURE_SYSTEM_COHERENT_MEMORY: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PDE_APERTURE_SYSTEM_NON_COHERENT_MEMORY: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_PDE_PCF_VALID_CACHED_ATS_ALLOWED__OR__INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_CACHED_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_UNCACHED_ATS_ALLOWED__OR__SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_UNCACHED_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_CACHED_ATS_NOT_ALLOWED__OR__INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_CACHED_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_UNCACHED_ATS_NOT_ALLOWED__OR__SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_VALID_UNCACHED_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_PDE_PCF_SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_PDE_ADDRESS_SHIFT: c_uint = 0x0000000c /*       */;
pub const NV_MMU_VER3_PDE__SIZE: c_int = 8;

pub const NV_MMU_VER3_DUAL_PDE_IS_PTE_TRUE: c_uint = 0x1 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_IS_PTE_FALSE: c_uint = 0x0 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_VALID_TRUE: c_uint = 0x1 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_VALID_FALSE: c_uint = 0x0 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_APERTURE_BIG_INVALID: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_BIG_VIDEO_MEMORY: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_BIG_SYSTEM_COHERENT_MEMORY: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_BIG_SYSTEM_NON_COHERENT_MEMORY: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_CACHED_ATS_ALLOWED__OR__INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_CACHED_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_UNCACHED_ATS_ALLOWED__OR__SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_UNCACHED_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_CACHED_ATS_NOT_ALLOWED__OR__INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_CACHED_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_UNCACHED_ATS_NOT_ALLOWED__OR__SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_VALID_UNCACHED_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_BIG_SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_APERTURE_SMALL_INVALID: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_SMALL_VIDEO_MEMORY: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_SMALL_SYSTEM_COHERENT_MEMORY: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_APERTURE_SMALL_SYSTEM_NON_COHERENT_MEMORY: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_CACHED_ATS_ALLOWED__OR__INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_CACHED_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_INVALID_ATS_ALLOWED: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_UNCACHED_ATS_ALLOWED__OR__SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_UNCACHED_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_SPARSE_ATS_ALLOWED: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_CACHED_ATS_NOT_ALLOWED__OR__INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_CACHED_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_INVALID_ATS_NOT_ALLOWED: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_UNCACHED_ATS_NOT_ALLOWED__OR__SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_VALID_UNCACHED_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_DUAL_PDE_PCF_SMALL_SPARSE_ATS_NOT_ALLOWED: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_DUAL_PDE_ADDRESS_SHIFT: c_uint = 0x0000000c /*       */;

pub const NV_MMU_VER3_DUAL_PDE__SIZE: c_int = 16;

pub const NV_MMU_VER3_PTE_VALID_TRUE: c_uint = 0x1 /* RW--V */;
pub const NV_MMU_VER3_PTE_VALID_FALSE: c_uint = 0x0 /* RW--V */;

pub const NV_MMU_VER3_PTE_APERTURE_VIDEO_MEMORY: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PTE_APERTURE_PEER_MEMORY: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PTE_APERTURE_SYSTEM_COHERENT_MEMORY: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PTE_APERTURE_SYSTEM_NON_COHERENT_MEMORY: c_uint = 0x00000003 /* RW--V */;

pub const NV_MMU_VER3_PTE_PCF_INVALID: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_SPARSE: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_MAPPING_NOWHERE: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_NO_VALID_4KB_PAGE: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_ATOMIC_CACHED_ACE: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_ATOMIC_UNCACHED_ACE: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_ATOMIC_CACHED_ACE: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_ATOMIC_UNCACHED_ACE: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_ATOMIC_CACHED_ACE: c_uint = 0x00000004 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_ATOMIC_UNCACHED_ACE: c_uint = 0x00000005 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_ATOMIC_CACHED_ACE: c_uint = 0x00000006 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_ATOMIC_UNCACHED_ACE: c_uint = 0x00000007 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_NO_ATOMIC_CACHED_ACE: c_uint = 0x00000008 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_NO_ATOMIC_UNCACHED_ACE: c_uint = 0x00000009 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_NO_ATOMIC_CACHED_ACE: c_uint = 0x0000000A /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_NO_ATOMIC_UNCACHED_ACE: c_uint = 0x0000000B /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_NO_ATOMIC_CACHED_ACE: c_uint = 0x0000000C /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_NO_ATOMIC_UNCACHED_ACE: c_uint = 0x0000000D /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_NO_ATOMIC_CACHED_ACE: c_uint = 0x0000000E /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_NO_ATOMIC_UNCACHED_ACE: c_uint = 0x0000000F /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_ATOMIC_CACHED_ACD: c_uint = 0x00000010 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_ATOMIC_UNCACHED_ACD: c_uint = 0x00000011 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_ATOMIC_CACHED_ACD: c_uint = 0x00000012 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_ATOMIC_UNCACHED_ACD: c_uint = 0x00000013 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_ATOMIC_CACHED_ACD: c_uint = 0x00000014 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_ATOMIC_UNCACHED_ACD: c_uint = 0x00000015 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_ATOMIC_CACHED_ACD: c_uint = 0x00000016 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_ATOMIC_UNCACHED_ACD: c_uint = 0x00000017 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_NO_ATOMIC_CACHED_ACD: c_uint = 0x00000018 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RW_NO_ATOMIC_UNCACHED_ACD: c_uint = 0x00000019 /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_NO_ATOMIC_CACHED_ACD: c_uint = 0x0000001A /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RW_NO_ATOMIC_UNCACHED_ACD: c_uint = 0x0000001B /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_NO_ATOMIC_CACHED_ACD: c_uint = 0x0000001C /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_REGULAR_RO_NO_ATOMIC_UNCACHED_ACD: c_uint = 0x0000001D /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_NO_ATOMIC_CACHED_ACD: c_uint = 0x0000001E /* RW--V */;
pub const NV_MMU_VER3_PTE_PCF_PRIVILEGE_RO_NO_ATOMIC_UNCACHED_ACD: c_uint = 0x0000001F /* RW--V */;

pub const NV_MMU_VER3_PTE_PEER_ID_0: c_uint = 0x00000000 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_1: c_uint = 0x00000001 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_2: c_uint = 0x00000002 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_3: c_uint = 0x00000003 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_4: c_uint = 0x00000004 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_5: c_uint = 0x00000005 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_6: c_uint = 0x00000006 /* RW--V */;
pub const NV_MMU_VER3_PTE_PEER_ID_7: c_uint = 0x00000007 /* RW--V */;
pub const NV_MMU_VER3_PTE_ADDRESS_SHIFT: c_uint = 0x0000000c /*       */;
pub const NV_MMU_VER3_PTE__SIZE: c_int = 8;
