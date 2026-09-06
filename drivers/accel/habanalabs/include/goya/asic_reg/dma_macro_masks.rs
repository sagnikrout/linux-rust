//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_macro_masks.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DMA_MACRO (Prototype: DMA_MACRO)
//
// DMA_MACRO_LBW_RANGE_HIT_BLOCK
pub const DMA_MACRO_LBW_RANGE_HIT_BLOCK_R_SHIFT: c_int = 0;
pub const DMA_MACRO_LBW_RANGE_HIT_BLOCK_R_MASK: c_uint = 0xFFFF;
// DMA_MACRO_LBW_RANGE_MASK
pub const DMA_MACRO_LBW_RANGE_MASK_R_SHIFT: c_int = 0;
pub const DMA_MACRO_LBW_RANGE_MASK_R_MASK: c_uint = 0x3FFFFFF;
// DMA_MACRO_LBW_RANGE_BASE
pub const DMA_MACRO_LBW_RANGE_BASE_R_SHIFT: c_int = 0;
pub const DMA_MACRO_LBW_RANGE_BASE_R_MASK: c_uint = 0x3FFFFFF;
// DMA_MACRO_HBW_RANGE_HIT_BLOCK
pub const DMA_MACRO_HBW_RANGE_HIT_BLOCK_R_SHIFT: c_int = 0;
pub const DMA_MACRO_HBW_RANGE_HIT_BLOCK_R_MASK: c_uint = 0xFF;
// DMA_MACRO_HBW_RANGE_MASK_49_32
pub const DMA_MACRO_HBW_RANGE_MASK_49_32_R_SHIFT: c_int = 0;
pub const DMA_MACRO_HBW_RANGE_MASK_49_32_R_MASK: c_uint = 0x3FFFF;
// DMA_MACRO_HBW_RANGE_MASK_31_0
pub const DMA_MACRO_HBW_RANGE_MASK_31_0_R_SHIFT: c_int = 0;
pub const DMA_MACRO_HBW_RANGE_MASK_31_0_R_MASK: c_uint = 0xFFFFFFFF;
// DMA_MACRO_HBW_RANGE_BASE_49_32
pub const DMA_MACRO_HBW_RANGE_BASE_49_32_R_SHIFT: c_int = 0;
pub const DMA_MACRO_HBW_RANGE_BASE_49_32_R_MASK: c_uint = 0x3FFFF;
// DMA_MACRO_HBW_RANGE_BASE_31_0
pub const DMA_MACRO_HBW_RANGE_BASE_31_0_R_SHIFT: c_int = 0;
pub const DMA_MACRO_HBW_RANGE_BASE_31_0_R_MASK: c_uint = 0xFFFFFFFF;
// DMA_MACRO_WRITE_EN
pub const DMA_MACRO_WRITE_EN_R_SHIFT: c_int = 0;
pub const DMA_MACRO_WRITE_EN_R_MASK: c_uint = 0x1;
// DMA_MACRO_WRITE_CREDIT
pub const DMA_MACRO_WRITE_CREDIT_R_SHIFT: c_int = 0;
pub const DMA_MACRO_WRITE_CREDIT_R_MASK: c_uint = 0x3FF;
// DMA_MACRO_READ_EN
pub const DMA_MACRO_READ_EN_R_SHIFT: c_int = 0;
pub const DMA_MACRO_READ_EN_R_MASK: c_uint = 0x1;
// DMA_MACRO_READ_CREDIT
pub const DMA_MACRO_READ_CREDIT_R_SHIFT: c_int = 0;
pub const DMA_MACRO_READ_CREDIT_R_MASK: c_uint = 0x3FF;
// DMA_MACRO_SRAM_BUSY
// DMA_MACRO_RAZWI_LBW_WT_VLD
pub const DMA_MACRO_RAZWI_LBW_WT_VLD_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_LBW_WT_VLD_R_MASK: c_uint = 0x1;
// DMA_MACRO_RAZWI_LBW_WT_ID
pub const DMA_MACRO_RAZWI_LBW_WT_ID_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_LBW_WT_ID_R_MASK: c_uint = 0x7FFF;
// DMA_MACRO_RAZWI_LBW_RD_VLD
pub const DMA_MACRO_RAZWI_LBW_RD_VLD_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_LBW_RD_VLD_R_MASK: c_uint = 0x1;
// DMA_MACRO_RAZWI_LBW_RD_ID
pub const DMA_MACRO_RAZWI_LBW_RD_ID_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_LBW_RD_ID_R_MASK: c_uint = 0x7FFF;
// DMA_MACRO_RAZWI_HBW_WT_VLD
pub const DMA_MACRO_RAZWI_HBW_WT_VLD_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_HBW_WT_VLD_R_MASK: c_uint = 0x1;
// DMA_MACRO_RAZWI_HBW_WT_ID
pub const DMA_MACRO_RAZWI_HBW_WT_ID_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_HBW_WT_ID_R_MASK: c_uint = 0x1FFFFFFF;
// DMA_MACRO_RAZWI_HBW_RD_VLD
pub const DMA_MACRO_RAZWI_HBW_RD_VLD_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_HBW_RD_VLD_R_MASK: c_uint = 0x1;
// DMA_MACRO_RAZWI_HBW_RD_ID
pub const DMA_MACRO_RAZWI_HBW_RD_ID_R_SHIFT: c_int = 0;
pub const DMA_MACRO_RAZWI_HBW_RD_ID_R_MASK: c_uint = 0x1FFFFFFF;
