//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_macro_regs.h
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
pub const mmDMA_MACRO_LBW_RANGE_HIT_BLOCK: c_uint = 0x4B0000;
pub const mmDMA_MACRO_LBW_RANGE_MASK_0: c_uint = 0x4B0004;
pub const mmDMA_MACRO_LBW_RANGE_MASK_1: c_uint = 0x4B0008;
pub const mmDMA_MACRO_LBW_RANGE_MASK_2: c_uint = 0x4B000C;
pub const mmDMA_MACRO_LBW_RANGE_MASK_3: c_uint = 0x4B0010;
pub const mmDMA_MACRO_LBW_RANGE_MASK_4: c_uint = 0x4B0014;
pub const mmDMA_MACRO_LBW_RANGE_MASK_5: c_uint = 0x4B0018;
pub const mmDMA_MACRO_LBW_RANGE_MASK_6: c_uint = 0x4B001C;
pub const mmDMA_MACRO_LBW_RANGE_MASK_7: c_uint = 0x4B0020;
pub const mmDMA_MACRO_LBW_RANGE_MASK_8: c_uint = 0x4B0024;
pub const mmDMA_MACRO_LBW_RANGE_MASK_9: c_uint = 0x4B0028;
pub const mmDMA_MACRO_LBW_RANGE_MASK_10: c_uint = 0x4B002C;
pub const mmDMA_MACRO_LBW_RANGE_MASK_11: c_uint = 0x4B0030;
pub const mmDMA_MACRO_LBW_RANGE_MASK_12: c_uint = 0x4B0034;
pub const mmDMA_MACRO_LBW_RANGE_MASK_13: c_uint = 0x4B0038;
pub const mmDMA_MACRO_LBW_RANGE_MASK_14: c_uint = 0x4B003C;
pub const mmDMA_MACRO_LBW_RANGE_MASK_15: c_uint = 0x4B0040;
pub const mmDMA_MACRO_LBW_RANGE_BASE_0: c_uint = 0x4B0044;
pub const mmDMA_MACRO_LBW_RANGE_BASE_1: c_uint = 0x4B0048;
pub const mmDMA_MACRO_LBW_RANGE_BASE_2: c_uint = 0x4B004C;
pub const mmDMA_MACRO_LBW_RANGE_BASE_3: c_uint = 0x4B0050;
pub const mmDMA_MACRO_LBW_RANGE_BASE_4: c_uint = 0x4B0054;
pub const mmDMA_MACRO_LBW_RANGE_BASE_5: c_uint = 0x4B0058;
pub const mmDMA_MACRO_LBW_RANGE_BASE_6: c_uint = 0x4B005C;
pub const mmDMA_MACRO_LBW_RANGE_BASE_7: c_uint = 0x4B0060;
pub const mmDMA_MACRO_LBW_RANGE_BASE_8: c_uint = 0x4B0064;
pub const mmDMA_MACRO_LBW_RANGE_BASE_9: c_uint = 0x4B0068;
pub const mmDMA_MACRO_LBW_RANGE_BASE_10: c_uint = 0x4B006C;
pub const mmDMA_MACRO_LBW_RANGE_BASE_11: c_uint = 0x4B0070;
pub const mmDMA_MACRO_LBW_RANGE_BASE_12: c_uint = 0x4B0074;
pub const mmDMA_MACRO_LBW_RANGE_BASE_13: c_uint = 0x4B0078;
pub const mmDMA_MACRO_LBW_RANGE_BASE_14: c_uint = 0x4B007C;
pub const mmDMA_MACRO_LBW_RANGE_BASE_15: c_uint = 0x4B0080;
pub const mmDMA_MACRO_HBW_RANGE_HIT_BLOCK: c_uint = 0x4B0084;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_0: c_uint = 0x4B00A8;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_1: c_uint = 0x4B00AC;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_2: c_uint = 0x4B00B0;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_3: c_uint = 0x4B00B4;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_4: c_uint = 0x4B00B8;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_5: c_uint = 0x4B00BC;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_6: c_uint = 0x4B00C0;
pub const mmDMA_MACRO_HBW_RANGE_MASK_49_32_7: c_uint = 0x4B00C4;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_0: c_uint = 0x4B00C8;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_1: c_uint = 0x4B00CC;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_2: c_uint = 0x4B00D0;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_3: c_uint = 0x4B00D4;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_4: c_uint = 0x4B00D8;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_5: c_uint = 0x4B00DC;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_6: c_uint = 0x4B00E0;
pub const mmDMA_MACRO_HBW_RANGE_MASK_31_0_7: c_uint = 0x4B00E4;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_0: c_uint = 0x4B00E8;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_1: c_uint = 0x4B00EC;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_2: c_uint = 0x4B00F0;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_3: c_uint = 0x4B00F4;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_4: c_uint = 0x4B00F8;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_5: c_uint = 0x4B00FC;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_6: c_uint = 0x4B0100;
pub const mmDMA_MACRO_HBW_RANGE_BASE_49_32_7: c_uint = 0x4B0104;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_0: c_uint = 0x4B0108;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_1: c_uint = 0x4B010C;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_2: c_uint = 0x4B0110;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_3: c_uint = 0x4B0114;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_4: c_uint = 0x4B0118;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_5: c_uint = 0x4B011C;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_6: c_uint = 0x4B0120;
pub const mmDMA_MACRO_HBW_RANGE_BASE_31_0_7: c_uint = 0x4B0124;
pub const mmDMA_MACRO_WRITE_EN: c_uint = 0x4B0128;
pub const mmDMA_MACRO_WRITE_CREDIT: c_uint = 0x4B012C;
pub const mmDMA_MACRO_READ_EN: c_uint = 0x4B0130;
pub const mmDMA_MACRO_READ_CREDIT: c_uint = 0x4B0134;
pub const mmDMA_MACRO_SRAM_BUSY: c_uint = 0x4B0138;
pub const mmDMA_MACRO_RAZWI_LBW_WT_VLD: c_uint = 0x4B013C;
pub const mmDMA_MACRO_RAZWI_LBW_WT_ID: c_uint = 0x4B0140;
pub const mmDMA_MACRO_RAZWI_LBW_RD_VLD: c_uint = 0x4B0144;
pub const mmDMA_MACRO_RAZWI_LBW_RD_ID: c_uint = 0x4B0148;
pub const mmDMA_MACRO_RAZWI_HBW_WT_VLD: c_uint = 0x4B014C;
pub const mmDMA_MACRO_RAZWI_HBW_WT_ID: c_uint = 0x4B0150;
pub const mmDMA_MACRO_RAZWI_HBW_RD_VLD: c_uint = 0x4B0154;
pub const mmDMA_MACRO_RAZWI_HBW_RD_ID: c_uint = 0x4B0158;
