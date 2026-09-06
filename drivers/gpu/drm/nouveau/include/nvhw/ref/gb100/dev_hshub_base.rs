//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/ref/gb100/dev_hshub_base.h
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

// Macro flag: #define __gb100_dev_hshub_base_h__
pub const NV_PFB_HSHUB0: c_uint = 0x00870fff:0x00870000;
pub const NV_PFB_HSHUB: c_uint = 0x00000FFF:0x00000000 /* RW--D */;
pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_LO: c_uint = 0x00000E50 /* RW-4R */;

pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_LO_ADR_INIT: c_uint = 0x00000000 /* RWI-V */;
pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_LO_ADR_MASK: c_uint = 0xFFFFFF00 /* ----V */;
pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_HI: c_uint = 0x00000E54 /* RW-4R */;

pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_HI_ADR_INIT: c_uint = 0x00000000 /* RWI-V */;
pub const NV_PFB_HSHUB_PCIE_FLUSH_SYSMEM_ADDR_HI_ADR_MASK: c_uint = 0x000FFFFF /* ----V */;
pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_LO: c_uint = 0x000006C0 /* RW-4R */;

pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_LO_ADR_INIT: c_uint = 0x00000000 /* RWI-V */;
pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_LO_ADR_MASK: c_uint = 0xFFFFFF00 /* ----V */;
pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_HI: c_uint = 0x000006C4 /* RW-4R */;

pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_HI_ADR_INIT: c_uint = 0x00000000 /* RWI-V */;
pub const NV_PFB_HSHUB_EG_PCIE_FLUSH_SYSMEM_ADDR_HI_ADR_MASK: c_uint = 0x000FFFFF /* ----V */;
