//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/gaudi2/gaudi2_masks.h
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
// Copyright 2020-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

// Useful masks for bits in various registers

// QM_IDLE_MASK is valid for all engines QM idle check

pub const DCORE0_TPC0_QM_CGM_STS_AGENT_IDLE_MASK: c_uint = 0x100;
pub const DCORE0_TPC0_EML_CFG_DBG_CNT_DBG_EXIT_MASK: c_uint = 0x40;
// CGM_IDLE_MASK is valid for all engines CGM idle check

pub const QM_ARB_ERR_MSG_EN_CHOISE_OVF_MASK: c_uint = 0x1;
pub const QM_ARB_ERR_MSG_EN_CHOISE_WDT_MASK: c_uint = 0x2;
pub const QM_ARB_ERR_MSG_EN_AXI_LBW_ERR_MASK: c_uint = 0x4;

pub const PCIE_AUX_FLR_CTRL_HW_CTRL_MASK: c_uint = 0x1;
pub const PCIE_AUX_FLR_CTRL_INT_MASK_MASK: c_uint = 0x2;

pub const SM_CQ_L2H_MASK_VAL: c_uint = 0xFFFFFFFFFC000000ull;
pub const SM_CQ_L2H_CMPR_VAL: c_uint = 0x1000007FFC000000ull;

pub const SM_CQ_L2H_LOW_SHIFT: c_int = 20;

pub const AXUSER_HB_SEC_ASID_MASK: c_uint = 0x3FF;
pub const AXUSER_HB_SEC_MMBP_MASK: c_uint = 0x400;

pub const PCIE_DBI_MSIX_ADDRESS_MATCH_LOW_OFF_MSIX_ADDRESS_MATCH_EN_SHIFT: c_int = 0;
pub const PCIE_DBI_MSIX_ADDRESS_MATCH_LOW_OFF_MSIX_ADDRESS_MATCH_EN_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_SIGN_SHIFT: c_int = 15;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_SIGN_MASK: c_uint = 0x8000;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_ERR_INTR_SHIFT: c_int = 0;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_ERR_INTR_MASK: c_uint = 0x1;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_LBW_ERR_INTR_SHIFT: c_int = 1;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_LBW_ERR_INTR_MASK: c_uint = 0x2;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_BAD_ACCESS_INTR_SHIFT: c_int = 2;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_BAD_ACCESS_INTR_MASK: c_uint = 0x4;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_ERR_INTR_MASK_SHIFT: c_int = 3;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_ERR_INTR_MASK_MASK: c_uint = 0x8;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_LBW_ERR_INTR_MASK_SHIFT: c_int = 4;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_AXI_LBW_ERR_INTR_MASK_MASK: c_uint = 0x10;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_BAD_ACCESS_INTR_MASK_SHIFT: c_int = 5;
pub const PCIE_WRAP_PCIE_IC_SEI_INTR_IND_BAD_ACCESS_INTR_MASK_MASK: c_uint = 0x20;
