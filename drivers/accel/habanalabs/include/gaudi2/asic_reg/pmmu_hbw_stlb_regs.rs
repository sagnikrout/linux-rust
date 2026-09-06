//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pmmu_hbw_stlb_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PMMU_HBW_STLB
// (Prototype: STLB)
//
pub const mmPMMU_HBW_STLB_BUSY: c_uint = 0x4D01000;
pub const mmPMMU_HBW_STLB_ASID: c_uint = 0x4D01004;
pub const mmPMMU_HBW_STLB_HOP0_PA43_12: c_uint = 0x4D01008;
pub const mmPMMU_HBW_STLB_HOP0_PA63_44: c_uint = 0x4D0100C;
pub const mmPMMU_HBW_STLB_CACHE_INV: c_uint = 0x4D01010;
pub const mmPMMU_HBW_STLB_CACHE_INV_BASE_39_8: c_uint = 0x4D01014;
pub const mmPMMU_HBW_STLB_CACHE_INV_BASE_63_40: c_uint = 0x4D01018;
pub const mmPMMU_HBW_STLB_STLB_FEATURE_EN: c_uint = 0x4D0101C;
pub const mmPMMU_HBW_STLB_STLB_AXI_CACHE: c_uint = 0x4D01020;
pub const mmPMMU_HBW_STLB_HOP_CONFIGURATION: c_uint = 0x4D01024;
pub const mmPMMU_HBW_STLB_LINK_LIST_LOOKUP_MASK_63_32: c_uint = 0x4D01028;
pub const mmPMMU_HBW_STLB_LINK_LIST_LOOKUP_MASK_31_0: c_uint = 0x4D0102C;
pub const mmPMMU_HBW_STLB_INV_ALL_START: c_uint = 0x4D01034;
pub const mmPMMU_HBW_STLB_INV_ALL_SET: c_uint = 0x4D01038;
pub const mmPMMU_HBW_STLB_INV_PS: c_uint = 0x4D0103C;
pub const mmPMMU_HBW_STLB_INV_CONSUMER_INDEX: c_uint = 0x4D01040;
pub const mmPMMU_HBW_STLB_INV_HIT_COUNT: c_uint = 0x4D01044;
pub const mmPMMU_HBW_STLB_INV_SET: c_uint = 0x4D01048;
pub const mmPMMU_HBW_STLB_SRAM_INIT: c_uint = 0x4D0104C;
pub const mmPMMU_HBW_STLB_MEM_CACHE_INVALIDATION: c_uint = 0x4D01050;
pub const mmPMMU_HBW_STLB_MEM_CACHE_INV_STATUS: c_uint = 0x4D01054;
pub const mmPMMU_HBW_STLB_MEM_CACHE_BASE_38_7: c_uint = 0x4D01058;
pub const mmPMMU_HBW_STLB_MEM_CACHE_BASE_63_39: c_uint = 0x4D0105C;
pub const mmPMMU_HBW_STLB_MEM_CACHE_CONFIG: c_uint = 0x4D01060;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP5: c_uint = 0x4D01064;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP4: c_uint = 0x4D01068;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP3: c_uint = 0x4D0106C;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP2: c_uint = 0x4D01070;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP1: c_uint = 0x4D01074;
pub const mmPMMU_HBW_STLB_SET_THRESHOLD_HOP0: c_uint = 0x4D01078;
pub const mmPMMU_HBW_STLB_MULTI_HIT_INTERRUPT_CLR: c_uint = 0x4D0107C;
pub const mmPMMU_HBW_STLB_MULTI_HIT_INTERRUPT_MASK: c_uint = 0x4D01080;
pub const mmPMMU_HBW_STLB_MEM_L0_CACHE_CFG: c_uint = 0x4D01084;
pub const mmPMMU_HBW_STLB_MEM_READ_ARPROT: c_uint = 0x4D01088;
pub const mmPMMU_HBW_STLB_RANGE_CACHE_INVALIDATION: c_uint = 0x4D0108C;
pub const mmPMMU_HBW_STLB_RANGE_INV_START_LSB: c_uint = 0x4D01090;
pub const mmPMMU_HBW_STLB_RANGE_INV_START_MSB: c_uint = 0x4D01094;
pub const mmPMMU_HBW_STLB_RANGE_INV_END_LSB: c_uint = 0x4D01098;
pub const mmPMMU_HBW_STLB_RANGE_INV_END_MSB: c_uint = 0x4D0109C;
pub const mmPMMU_HBW_STLB_ASID_SCRAMBLER_CTRL: c_uint = 0x4D01100;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_0: c_uint = 0x4D01104;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_1: c_uint = 0x4D01108;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_2: c_uint = 0x4D0110C;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_3: c_uint = 0x4D01110;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_4: c_uint = 0x4D01114;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_5: c_uint = 0x4D01118;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_6: c_uint = 0x4D0111C;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_7: c_uint = 0x4D01120;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_8: c_uint = 0x4D01124;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MATRIX_H3_9: c_uint = 0x4D01128;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_10: c_uint = 0x4D0112C;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_11: c_uint = 0x4D01130;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_12: c_uint = 0x4D01134;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_13: c_uint = 0x4D01138;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_14: c_uint = 0x4D0113C;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_15: c_uint = 0x4D01140;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_16: c_uint = 0x4D01144;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_17: c_uint = 0x4D01148;
pub const mmPMMU_HBW_STLB_ASID_SCR_POLY_MAT_H3_18: c_uint = 0x4D0114C;
