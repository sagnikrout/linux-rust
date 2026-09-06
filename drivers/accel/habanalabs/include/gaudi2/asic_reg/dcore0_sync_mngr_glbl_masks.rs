//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_sync_mngr_glbl_masks.h
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
// DCORE0_SYNC_MNGR_GLBL
// (Prototype: SOB_GLBL)
//
// DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_SO_OVERFLOW_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_SO_OVERFLOW_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_MST_UNALIGN4B_SHIFT: c_int = 1;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_MST_UNALIGN4B_MASK: c_uint = 0x2;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_MST_RSP_ERR_SHIFT: c_int = 2;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_MASK_MST_RSP_ERR_MASK: c_uint = 0x4;
// DCORE0_SYNC_MNGR_GLBL_SM_SEI_CAUSE
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_CAUSE_CAUSE_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_CAUSE_CAUSE_MASK: c_uint = 0x7;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_CAUSE_LOG_SHIFT: c_int = 4;
pub const DCORE0_SYNC_MNGR_GLBL_SM_SEI_CAUSE_LOG_MASK: c_uint = 0xFFFF0;
// DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_L
pub const DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_L_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_L_VAL_MASK: c_uint = 0xFFF;
// DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_H
pub const DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_H_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_L2H_CPMR_H_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_L2H_MASK_L
pub const DCORE0_SYNC_MNGR_GLBL_L2H_MASK_L_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_L2H_MASK_L_VAL_MASK: c_uint = 0xFFF;
// DCORE0_SYNC_MNGR_GLBL_L2H_MASK_H
pub const DCORE0_SYNC_MNGR_GLBL_L2H_MASK_H_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_L2H_MASK_H_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_ASID_SEC
pub const DCORE0_SYNC_MNGR_GLBL_ASID_SEC_ASID_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_SEC_ASID_MASK: c_uint = 0xFFFF;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_SEC_BP_MMU_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_SEC_BP_MMU_MASK: c_uint = 0x10000;
// DCORE0_SYNC_MNGR_GLBL_ASID_PRIV_ONLY
pub const DCORE0_SYNC_MNGR_GLBL_ASID_PRIV_ONLY_ASID_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_PRIV_ONLY_ASID_MASK: c_uint = 0xFFFF;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_PRIV_ONLY_BP_MMU_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_PRIV_ONLY_BP_MMU_MASK: c_uint = 0x10000;
// DCORE0_SYNC_MNGR_GLBL_LBW_DELAY
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DELAY_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DELAY_VAL_MASK: c_uint = 0xFFFF;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DELAY_EN_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DELAY_EN_MASK: c_uint = 0x10000;
// DCORE0_SYNC_MNGR_GLBL_PI_SIZE
pub const DCORE0_SYNC_MNGR_GLBL_PI_SIZE_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_PI_SIZE_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_SOB_ONLY
pub const DCORE0_SYNC_MNGR_GLBL_SOB_ONLY_EN_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_SOB_ONLY_EN_MASK: c_uint = 0x1;
// DCORE0_SYNC_MNGR_GLBL_CQ_INTR
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_SEC_INTR_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_SEC_INTR_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_SEC_INTR_MASK_SHIFT: c_int = 8;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_SEC_INTR_MASK_MASK: c_uint = 0x100;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_INTR_QUEUE_INDEX_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INTR_CQ_INTR_QUEUE_INDEX_MASK: c_uint = 0x3F0000;
// DCORE0_SYNC_MNGR_GLBL_ASID_NONE_SEC_PRIV
pub const DCORE0_SYNC_MNGR_GLBL_ASID_NONE_SEC_PRIV_ASID_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_NONE_SEC_PRIV_ASID_MASK: c_uint = 0xFFFF;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_NONE_SEC_PRIV_BP_MMU_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_GLBL_ASID_NONE_SEC_PRIV_BP_MMU_MASK: c_uint = 0x10000;
// DCORE0_SYNC_MNGR_GLBL_PI_INC_MODE_SIZE
pub const DCORE0_SYNC_MNGR_GLBL_PI_INC_MODE_SIZE_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_PI_INC_MODE_SIZE_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_L
pub const DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_L_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_L_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_H
pub const DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_H_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_BASE_ADDR_H_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_SIZE_LOG2
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SIZE_LOG2_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SIZE_LOG2_VAL_MASK: c_uint = 0xFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_PI
pub const DCORE0_SYNC_MNGR_GLBL_CQ_PI_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_PI_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_SEC
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SEC_SEC_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SEC_SEC_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SEC_PRIV_SHIFT: c_int = 4;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_SEC_PRIV_MASK: c_uint = 0x10;
// DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_L
pub const DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_L_ADDRL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_L_ADDRL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_H
pub const DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_H_ADDRH_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_ADDR_H_ADDRH_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_LBW_DATA
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DATA_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_LBW_DATA_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_GLBL_CQ_INC_MODE
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INC_MODE_MODE_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_GLBL_CQ_INC_MODE_MODE_MASK: c_uint = 0x1;
