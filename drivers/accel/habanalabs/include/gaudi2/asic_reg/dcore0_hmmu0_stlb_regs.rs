//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_hmmu0_stlb_regs.h
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
// DCORE0_HMMU0_STLB
// (Prototype: STLB)
//
pub const mmDCORE0_HMMU0_STLB_BUSY: c_uint = 0x4081000;
pub const mmDCORE0_HMMU0_STLB_ASID: c_uint = 0x4081004;
pub const mmDCORE0_HMMU0_STLB_HOP0_PA43_12: c_uint = 0x4081008;
pub const mmDCORE0_HMMU0_STLB_HOP0_PA63_44: c_uint = 0x408100C;
pub const mmDCORE0_HMMU0_STLB_CACHE_INV: c_uint = 0x4081010;
pub const mmDCORE0_HMMU0_STLB_CACHE_INV_BASE_39_8: c_uint = 0x4081014;
pub const mmDCORE0_HMMU0_STLB_CACHE_INV_BASE_63_40: c_uint = 0x4081018;
pub const mmDCORE0_HMMU0_STLB_STLB_FEATURE_EN: c_uint = 0x408101C;
pub const mmDCORE0_HMMU0_STLB_STLB_AXI_CACHE: c_uint = 0x4081020;
pub const mmDCORE0_HMMU0_STLB_HOP_CONFIGURATION: c_uint = 0x4081024;
pub const mmDCORE0_HMMU0_STLB_LINK_LIST_LOOKUP_MASK_63_32: c_uint = 0x4081028;
pub const mmDCORE0_HMMU0_STLB_LINK_LIST_LOOKUP_MASK_31_0: c_uint = 0x408102C;
pub const mmDCORE0_HMMU0_STLB_INV_ALL_START: c_uint = 0x4081034;
pub const mmDCORE0_HMMU0_STLB_INV_ALL_SET: c_uint = 0x4081038;
pub const mmDCORE0_HMMU0_STLB_INV_PS: c_uint = 0x408103C;
pub const mmDCORE0_HMMU0_STLB_INV_CONSUMER_INDEX: c_uint = 0x4081040;
pub const mmDCORE0_HMMU0_STLB_INV_HIT_COUNT: c_uint = 0x4081044;
pub const mmDCORE0_HMMU0_STLB_INV_SET: c_uint = 0x4081048;
pub const mmDCORE0_HMMU0_STLB_SRAM_INIT: c_uint = 0x408104C;
pub const mmDCORE0_HMMU0_STLB_MEM_CACHE_INVALIDATION: c_uint = 0x4081050;
pub const mmDCORE0_HMMU0_STLB_MEM_CACHE_INV_STATUS: c_uint = 0x4081054;
pub const mmDCORE0_HMMU0_STLB_MEM_CACHE_BASE_38_7: c_uint = 0x4081058;
pub const mmDCORE0_HMMU0_STLB_MEM_CACHE_BASE_63_39: c_uint = 0x408105C;
pub const mmDCORE0_HMMU0_STLB_MEM_CACHE_CONFIG: c_uint = 0x4081060;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP5: c_uint = 0x4081064;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP4: c_uint = 0x4081068;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP3: c_uint = 0x408106C;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP2: c_uint = 0x4081070;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP1: c_uint = 0x4081074;
pub const mmDCORE0_HMMU0_STLB_SET_THRESHOLD_HOP0: c_uint = 0x4081078;
pub const mmDCORE0_HMMU0_STLB_MULTI_HIT_INTERRUPT_CLR: c_uint = 0x408107C;
pub const mmDCORE0_HMMU0_STLB_MULTI_HIT_INTERRUPT_MASK: c_uint = 0x4081080;
pub const mmDCORE0_HMMU0_STLB_MEM_L0_CACHE_CFG: c_uint = 0x4081084;
pub const mmDCORE0_HMMU0_STLB_MEM_READ_ARPROT: c_uint = 0x4081088;
pub const mmDCORE0_HMMU0_STLB_RANGE_CACHE_INVALIDATION: c_uint = 0x408108C;
pub const mmDCORE0_HMMU0_STLB_RANGE_INV_START_LSB: c_uint = 0x4081090;
pub const mmDCORE0_HMMU0_STLB_RANGE_INV_START_MSB: c_uint = 0x4081094;
pub const mmDCORE0_HMMU0_STLB_RANGE_INV_END_LSB: c_uint = 0x4081098;
pub const mmDCORE0_HMMU0_STLB_RANGE_INV_END_MSB: c_uint = 0x408109C;
pub const mmDCORE0_HMMU0_STLB_ASID_SCRAMBLER_CTRL: c_uint = 0x4081100;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_0: c_uint = 0x4081104;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_1: c_uint = 0x4081108;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_2: c_uint = 0x408110C;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_3: c_uint = 0x4081110;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_4: c_uint = 0x4081114;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_5: c_uint = 0x4081118;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_6: c_uint = 0x408111C;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_7: c_uint = 0x4081120;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_8: c_uint = 0x4081124;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MATRIX_H3_9: c_uint = 0x4081128;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_10: c_uint = 0x408112C;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_11: c_uint = 0x4081130;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_12: c_uint = 0x4081134;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_13: c_uint = 0x4081138;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_14: c_uint = 0x408113C;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_15: c_uint = 0x4081140;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_16: c_uint = 0x4081144;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_17: c_uint = 0x4081148;
pub const mmDCORE0_HMMU0_STLB_ASID_SCR_POLY_MAT_H3_18: c_uint = 0x408114C;
