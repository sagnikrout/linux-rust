//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/d71/d71_dev.h
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
//
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d71_pipeline {
    pub base: komeda_pipeline,
// d71 private pipeline blocks
    pub lpu_addr: *mut u32 __iomem,
    pub cu_addr: *mut u32 __iomem,
    pub dou_addr: *mut u32 __iomem,
    pub /: *mut *mut *mut u32 __iomem dou_ft_coeff_addr; / forward transform coeffs table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d71_dev {
    pub mdev: *mut komeda_dev,
    pub num_blocks: c_int,
    pub num_pipelines: c_int,
    pub num_rich_layers: c_int,
    pub max_line_size: u32,
    pub max_vsize: u32,
    pub 1: u32 supports_dual_link :,
    pub 1: u32 integrates_tbu :,
// global register blocks
    pub gcu_addr: *mut u32 __iomem,
// scaling coeffs table
    pub glb_scl_coeff_addr: [*mut u32 __iomem; D71_MAX_GLB_SCL_COEFF],
    pub periph_addr: *mut u32 __iomem,
    pub pipes: [*mut d71_pipeline; D71_MAX_PIPELINE],
}

extern "C" {
    pub fn d71_read_block_header(reg: *mut u32 __iomem, blk: *mut block_header);
}
extern "C" {
    pub fn d71_dump(mdev: *mut komeda_dev, sf: *mut seq_file);
}
