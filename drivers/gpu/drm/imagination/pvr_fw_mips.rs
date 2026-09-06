//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_fw_mips.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from pvr_gem.h.

//
// struct pvr_fw_mips_data - MIPS-specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_mips_data {
//
// @pt_pages: Pages containing MIPS pagetable.
//
    pub pt_pages: [*mut page; PVR_MIPS_PT_PAGE_COUNT],
// @pt: Pointer to CPU mapping of MIPS pagetable.
    pub pt: *mut u32,
// @pt_dma_addr: DMA mappings of MIPS pagetable.
    pub pt_dma_addr: [dma_addr_t; PVR_MIPS_PT_PAGE_COUNT],
// @boot_code_dma_addr: DMA address of MIPS boot code.
    pub boot_code_dma_addr: dma_addr_t,
// @boot_data_dma_addr: DMA address of MIPS boot data.
    pub boot_data_dma_addr: dma_addr_t,
// @exception_code_dma_addr: DMA address of MIPS exception code.
    pub exception_code_dma_addr: dma_addr_t,
// @cache_policy: Cache policy for this processor.
    pub cache_policy: u32,
// @pfn_mask: PFN mask for MIPS pagetable.
    pub pfn_mask: u32,
}
