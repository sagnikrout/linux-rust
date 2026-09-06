//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_hmmu0_mmu_regs.h
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
// DCORE0_HMMU0_MMU
// (Prototype: MMU)
//
pub const mmDCORE0_HMMU0_MMU_MMU_ENABLE: c_uint = 0x408000C;
pub const mmDCORE0_HMMU0_MMU_FORCE_ORDERING: c_uint = 0x4080010;
pub const mmDCORE0_HMMU0_MMU_FEATURE_ENABLE: c_uint = 0x4080014;
pub const mmDCORE0_HMMU0_MMU_VA_ORDERING_MASK_38_7: c_uint = 0x4080018;
pub const mmDCORE0_HMMU0_MMU_VA_ORDERING_MASK_64_39: c_uint = 0x408001C;
pub const mmDCORE0_HMMU0_MMU_LOG2_DDR_SIZE: c_uint = 0x4080020;
pub const mmDCORE0_HMMU0_MMU_SCRAMBLER: c_uint = 0x4080024;
pub const mmDCORE0_HMMU0_MMU_MEM_INIT_BUSY: c_uint = 0x4080028;
pub const mmDCORE0_HMMU0_MMU_SPI_SEI_MASK: c_uint = 0x408002C;
pub const mmDCORE0_HMMU0_MMU_SPI_SEI_CAUSE: c_uint = 0x4080030;
pub const mmDCORE0_HMMU0_MMU_PAGE_ERROR_CAPTURE: c_uint = 0x4080034;
pub const mmDCORE0_HMMU0_MMU_PAGE_ERROR_CAPTURE_VA: c_uint = 0x4080038;
pub const mmDCORE0_HMMU0_MMU_ACCESS_ERROR_CAPTURE: c_uint = 0x408003C;
pub const mmDCORE0_HMMU0_MMU_ACCESS_ERROR_CAPTURE_VA: c_uint = 0x4080040;
pub const mmDCORE0_HMMU0_MMU_ACCESS_PAGE_ERROR_VALID: c_uint = 0x4080044;
pub const mmDCORE0_HMMU0_MMU_INTERRUPT_CLR: c_uint = 0x4080048;
pub const mmDCORE0_HMMU0_MMU_INTERRUPT_MASK: c_uint = 0x408004C;
pub const mmDCORE0_HMMU0_MMU_DBG_MEM_WRAP_RM: c_uint = 0x4080050;
pub const mmDCORE0_HMMU0_MMU_SPI_CAUSE_CLR: c_uint = 0x4080054;
pub const mmDCORE0_HMMU0_MMU_PIPE_CREDIT: c_uint = 0x4080058;
pub const mmDCORE0_HMMU0_MMU_MMU_BYPASS: c_uint = 0x408006C;
pub const mmDCORE0_HMMU0_MMU_STATIC_MULTI_PAGE_SIZE: c_uint = 0x4080070;
pub const mmDCORE0_HMMU0_MMU_CORE_SEP_CACHE_RNG: c_uint = 0x40800A0;
pub const mmDCORE0_HMMU0_MMU_CORE_SEP_SLICE_CRDT: c_uint = 0x40800D0;
pub const mmDCORE0_HMMU0_MMU_TOTAL_SLICE_CREDIT: c_uint = 0x40800F4;
pub const mmDCORE0_HMMU0_MMU_PAGE_FAULT_ID_LSB: c_uint = 0x40800F8;
pub const mmDCORE0_HMMU0_MMU_PAGE_FAULT_ID_MSB: c_uint = 0x40800FC;
pub const mmDCORE0_HMMU0_MMU_PAGE_ACCESS_ID_LSB: c_uint = 0x4080100;
pub const mmDCORE0_HMMU0_MMU_PAGE_ACCESS_ID_MSB: c_uint = 0x4080104;
pub const mmDCORE0_HMMU0_MMU_DDR_RANGE_REG_ENABLE: c_uint = 0x4080108;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_0: c_uint = 0x4080110;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_1: c_uint = 0x4080114;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_2: c_uint = 0x4080118;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_3: c_uint = 0x408011C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_4: c_uint = 0x4080120;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_5: c_uint = 0x4080124;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_6: c_uint = 0x4080128;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_63_32_7: c_uint = 0x408012C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_0: c_uint = 0x4080140;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_1: c_uint = 0x4080144;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_2: c_uint = 0x4080148;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_3: c_uint = 0x408014C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_4: c_uint = 0x4080150;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_5: c_uint = 0x4080154;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_6: c_uint = 0x4080158;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MIN_31_0_7: c_uint = 0x408015C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_0: c_uint = 0x4080170;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_1: c_uint = 0x4080174;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_2: c_uint = 0x4080178;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_3: c_uint = 0x408017C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_4: c_uint = 0x4080180;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_5: c_uint = 0x4080184;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_6: c_uint = 0x4080188;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_63_32_7: c_uint = 0x408018C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_0: c_uint = 0x40801A0;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_1: c_uint = 0x40801A4;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_2: c_uint = 0x40801A8;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_3: c_uint = 0x40801AC;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_4: c_uint = 0x40801B0;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_5: c_uint = 0x40801B4;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_6: c_uint = 0x40801B8;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_SEC_MAX_31_0_7: c_uint = 0x40801BC;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_0: c_uint = 0x40801D0;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_1: c_uint = 0x40801D4;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_2: c_uint = 0x40801D8;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_3: c_uint = 0x40801DC;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_4: c_uint = 0x40801E0;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_5: c_uint = 0x40801E4;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_6: c_uint = 0x40801E8;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_63_32_7: c_uint = 0x40801EC;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_0: c_uint = 0x4080200;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_1: c_uint = 0x4080204;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_2: c_uint = 0x4080208;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_3: c_uint = 0x408020C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_4: c_uint = 0x4080210;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_5: c_uint = 0x4080214;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_6: c_uint = 0x4080218;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MIN_31_0_7: c_uint = 0x408021C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_0: c_uint = 0x4080230;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_1: c_uint = 0x4080234;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_2: c_uint = 0x4080238;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_3: c_uint = 0x408023C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_4: c_uint = 0x4080240;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_5: c_uint = 0x4080244;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_6: c_uint = 0x4080248;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_63_32_7: c_uint = 0x408024C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_0: c_uint = 0x4080260;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_1: c_uint = 0x4080264;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_2: c_uint = 0x4080268;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_3: c_uint = 0x408026C;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_4: c_uint = 0x4080270;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_5: c_uint = 0x4080274;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_6: c_uint = 0x4080278;
pub const mmDCORE0_HMMU0_MMU_MMU_RR_PRIV_MAX_31_0_7: c_uint = 0x408027C;
pub const mmDCORE0_HMMU0_MMU_ILLEGAL_ADDR_WRITE_63_32: c_uint = 0x4080290;
pub const mmDCORE0_HMMU0_MMU_ILLEGAL_ADDR_WRITE_31_0: c_uint = 0x4080294;
pub const mmDCORE0_HMMU0_MMU_ILLEGAL_ADDR_READ_63_32: c_uint = 0x4080298;
pub const mmDCORE0_HMMU0_MMU_ILLEGAL_ADDR_READ_31_0: c_uint = 0x408029C;
pub const mmDCORE0_HMMU0_MMU_RAZWI_WRITE_VLD: c_uint = 0x4080300;
pub const mmDCORE0_HMMU0_MMU_RAZWI_WRITE_ID_31_0: c_uint = 0x4080304;
pub const mmDCORE0_HMMU0_MMU_RAZWI_WRITE_ID_42_32: c_uint = 0x4080308;
pub const mmDCORE0_HMMU0_MMU_RAZWI_READ_VLD: c_uint = 0x408030C;
pub const mmDCORE0_HMMU0_MMU_RAZWI_READ_ID_31_0: c_uint = 0x4080310;
pub const mmDCORE0_HMMU0_MMU_RAZWI_READ_ID_42_32: c_uint = 0x4080314;
pub const mmDCORE0_HMMU0_MMU_MMU_SRC_NUM: c_uint = 0x408031C;
pub const mmDCORE0_HMMU0_MMU_RAZWI_ADDR_LSB: c_uint = 0x4080320;
pub const mmDCORE0_HMMU0_MMU_RAZWI_ADDR_MSB: c_uint = 0x4080324;
