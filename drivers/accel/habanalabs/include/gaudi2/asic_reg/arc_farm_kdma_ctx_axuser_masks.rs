//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/arc_farm_kdma_ctx_axuser_masks.h
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
// ARC_FARM_KDMA_CTX_AXUSER
// (Prototype: AXUSER)
//
// ARC_FARM_KDMA_CTX_AXUSER_HB_ASID
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_ASID_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_ASID_WR_MASK: c_uint = 0x3FF;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_ASID_RD_SHIFT: c_int = 16;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_ASID_RD_MASK: c_uint = 0x3FF0000;
// ARC_FARM_KDMA_CTX_AXUSER_HB_MMU_BP
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_MMU_BP_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_MMU_BP_WR_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_MMU_BP_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_MMU_BP_RD_MASK: c_uint = 0x10;
// ARC_FARM_KDMA_CTX_AXUSER_HB_STRONG_ORDER
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_STRONG_ORDER_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_STRONG_ORDER_WR_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_STRONG_ORDER_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_STRONG_ORDER_RD_MASK: c_uint = 0x10;
// ARC_FARM_KDMA_CTX_AXUSER_HB_NO_SNOOP
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_NO_SNOOP_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_NO_SNOOP_WR_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_NO_SNOOP_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_NO_SNOOP_RD_MASK: c_uint = 0x10;
// ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_IND_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_IND_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_DTYPE_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_DTYPE_MASK: c_uint = 0xF0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_OP_SHIFT: c_int = 8;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_OP_MASK: c_uint = 0x300;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_ROUND_SHIFT: c_int = 12;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_ROUND_MASK: c_uint = 0x3000;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_MAX_SHIFT: c_int = 16;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_REDUCTION_MAX_MASK: c_uint = 0x10000;
// ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_IND_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_IND_MASK: c_uint = 0x3;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_ADDITION_SIZE_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_ADDITION_SIZE_MASK: c_uint = 0xFF0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_MSB_MASK_SHIFT: c_int = 12;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_ATOMIC_MSB_MASK_MASK: c_uint = 0x1F000;
// ARC_FARM_KDMA_CTX_AXUSER_HB_QOS
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_QOS_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_QOS_WR_MASK: c_uint = 0xF;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_QOS_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_QOS_RD_MASK: c_uint = 0x70;
// ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_27_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_27_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_28_SHIFT: c_int = 1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_28_MASK: c_uint = 0x2;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_30_SHIFT: c_int = 2;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_30_MASK: c_uint = 0x4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_31_SHIFT: c_int = 3;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RSVD_WR_BIT_31_MASK: c_uint = 0x8;
// ARC_FARM_KDMA_CTX_AXUSER_HB_EMEM_CPAGE
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_EMEM_CPAGE_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_EMEM_CPAGE_WR_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_EMEM_CPAGE_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_EMEM_CPAGE_RD_MASK: c_uint = 0x10;
// ARC_FARM_KDMA_CTX_AXUSER_HB_CORE
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_CORE_WR_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_CORE_WR_MASK: c_uint = 0x1;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_CORE_RD_SHIFT: c_int = 4;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_CORE_RD_MASK: c_uint = 0x10;
// ARC_FARM_KDMA_CTX_AXUSER_E2E_COORD
pub const ARC_FARM_KDMA_CTX_AXUSER_E2E_COORD_X_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_E2E_COORD_X_MASK: c_uint = 0x1F;
pub const ARC_FARM_KDMA_CTX_AXUSER_E2E_COORD_Y_SHIFT: c_int = 8;
pub const ARC_FARM_KDMA_CTX_AXUSER_E2E_COORD_Y_MASK: c_uint = 0xF00;
// ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_LO
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_LO_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_HI
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_HI_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_WR_OVRD_HI_VAL_MASK: c_uint = 0x3FF;
// ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_LO
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_LO_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_LO_VAL_MASK: c_uint = 0xFFFFFFFF;
// ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_HI
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_HI_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_HB_RD_OVRD_HI_VAL_MASK: c_uint = 0x3FF;
// ARC_FARM_KDMA_CTX_AXUSER_LB_COORD
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_COORD_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_COORD_VAL_MASK: c_uint = 0x3FF;
// ARC_FARM_KDMA_CTX_AXUSER_LB_LOCK
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_LOCK_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_LOCK_VAL_MASK: c_uint = 0x1;
// ARC_FARM_KDMA_CTX_AXUSER_LB_RSVD
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_RSVD_BIT_21_11_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_RSVD_BIT_21_11_MASK: c_uint = 0x7FF;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_RSVD_BIT_22_SHIFT: c_int = 12;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_RSVD_BIT_22_MASK: c_uint = 0x1000;
// ARC_FARM_KDMA_CTX_AXUSER_LB_OVRD
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_OVRD_VAL_SHIFT: c_int = 0;
pub const ARC_FARM_KDMA_CTX_AXUSER_LB_OVRD_VAL_MASK: c_uint = 0xFFFFFFFF;
